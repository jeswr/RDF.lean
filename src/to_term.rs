extern crate json;
extern crate oxrdf;

use json::JsonValue;
use oxrdf::{BlankNode, Literal, NamedNode, Subject, Term};

pub fn to_term(term_type: JsonValue) -> Option<Term> {
  if term_type.has_key("NamedNode") {
    return term_type["NamedNode"].as_str().and_then(|value| NamedNode::new(value).ok()).map(|node| node.into());
  }
  if term_type.has_key("BlankNode") {
    return term_type["BlankNode"].as_str().and_then(|value| BlankNode::new(value).ok()).map(|node| node.into());
  }
  if term_type.has_key("Literal") {
    if !term_type["Literal"].is_array() {
      return None;
    }
    return term_type["Literal"][0].as_str().and_then(|value| {
      to_term(term_type["Literal"][1].clone()).and_then(|datatype| {
        match datatype {
          Term::NamedNode(iri) => {
            // FIXME: Error in the case where we have a language tag, but the datatype is not rdf:langString
            return term_type["Literal"][2].as_str().map(|language| {
              Term::Literal(Literal::new_language_tagged_literal(value, language).unwrap()).into()
            }).unwrap_or_else(|| {
              Term::Literal(Literal::new_typed_literal(value, iri)).into()
            });
          },
          _ => None::<Term>,
        };
        None
      })
    })
  }
  return None
}

fn to_subject(subject: JsonValue) -> Option<Subject> {
  to_term(subject).and_then(|term| {
    match term {
      Term::NamedNode(namedNode) => Some(Subject::NamedNode(namedNode)),
      Term::BlankNode(namedNode) => Some(Subject::BlankNode(namedNode)),
      _ => None,
    }
  })
}

fn to_named_node(named_node: JsonValue) -> Option<NamedNode> {
  to_term(named_node).and_then(|term| {
    match term {
      Term::NamedNode(namedNode) => Some(namedNode),
      _ => None,
    }
  })
}

pub fn to_triple(triple: JsonValue) -> Option<oxrdf::Triple> {
  match (to_subject(triple["subject"].clone()), to_named_node(triple["predicate"].clone()), to_term(triple["object"].clone())) {
    (Some(subject), Some(predicate), Some(object)) => Some(oxrdf::Triple::new(subject, predicate, object)),
    _ => None,
  }
}

#[cfg(test)]
mod tests {
  use super::to_term;
  use oxrdf::{BlankNode, Literal, NamedNode, Term};

  #[test]
  fn exploration() {
    assert_eq!(to_term(json::object!{"NamedNode": "http://example.org/"}), NamedNode::new("http://example.org/").ok().map(|node| Term::NamedNode(node)));
    assert_eq!(to_term(json::object!{"BlankNode": "abc123"}), BlankNode::new("abc123").ok().map(|node| Term::BlankNode(node)));
    assert_eq!(to_term(json::object!{"Literal": ["Hello World!", {"NamedNode": "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString" }, "en"]}), Literal::new_language_tagged_literal("Hello World!", "en").ok().map(|node| Term::Literal(node)));
    assert_eq!(to_term(json::object!{"Literal": ["Hello World!", {"NamedNode": "http://www.w3.org/2001/XMLSchema#string" }]}), Some(Term::Literal(Literal::new_simple_literal("Hello World!"))));
    assert_eq!(to_term(json::object!{"Literal": ["3", {"NamedNode": "http://www.w3.org/2001/XMLSchema#integer" }]}), Some(Term::Literal(3.into())));
    assert_eq!(to_term(json::object!{"Literal": ["true", {"NamedNode": "http://www.w3.org/2001/XMLSchema#boolean" }]}), Some(Term::Literal(true.into())));

    assert_eq!(to_term(json::object!{"Literal": ["3", {"NamedNode": "http://www.w3.org/2001/XMLSchema#integer" }, "en"]}), None);
  }
}
