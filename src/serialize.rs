extern crate json;
extern crate lean_sys;
extern crate oxrdf;

use self::lean_sys::{
    lean_array_size, lean_array_uget, lean_obj_res,
};

extern crate oxrdfio;
use oxrdf::{Quad, Triple};
use oxrdfio::{RdfFormat, RdfSerializer}; // RdfSerializer

use crate::{
    to_term::to_triple,
    utils::{
        lean_mk_string_from_str, lean_string_str,
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
