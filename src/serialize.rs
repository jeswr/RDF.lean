extern crate json;
extern crate lean_sys;
extern crate oxrdf;

use self::lean_sys::{
    lean_array_push, lean_array_size, lean_array_uget, lean_initialize, lean_mk_empty_array,
    lean_mk_string_from_bytes, lean_obj_res, lean_string_cstr, lean_string_len,
};
use json::{object::Object, JsonValue};
use std::{convert::TryInto, error, slice};
extern crate oxrdfio;
use oxrdf::{BlankNode, Literal, NamedNode, Quad, Subject, Term, Triple};
use oxrdfio::{ParseError, RdfFormat, RdfParser, RdfSerializer}; // RdfSerializer

use crate::{
    from_term::{from_term, from_triple},
    to_term::to_triple,
    utils::{
        lean_mk_string_from_str, lean_mk_string_from_string, lean_string_str, lean_string_utf8,
    },
};

pub fn i_serialize(quads: lean_obj_res, fmt: lean_obj_res) -> Option<lean_obj_res> {
    let mut serializer = RdfSerializer::from_format(RdfFormat::from_media_type(lean_string_str(fmt).ok()?)?).serialize_to_write(Vec::new());

    let size = unsafe { lean_array_size(quads) };
    for i in 0..size {
        let quad = lean_string_str(unsafe { lean_array_uget(quads, i) }).ok()?;
        let triple: Triple = to_triple(json::parse(quad).ok()?)?;
        serializer.write_quad(&Quad {
            subject: triple.subject,
            predicate: triple.predicate,
            object: triple.object,
            graph_name: oxrdf::GraphName::DefaultGraph.into(),
        }).ok()?;
    }
    let bytes = serializer.finish().ok()?;

    Some(lean_mk_string_from_str(&String::from_utf8(bytes).ok()?))
}

#[no_mangle]
pub fn serialize(quads: lean_obj_res, fmt: lean_obj_res) -> lean_obj_res {
    i_serialize(quads, fmt).unwrap()
}
