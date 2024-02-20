extern crate json;
extern crate oxrdf;

use json::JsonValue;
use oxrdf::{Term, Triple};

pub fn from_term(object: Term) -> JsonValue {
    match object {
        Term::NamedNode(iri) => json::object!{"NamedNode": iri.into_string()},
        Term::BlankNode(id) => json::object!{"BlankNode": id.into_string()},
        Term::Literal(literal) => {
            let value = literal.value();
            let datatype = from_term(literal.datatype().into());
            if let Some(language) = literal.language() {
                return json::object! {"Literal": [value, datatype, language]};
            }
            json::object! {"Literal": [value, datatype]}
        }
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
        assert_eq!(from_term(NamedNode::new("http://example.org/").unwrap().into()), json::object!{"NamedNode": "http://example.org/"});
        assert_eq!(from_term(BlankNode::new("abc123").unwrap().into()), json::object!{"BlankNode": "abc123"});
        assert_eq!(from_term(Literal::new_language_tagged_literal("Hello World!", "en").unwrap().into()), json::object!{"Literal": ["Hello World!", {"NamedNode": "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString" }, "en"]});
        assert_eq!(from_term(Literal::new_simple_literal("Hello World!").into()), json::object!{"Literal": ["Hello World!", {"NamedNode": "http://www.w3.org/2001/XMLSchema#string" }]});

        assert_eq!(from_triple(Triple::new(NamedNode::new("http://example.org/").unwrap(), NamedNode::new("http://example.org/").unwrap(), NamedNode::new("http://example.org/").unwrap())), json::object!{
            "subject": {"NamedNode": "http://example.org/"},
            "predicate": {"NamedNode": "http://example.org/"},
            "object": {"NamedNode": "http://example.org/"},
        });
        assert_eq!(from_triple(Triple::new(NamedNode::new("http://example.org/").unwrap(), NamedNode::new("http://example.org/").unwrap(), Term::Literal(true.into()))), json::object!{
            "subject": {"NamedNode": "http://example.org/"},
            "predicate": {"NamedNode": "http://example.org/"},
            "object": {"Literal": ["true", {"NamedNode": "http://www.w3.org/2001/XMLSchema#boolean" }]},
        });
      
    }
}
