extern crate lean_sys;
extern crate oxrdf;
extern crate json;

use std::{convert::TryInto, error, slice};
use json::{object::Object, JsonValue};
use self::lean_sys::{lean_array_push, lean_mk_empty_array, lean_obj_res, lean_mk_string_from_bytes, lean_string_cstr, lean_string_len, lean_array_size, lean_array_uget, lean_initialize};
extern crate oxrdfio;
use oxrdfio::{RdfFormat, RdfParser, ParseError, RdfSerializer}; // RdfSerializer
use oxrdf::{BlankNode, Literal, NamedNode, Quad, Subject, Term, Triple};

use crate::{from_term::{from_term, from_triple}, utils::{lean_mk_string_from_str, lean_mk_string_from_string, lean_string_str, lean_string_utf8}};

pub fn i_parse(s: lean_obj_res, fmt: lean_obj_res, base_iri: lean_obj_res) -> Option<lean_obj_res> {
    let mut parser = RdfParser::from_format(RdfFormat::from_media_type(lean_string_str(fmt).ok()?)?);

    let base_iri_str = lean_string_str(base_iri).ok()?;
    if base_iri_str != "" {
        parser = parser.with_base_iri(base_iri_str).ok()?;
    }

    let mut x = unsafe { lean_mk_empty_array() };
    for quad in parser.parse_read(lean_string_utf8(s)) {
        x = unsafe { lean_array_push(x, lean_mk_string_from_string(from_triple(quad.ok()?.into()).to_string())) };
    }

    Some(x)
}

#[no_mangle]
pub fn parse(s: lean_obj_res, fmt: lean_obj_res, base_iri: lean_obj_res) -> lean_obj_res {
    i_parse(s, fmt, base_iri).unwrap()
} 


// #[no_mangle]
// pub fn parse(s: lean_obj_res, fmt: lean_obj_res, base_iri: lean_obj_res) -> lean_obj_res {
//     let mut parser = RdfFormat::from_media_type(lean_string_str(fmt)).and_then(|fmt| Some(RdfParser::from_format(fmt)));

//     let base_iri_str = lean_string_str(base_iri);
//     if base_iri_str != "" {
//         parser = parser.and_then(|p| p.with_base_iri(base_iri_str).ok());
//     }

//     let mut x = unsafe { lean_mk_empty_array() };
//     if let Some(parse) = parser {
//         for quad in parse.parse_read(lean_string_utf8(s)) {
//             if let Ok(safe_quad) = quad {
//                 x = unsafe { lean_array_push(x, lean_mk_string_from_string(from_triple(safe_quad.into()).to_string())) };
//             } else {
//                 break;
//             }
//         }
//     };

//     x
// }
