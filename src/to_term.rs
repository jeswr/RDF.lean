extern crate json;
extern crate oxrdf;

use json::JsonValue::{self};
use oxrdf::{BlankNode, Literal, NamedNode, Term};

pub fn to_term(term_type: JsonValue) -> Option<Term> {
    if let Some(value) = term_type["NamedNode"]["iri"].as_str() {
        return NamedNode::new(value).ok().map(|node| node.into());
    }
    if let Some(value) = term_type["BlankNode"]["id"].as_str() {
        return BlankNode::new(value).ok().map(|node| node.into());
    }
    if let (Some(value), Some(language)) = (
        term_type["Literal"]["LanguageTaggedString"]["value"].as_str(),
        term_type["Literal"]["LanguageTaggedString"]["language"].as_str(),
    ) {
        return Literal::new_language_tagged_literal(value, language)
            .ok()
            .map(|node| node.into());
    }
    if let (Some(value), Some(named_node)) = (
        term_type["Literal"]["Typed"]["value"].as_str(),
        term_type["Literal"]["Typed"]["datatype"]["iri"].as_str(),
    ) {
        return Some(Term::Literal(Literal::new_typed_literal(value, NamedNode::new(named_node).ok()?)));
    }
    None
}

pub fn to_triple(triple: JsonValue) -> Option<oxrdf::Triple> {
    match (
        to_term(triple["subject"].clone()),
        to_term(triple["predicate"].clone()),
        to_term(triple["object"].clone()),
    ) {
        (Some(Term::NamedNode(subject)), Some(Term::NamedNode(predicate)), Some(object)) => {
            Some(oxrdf::Triple::new(subject, predicate, object))
        }
        (Some(Term::BlankNode(subject)), Some(Term::NamedNode(predicate)), Some(object)) => {
            Some(oxrdf::Triple::new(subject, predicate, object))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxrdf::{BlankNode, Literal, NamedNode, Term, Triple};

    #[test]
    fn exploration() {
        assert_eq!(
            to_term(json::object! {"NamedNode": {"iri": "http://example.org/" }}),
            NamedNode::new("http://example.org/")
                .ok()
                .map(|node| Term::NamedNode(node))
        );
        assert_eq!(to_term(json::object! {}), None);
        assert_eq!(
            to_term(json::object! {"BlankNode": {"id": "abc123"}}),
            BlankNode::new("abc123")
                .ok()
                .map(|node| Term::BlankNode(node))
        );
        assert_eq!(
            to_term(
                json::object! {"Literal": {"LanguageTaggedString": {"value": "Hello World!", "language": "en"}}}
            ),
            Literal::new_language_tagged_literal("Hello World!", "en")
                .ok()
                .map(|node| Term::Literal(node))
        );
        assert_eq!(
            to_term(
                json::object! {"Literal": {"Typed": {"value": "Hello World!", "datatype": {"iri": "http://www.w3.org/2001/XMLSchema#string"}}}}
            ),
            Some(Term::Literal(Literal::new_typed_literal("Hello World!", NamedNode::new("http://www.w3.org/2001/XMLSchema#string").unwrap())))
        );
        assert_eq!(
            to_term(
                json::object! {"Literal": {"Typed": {"value": "3", "datatype": {"iri": "http://www.w3.org/2001/XMLSchema#integer"}}}}
            ),
            Some(Term::Literal(Literal::new_typed_literal("3", NamedNode::new("http://www.w3.org/2001/XMLSchema#integer").unwrap())))
        );
        assert_eq!(
            to_term(
                json::object! {"Literal": {"Typed": {"value": "true", "datatype": {"iri": "http://www.w3.org/2001/XMLSchema#boolean"}}}}
            ),
            Some(Term::Literal(Literal::new_typed_literal("true", NamedNode::new("http://www.w3.org/2001/XMLSchema#boolean").unwrap())))
        );

        assert_eq!(
            to_term(
                json::object! {"Literal": ["3", {"NamedNode": "http://www.w3.org/2001/XMLSchema#integer" }, "en"]}
            ),
            None
        );
        assert_eq!(to_term(json::object! {"Literal": "3"}), None);
        assert_eq!(
            to_triple(json::object! {
              "subject": {"NamedNode": {"iri": "http://example.org/"}},
              "predicate": {"NamedNode": {"iri": "http://example.org/"}},
              "object": {"NamedNode": {"iri": "http://example.org/"}},
            }),
            Some(Triple::new(
                NamedNode::new("http://example.org/").unwrap(),
                NamedNode::new("http://example.org/").unwrap(),
                NamedNode::new("http://example.org/").unwrap()
            ))
        );
        assert_eq!(
            to_triple(json::object! {
              "subject": {"NamedNode": {"iri": "http://example.org/"}},
              "predicate": {"NamedNode": {"iri": "http://example.org/"}},
              "object": {"Literal": {"Typed": {"value": "true", "datatype": {"iri": "http://www.w3.org/2001/XMLSchema#boolean"}}}},
            }),
            Some(Triple::new(
                NamedNode::new("http://example.org/").unwrap(),
                NamedNode::new("http://example.org/").unwrap(),
                Term::Literal(Literal::new_typed_literal("true", NamedNode::new("http://www.w3.org/2001/XMLSchema#boolean").unwrap()))
            ))
        );

        assert_eq!(
            to_triple(json::object! {
              "subject": {"Literal": {"Typed": {"value": "true", "datatype": {"iri": "http://www.w3.org/2001/XMLSchema#boolean"}}}},
              "predicate": {"NamedNode": {"iri": "http://example.org/"}},
              "object": {"NamedNode": {"iri": "http://example.org/"}},
            }),
            None
        );
    }
}
