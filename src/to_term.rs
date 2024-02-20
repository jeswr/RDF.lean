extern crate json;
extern crate oxrdf;

use json::JsonValue::{self, Null};
use oxrdf::{BlankNode, Literal, NamedNode, Subject, Term};

pub fn to_term(term_type: JsonValue) -> Option<Term> {
  if let Some(value) = term_type["NamedNode"].as_str() {
    return NamedNode::new(value).ok().map(|node| node.into());
  }
  if let Some(value) = term_type["BlankNode"].as_str() {
    return BlankNode::new(value).ok().map(|node| node.into());
  }
  if term_type["Literal"].is_array() {
    return match (term_type["Literal"][0].as_str(), to_term(term_type["Literal"][1].clone()), term_type["Literal"][2].as_str()) {
      (Some(value), Some(Term::NamedNode(datatype)), Some(language)) => {
        if datatype.as_str() == "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString" {
          return Literal::new_language_tagged_literal(value, language).ok().map(|node| node.into())
        }
        None
      },
      (Some(value), Some(Term::NamedNode(datatype)), _) => Some(Literal::new_typed_literal(value, datatype).into()),
      _ => None,
    };
  }
  None
}

pub fn to_triple(triple: JsonValue) -> Option<oxrdf::Triple> {
  match (to_term(triple["subject"].clone()), to_term(triple["predicate"].clone()), to_term(triple["object"].clone())) {
    (Some(Term::NamedNode(subject)), Some(Term::NamedNode(predicate)), Some(object)) => Some(oxrdf::Triple::new(subject, predicate, object)),
    (Some(Term::BlankNode(subject)), Some(Term::NamedNode(predicate)), Some(object)) => Some(oxrdf::Triple::new(subject, predicate, object)),
    _ => None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use oxrdf::{BlankNode, Literal, NamedNode, Term, Triple};

  #[test]
  fn exploration() {
    assert_eq!(to_term(json::object!{"NamedNode": "http://example.org/"}), NamedNode::new("http://example.org/").ok().map(|node| Term::NamedNode(node)));
    assert_eq!(to_term(json::object!{}), None);
    assert_eq!(to_term(json::object!{"BlankNode": "abc123"}), BlankNode::new("abc123").ok().map(|node| Term::BlankNode(node)));
    assert_eq!(to_term(json::object!{"Literal": ["Hello World!", {"NamedNode": "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString" }, "en"]}), Literal::new_language_tagged_literal("Hello World!", "en").ok().map(|node| Term::Literal(node)));
    assert_eq!(to_term(json::object!{"Literal": ["Hello World!", {"NamedNode": "http://www.w3.org/2001/XMLSchema#string" }]}), Some(Term::Literal(Literal::new_simple_literal("Hello World!"))));
    assert_eq!(to_term(json::object!{"Literal": ["3", {"NamedNode": "http://www.w3.org/2001/XMLSchema#integer" }]}), Some(Term::Literal(3.into())));
    assert_eq!(to_term(json::object!{"Literal": ["true", {"NamedNode": "http://www.w3.org/2001/XMLSchema#boolean" }]}), Some(Term::Literal(true.into())));

    assert_eq!(to_term(json::object!{"Literal": ["3", {"NamedNode": "http://www.w3.org/2001/XMLSchema#integer" }, "en"]}), None);
    assert_eq!(to_term(json::object!{"Literal": "3"}), None);
    assert_eq!(to_triple(json::object!{
      "subject": {"NamedNode": "http://example.org/"},
      "predicate": {"NamedNode": "http://example.org/"},
      "object": {"NamedNode": "http://example.org/"},
    }), Some(Triple::new(NamedNode::new("http://example.org/").unwrap(), NamedNode::new("http://example.org/").unwrap(), NamedNode::new("http://example.org/").unwrap())));
    assert_eq!(to_triple(json::object!{
      "subject": {"NamedNode": "http://example.org/"},
      "predicate": {"NamedNode": "http://example.org/"},
      "object": {"Literal": ["true", {"NamedNode": "http://www.w3.org/2001/XMLSchema#boolean" }]},
    }), Some(Triple::new(NamedNode::new("http://example.org/").unwrap(), NamedNode::new("http://example.org/").unwrap(), Term::Literal(true.into()))));

    assert_eq!(to_triple(json::object!{
      "subject": {"Literal": ["true", {"NamedNode": "http://www.w3.org/2001/XMLSchema#boolean" }]},
      "predicate": {"NamedNode": "http://example.org/"},
      "object": {"NamedNode": "http://example.org/"},
    }), None);
  }
}
