extern crate json;
extern crate oxrdf;

use json::JsonValue;
use oxrdf::{NamedNode, Term, Triple};

pub fn from_term(object: Term) -> JsonValue {
    match object {
        Term::NamedNode(iri) => json::object!{"NamedNode": { "iri": iri.into_string() }},
        Term::BlankNode(id) => json::object!{"BlankNode": { "id": id.into_string() }},
        Term::Literal(literal) => json::object!{"Literal": match literal.destruct() {
            (value, _, Some(language)) => json::object!{ "LanguageTaggedString": { "value": value, "language": language } },
            (value, datatype, _) => json::object!{ "Typed": {
                "value": value,
                "datatype": { "iri": if let Some(named_node) = datatype { named_node.into_string() } else { String::from("http://www.w3.org/2001/XMLSchema#string") } },
            }},
        }},
    }
}

pub fn from_triple(triple: Triple) -> JsonValue {
    json::object! {
        "subject": from_term(triple.subject.into()),
        "predicate": from_term(triple.predicate.into()),
        "object": from_term(triple.object.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use oxrdf::{BlankNode, Literal, NamedNode, Triple};

    #[test]
    fn exploration() {
        let hello_world_literal = json::object!{"Literal": {"LanguageTaggedString": {
            "value": "Hello World!",
            "language": "en",
        }}};
        let literal = json::object!{"Literal": {"Typed": json::object!{"value": String::from("Hello World!"), "dataype": {"NamedNode": {"iri": "http://www.w3.org/2001/XMLSchema#string"}}}}};
        let literal_true = json::object!{"Literal": {"Typed": {"value": "true", "dataype": {"NamedNode": {"iri": "http://www.w3.org/2001/XMLSchema#boolean"}}}}};
        assert_eq!(from_term(NamedNode::new("http://example.org/").unwrap().into()), json::object!{"NamedNode": {"iri": "http://example.org/"}});
        assert_eq!(from_term(BlankNode::new("abc123").unwrap().into()), json::object!{"BlankNode": {"id": "abc123"}});
        assert_eq!(from_term(Literal::new_language_tagged_literal("Hello World!", "en").unwrap().into()), hello_world_literal);
        assert_eq!(from_term(Literal::new_simple_literal("Hello World!").into()), literal);
        assert_eq!(from_triple(Triple::new(NamedNode::new("http://example.org/").unwrap(), NamedNode::new("http://example.org/").unwrap(), NamedNode::new("http://example.org/").unwrap())), json::object!{
            "subject": {"NamedNode": {"iri": "http://example.org/"}},
            "predicate": {"NamedNode": {"iri": "http://example.org/"}},
            "object": {"NamedNode": {"iri": "http://example.org/"}},
        });
        assert_eq!(from_triple(Triple::new(NamedNode::new("http://example.org/").unwrap(), NamedNode::new("http://example.org/").unwrap(), Term::Literal(true.into()))), json::object!{
            "subject": {"NamedNode": {"iri": "http://example.org/"}},
            "predicate": {"NamedNode": {"iri": "http://example.org/"}},
            "object": literal_true,
        });
    }
}
