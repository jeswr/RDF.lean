extern crate lean_sys;
extern crate oxrdf;
use std::slice;
use lean_sys::{lean_array_push, lean_mk_empty_array, lean_obj_res, lean_mk_string_from_bytes, lean_string_cstr, lean_string_len, lean_array_size, lean_array_uget};
extern crate oxrdfio;
use oxrdfio::{RdfFormat, RdfParser, FromReadQuadReader, ParseError}; // RdfSerializer
use oxrdf::{NamedNode, NamedNodeRef, Quad};

pub fn parse(s: &[u8], fmt: &str, base_iri: &str) -> Option<Vec<Quad>> {
    let mut parser = RdfFormat::from_media_type(fmt).and_then(|fmt| Some(RdfParser::from_format(fmt)));

    if base_iri != "" {
        parser = parser.and_then(|p| p.with_base_iri(base_iri).ok());
    }

    return parser.and_then(|p| p.parse_read(s).collect::<Result<Vec<Quad>, ParseError>>().ok());
}

#[no_mangle]
pub fn parse_from_rust(s: lean_obj_res, fmt: lean_obj_res, base_iri: lean_obj_res) -> lean_obj_res {
    let str = lean_to_rust_string(s);
    let fmt_str = lean_to_rust_string(fmt);
    let base_iri_str = lean_to_rust_string(base_iri);

    let quads = parse(str.as_bytes(), fmt_str, base_iri_str).unwrap();

    let mut x = unsafe { lean_mk_empty_array() };
    for q in quads {
        x = unsafe { lean_array_push(x, lean_mk_rust_string(q.subject.to_string().as_str())) };
        x = unsafe { lean_array_push(x, lean_mk_rust_string(q.predicate.to_string().as_str())) };
        x = unsafe { lean_array_push(x, lean_mk_rust_string(q.object.to_string().as_str())) };
    }
    return x;
}

pub fn lean_mk_rust_string(s: &str) -> lean_obj_res {
    unsafe { lean_mk_string_from_bytes(s.as_ptr(), s.len()) }
}

pub fn lean_to_rust_string(s: lean_obj_res) -> &'static str {
    let ptr = unsafe { lean_string_cstr(s) };
    let len = unsafe { lean_string_len(s) };

    // We can re-build a str out of ptr and len. This is all unsafe because
    // we are responsible for making sure the two components are valid:
    let s = unsafe {
        // First, we build a &[u8]...
        let slice = slice::from_raw_parts(ptr, len);

        // ... and then convert that slice into a string slice
        std::str::from_utf8(slice)
    };

    return s.unwrap();
}

pub fn lean_mk_string_array(strings: Vec<&str>) -> lean_obj_res {
    let mut x = unsafe { lean_mk_empty_array() };
    for s in strings {
        x = unsafe { lean_array_push(x, lean_mk_rust_string(s)) };
    }
    return x;
}

pub fn array_from_lean_string_array(lean_array: lean_obj_res) -> Vec<&'static str> {
    let mut result = Vec::new();
    let size = unsafe { lean_array_size(lean_array) };
    for i in 0..size {
        let item = lean_to_rust_string(unsafe { lean_array_uget(lean_array, i) });
        result.push(item);
    }
    return result;
}

#[no_mangle]
pub extern "C" fn add_from_rust(a: lean_obj_res) -> lean_obj_res {
    let vec: Vec<&str> = ["NamedNode", "http://example.org/test", "NamedNode", "http://example.org/predicate", "Literal", "belting", lean_to_rust_string(a), "5"].to_vec();
    return lean_mk_string_array(vec.clone())
}
