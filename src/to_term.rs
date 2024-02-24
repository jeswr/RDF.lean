extern crate json;
extern crate oxrdf;

use json::JsonValue::{self, Null};
use oxrdf::{BlankNode, Literal, NamedNode, Subject, Term};

pub fn to_term(term_type: JsonValue) -> Option<Term> {
  if let Some(value) = term_type["NamedNode"]["iri"].as_str() {
    return NamedNode::new(value).ok().map(|node| node.into());
  }
  if let Some(value) = term_type["BlankNode"]["id"].as_str() {
    return BlankNode::new(value).ok().map(|node| node.into());
  }
  if let (Some(value), Some(language)) = (term_type["Literal"]["LanguageTaggedString"]["value"].as_str(), term_type["Literal"]["LanguageTaggedString"]["language"].as_str()) {
    return Literal::new_language_tagged_literal(value, language).ok().map(|node| node.into());
  }
  if let (Some(value), Some(Term::NamedNode(named_node))) = (term_type["Literal"]["Typed"]["value"].as_str(), to_term(term_type["Literal"]["Typed"]["datatype"].clone())) {
    return Some(Term::Literal(Literal::new_typed_literal(value, named_node)));
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
