# RDF.lean

This is an RDF library for Lean 4. Parsing and serialization is done using [oxrdfio](https://crates.io/crates/oxrdfio).

## Usage

### Getting started with Lean4

If you've not used Lean4 before first visit https://lean-lang.org/lean4/doc/quickstart.html

### Setting up in a project

This is a library designed to be used in other projects. If you have not initialised your own project [see the Functional Programming in Lean4](https://lean-lang.org/functional_programming_in_lean/hello-world/starting-a-project.html) manual to learn how to do so.

Then add the following you your `lakefile.lean` configuration file.

```lean
require RDF from git
  "https://github.com/jeswr/RDF.lean" @ "main"
```

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
