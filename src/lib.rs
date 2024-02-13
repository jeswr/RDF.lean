extern crate lean_sys;
extern crate oxrdf;
extern crate json;

use std::{error, slice};
use json::JsonValue;
use lean_sys::{lean_array_push, lean_mk_empty_array, lean_obj_res, lean_mk_string_from_bytes, lean_string_cstr, lean_string_len, lean_array_size, lean_array_uget};
extern crate oxrdfio;
use oxrdfio::{RdfFormat, RdfParser, ParseError, RdfSerializer}; // RdfSerializer
use oxrdf::{BlankNode, Literal, NamedNode, Quad, Subject, Term, Triple};
use std::str;

pub fn to_term(term_type: &str, value: &str) -> Option<Term> {
    if term_type == "NamedNode" {
        return Some(NamedNode::new(value).unwrap().into());
    } else if term_type == "BlankNode" {
        return Some(BlankNode::new(value).unwrap().into());
    } else if term_type == "Literal" {
        // Literal::new_typed_literal(value, datatype);
        // Literal::new_language_tagged_literal(value, language);
        // Literal::new_simple_literal(value);
        // Literal::new_language_tagged_literal_unchecked(value, language);
        // Literal::
        return Some(NamedNode::new("http://example.com/error").unwrap().into());
    } else if term_type == "DefaultGraph" {
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
        let _quad_strings = array_from_lean_string_array(quad);
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

// pub fn Triple::from

// Triple::From<JsonValue>

pub fn to_term_2() -> Term {
    return NamedNode::new("http://example.com/s").unwrap().into();
}

// impl From<JsonValue> for Triple {
//     fn from(json: JsonValue) -> Self {
//         // let subject = json["subject"]["NamedNode"].as_str().unwrap();
//         // let predicate = json["predicate"]["NamedNode"].as_str().unwrap();
//         // let object = json["object"].as_str().unwrap();
        // return Triple {
        //     subject: NamedNode::new(subject).unwrap().into(),
        //     predicate: NamedNode::new(subject).unwrap().into(),
        //     object: NamedNode::new(subject).unwrap().into(),
        // };
//     }
// }

pub fn from_subject(subject: Subject) -> JsonValue {
    if subject.is_blank_node() {
        return json::object! {
            "BlankNode" => subject.to_string()
        };
    } else if subject.is_named_node() {
        return json::object! {
            "NamedNode" => subject.to_string()
        };
    }
    panic!("Invalid subject type");
}

pub fn from_predicate(predicate: NamedNode) -> JsonValue {
    return json::object! {
        "NamedNode" => predicate.to_string()
    };
}

pub fn from_term(object: Term) -> JsonValue {
    if object.is_blank_node() {
        return json::object! {
            "BlankNode" => object.to_string()
        };
    } else if object.is_named_node() {
        return json::object! {
            "NamedNode" => object.to_string()
        };
    } else if object.is_literal() {
        let literal: Literal = object;
        if literal.is_language_tagged_literal() {
            return json::object! {
                "Literal" => [
                    literal.value(),
                    "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString",
                    literal.language().unwrap()
                ]
            };
        } else {
            return json::object! {
                "Literal" => [
                    literal.value(),
                    literal.datatype().unwrap().to_string()
                ]
            };
        }
    }
    panic!("Invalid object type");
}

pub fn get_subject(json: JsonValue) -> Subject {
    if json.has_key("NamedNode") {
        let subject = json["NamedNode"].as_str().unwrap();
        return NamedNode::new(subject).unwrap().into();
    } else if json.has_key("BlankNode") {
        let subject = json["BlankNode"].as_str().unwrap();
        return BlankNode::new(subject).unwrap().into();
    }
    panic!("Invalid subject type");
}

pub fn get_predicate(json: JsonValue) -> NamedNode {
    if json.has_key("NamedNode") {
        let subject = json["NamedNode"].as_str().unwrap();
        return NamedNode::new(subject).unwrap().into();
    }
    panic!("Invalid predicate type");
}

pub fn get_object(json: JsonValue) -> Term {
    if json.has_key("NamedNode") {
        let subject = json["NamedNode"].as_str().unwrap();
        return NamedNode::new(subject).unwrap().into();
    } else if json.has_key("BlankNode") {
        let subject = json["BlankNode"].as_str().unwrap();
        return BlankNode::new(subject).unwrap().into();
    } else if json.has_key("Literal") {
        let subject = json["Literal"][0].as_str().unwrap();
        if json["Literal"][2].is_null() {
            return Literal::new_typed_literal(
                subject,
                NamedNode::new(json["Literal"][1].as_str().unwrap()).unwrap()
            ).into();
        }
        // FIXME: Panic if the datatype is not a langString here
        return Literal::new_language_tagged_literal(
            subject,
            json["Literal"][2].as_str().unwrap()
        ).unwrap().into();
    }
    panic!("Invalid object type");
}

#[no_mangle]
pub fn input_test(object: lean_obj_res) -> lean_obj_res {
    // lean_is_ref(object);
    // JSON.p
    // let str = lean_to_rust_string(object);
    // let js: Triple = json::parse(str).unwrap().into();
    // let s = js["subject"]["NamedNode"].as_str().unwrap();
    // let o = &js["object"]["Literal"][1].as_str().unwrap();

    let js = Triple {
        subject: to_term_2().,
        predicate: NamedNode::new(subject).unwrap().into(),
        object: NamedNode::new(subject).unwrap().into(),
    };

    println!("Hello from Rust {js}!");

    return object;
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
