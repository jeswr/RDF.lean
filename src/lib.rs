extern crate json;
extern crate oxrdf;
extern crate oxrdfio;
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
