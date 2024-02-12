extern crate oxrdfio;
extern crate oxrdf;
extern crate oxttl;
extern crate cxx;

use oxrdfio::{RdfFormat, RdfParser, FromReadQuadReader}; // RdfSerializer
use std::io::Read;
use std::ffi::{CString};
use oxttl::TurtleParser;
use oxrdf::{NamedNode, NamedNodeRef, Quad};
use cxx::Vector;

// #[repr(C)]
pub struct QuadArray {
    pub data: *const Quad,
    pub len: usize,
}

#[no_mangle]
pub extern "C" fn parse() -> cxx::UniquePtr<cxx::Vector<CString>> {
    RdfFormat.values();
    let parser = RdfParser::from_format(RdfFormat::Turtle);


    // let quad = Quad {
    //     subject: NamedNode::new("http://example.com/s").unwrap().into(),
    //     predicate: NamedNode::new("http://example.com/p").unwrap().into(),
    //     object: NamedNode::new("http://example.com/o").unwrap().into(),
    //     graph_name: NamedNode::new("http://example.com/g").unwrap().into(),
    // };
    return cxx::Vector::new();
    // return QuadArray {
    //     data: &quad as *const Quad,
    //     len: 1,
    // };
    // return Vec::new();
}


pub extern "C" fn parse<R: Read>(_fmt: *const i8) -> Vec<Quad> {
    // FIXME: Use types rather than runtime errors
    // let fm = RdfFormat::from_media_type(unsafe { CStr::from_ptr(fmt) }.to_str().unwrap()).unwrap();
    // let parser = RdfParser::from_format(fm);
    let parser = RdfParser::from_format(RdfFormat::Turtle);
    // let base: &str = unsafe { CStr::from_ptr(base_iri) }.to_str().unwrap();
    
//     if base != "" {
//         let res = parser.with_base_iri(base).unwrap().parse_read(unsafe { CStr::from_ptr(fmt) }.to_str().unwrap().as_ref());
    }


// let f2 = b"@base <http://example.com/> .
// @prefix schema: <http://schema.org/> .
// <foo> a schema:Person ;
//     schema:name \"Foo\" .
// <bar> a schema:Person ;
//     schema:name \"Bar\" .";

//     let collected: Result<Vec<Quad>, oxrdfio::ParseError> = parser.parse_read(f2.as_ref()).collect();
//     return collected.unwrap();
// }

// #[no_mangle]
// pub extern "C" fn add_from_rust(a : i32, b : i32) -> i32 {

// let file = b"@base <http://example.com/> .
// @prefix schema: <http://schema.org/> .
// <foo> a schema:Person ;
//     schema:name \"Foo\" .
// <bar> a schema:Person ;
//     schema:name \"Bar\" .";

// let schema_person = NamedNodeRef::new("http://schema.org/Person").unwrap();
// let mut count = 0;
// for triple in TurtleParser::new().parse_read(file.as_ref()) {
//     let triple = triple.unwrap();
//     if triple.predicate == rdf::TYPE && triple.object == schema_person.into() {
//         count += 1;
//     }
// }

// print!("Found {} schema:Person triples\n", count);
// // assert_eq!(2, count);

//     return a + b + 5 + count
// }