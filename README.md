# RDF.lean

This is an RDF library for Lean 4. Parsing and serialization is done using [oxrdfio](https://crates.io/crates/oxrdfio).

## Usage

### Parsing a string

```lean
import RDF

def main : IO Unit := do
  IO.println (← IO.ofExcept $ RDFParse "</a> </b> true ." "text/turtle" "http://example.org")
```

### Serializing a string

```lean
import RDF

def main : IO Unit := do
  let triples: Array Triple := #[
    ⟨Subject.NamedNode ⟨"http://example.org"⟩, Predicate.NamedNode ⟨"http://example.org"⟩, (1:)⟩
  ]
  IO.println $ RDFSerialize triples "text/turtle"
```
