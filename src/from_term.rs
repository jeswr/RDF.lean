extern crate json;
extern crate lean_sys;
extern crate oxrdf;

use self::json::{object::Object, JsonValue};
use std::{convert::TryInto, error, slice};
use self::oxrdf::{BlankNode, Literal, NamedNode, Quad, Subject, Term, Triple};

pub fn from_term(object: Term) -> JsonValue {
    match object {
        Term::NamedNode(iri) => {
            let str = iri.into_string();
            return json::object! {"NamedNode": &str[1..str.len()-1]};
        }
        Term::BlankNode(id) => {
            let str = id.into_string();
            return json::object! {"BlankNode": &str[2..str.len()]};
        }
        Term::Literal(literal) => {
            let value = literal.value();
            let datatype = from_term(literal.datatype().into());
            if let Some(language) = literal.language() {
                return json::object! {"Literal": [&value[1..value.len()-1], datatype, language]};
            } else {
                return json::object! {"Literal": [&value[1..value.len()-1], datatype]};
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
