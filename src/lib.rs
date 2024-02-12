extern crate lean_sys;
extern crate oxrdf;
use std::slice;
use lean_sys::{lean_array_push, lean_mk_empty_array, lean_obj_res, lean_mk_string_from_bytes, lean_string_cstr, lean_string_len, lean_array_size, lean_array_uget};
extern crate oxrdfio;
use oxrdfio::{RdfFormat, RdfParser, ParseError, RdfSerializer}; // RdfSerializer
use oxrdf::{BlankNode, GraphName::DefaultGraph, Literal, NamedNode, Quad, Term};
use std::str;

pub fn toTerm(termType: &str, value: &str) -> Option<Term> {
    if termType == "NamedNode" {
        return Some(NamedNode::new(value).unwrap().into());
    } else if termType == "BlankNode" {
        return Some(oxrdf::BlankNode::new(value).unwrap().into());
    } else if termType == "Literal" {
        // Literal::
        return Some(NamedNode::new("http://example.com/error").unwrap().into());
    } else if termType == "DefaultGraph" {
        // FIXME: Handle this case
        return None;
        // return Some(GraphName::DefaultGraph);
    } else {
        return None;
    }
}

#[no_mangle]
pub fn serialize_to_rust(quads: lean_obj_res, fmt: lean_obj_res) -> lean_obj_res {
    let fmt_str = lean_to_rust_string(fmt);
    // fixme: do error handling properly
    let mut serializer = RdfFormat::from_media_type(fmt_str).and_then(|fmt| Some(RdfSerializer::from_format(fmt).serialize_to_write(Vec::new()))).unwrap();

    let size = unsafe { lean_array_size(quads) };
    for i in 0..size {
        let quad = unsafe { lean_array_uget(quads, i) };
        let quadStrings = array_from_lean_string_array(quad);
        let _ = serializer.write_quad(&Quad {
            subject: NamedNode::new("http://example.com/s").unwrap().into(),
            predicate: NamedNode::new("http://example.com/p").unwrap().into(),
            object: NamedNode::new("http://example.com/o").unwrap().into(),
            graph_name: oxrdf::GraphName::DefaultGraph.into(),
        });
    }

    let result = serializer.finish().unwrap();
    let resultstr = str::from_utf8(&result).unwrap();
    return lean_mk_rust_string(resultstr);
}

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

    let mut y = unsafe { lean_mk_empty_array() };
    for q in quads {
        let mut x = unsafe { lean_mk_empty_array() };

        let sub_string_subject = q.subject.to_string();
        
        if q.subject.is_blank_node() {

            x = unsafe { lean_array_push(x, lean_mk_rust_string("BlankNode")) };
            let slice = &sub_string_subject[2..sub_string_subject.len()];
            x = unsafe { lean_array_push(x, lean_mk_rust_string(slice)) };
        } else if q.subject.is_named_node() {

            x = unsafe { lean_array_push(x, lean_mk_rust_string("NamedNode")) };
            let slice = &sub_string_subject[1..sub_string_subject.len()-1];
            x = unsafe { lean_array_push(x, lean_mk_rust_string(slice)) };
        } else  {
            // This should actually be an error case
            // x = unsafe { lean_array_push(x, lean_mk_rust_string("Literal")) };
        }

        x = unsafe { lean_array_push(x, lean_mk_rust_string("NamedNode")) };
        let sub_string_predicate = q.predicate.to_string();
        let slice = &sub_string_predicate[1..sub_string_predicate.len()-1];
        x = unsafe { lean_array_push(x, lean_mk_rust_string(slice)) };

        let sub_string_object = q.object.to_string();

        // // x = unsafe { lean_array_push(x, lean_mk_rust_string(q.object.to_string().as_str())) };

        if q.object.is_blank_node() {

            x = unsafe { lean_array_push(x, lean_mk_rust_string("BlankNode")) };
            let slice = &sub_string_object[2..sub_string_object.len()];
            x = unsafe { lean_array_push(x, lean_mk_rust_string(slice)) };
        } else if q.object.is_named_node() {

            x = unsafe { lean_array_push(x, lean_mk_rust_string("NamedNode")) };
            let slice = &sub_string_object[1..sub_string_object.len()-1];
            x = unsafe { lean_array_push(x, lean_mk_rust_string(slice)) };
        } else if q.object.is_literal()  {
            let object_literal: Literal = 3.into();
            print!("OBJECT LITERAL IS: {}", object_literal);
            x = unsafe { lean_array_push(x, lean_mk_rust_string("Literal")) };
            let slice = &sub_string_object[1..sub_string_object.len()-1];
            x = unsafe { lean_array_push(x, lean_mk_rust_string(slice)) };
        } else {
            // error
        }

        // FIXME: Add graph support
        // x = unsafe { lean_array_push(x, lean_mk_rust_string(q.graph_name.to_string().as_str())) };

        y = unsafe { lean_array_push(y, x) };
    }
    return y;
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
