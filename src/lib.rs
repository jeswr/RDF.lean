// extern crate lean_sys;
extern crate json;
extern crate oxrdf;

use json::{object::Object, JsonValue};
use std::{convert::TryInto, error, slice};
// use lean_sys::{lean_array_push, lean_mk_empty_array, lean_obj_res, lean_mk_string_from_bytes, lean_string_cstr, lean_string_len, lean_array_size, lean_array_uget, lean_initialize};
extern crate oxrdfio;
use oxrdf::{BlankNode, Literal, NamedNode, Quad, Subject, Term, Triple};
use oxrdfio::{ParseError, RdfFormat, RdfParser, RdfSerializer}; // RdfSerializer
use std::str;

mod from_term;
mod to_term;
mod serialize;
mod parse;
mod utils;

#[cfg(test)]
mod tests {
    use oxrdf::NamedNode;

    use crate::from_term::from_triple;
    use crate::to_term::to_triple;

    #[test]
    fn exploration() {
        let triple = oxrdf::Triple::new(
            NamedNode::new("http://example.org/").unwrap(),
            NamedNode::new("http://example.org/").unwrap(),
            NamedNode::new("http://example.org/").unwrap(),
        );
        assert_eq!(to_triple(from_triple(triple.clone())), Some(triple));
    }
}
