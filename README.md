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
  IO.println (← IO.ofExcept $ RDFParse "[] </predicate> true, 1, \"hello\" \"hello\"@en ." "text/turtle" "http://example.org")
```

### Serializing a string

```lean
import RDF

def main : IO Unit := do
  let triples: Array Triple := #[
    ⟪_:"b0", #⟨"http://example.org/predicate"⟩, true⟫,
    ⟪_:"b0", #⟨"http://example.org/predicate"⟩, (1:)⟫,
    ⟪_:"b0", #⟨"http://example.org/predicate"⟩, "hello"⟫,
    ⟪_:"b0", #⟨"http://example.org/predicate"⟩, "hello"@"en"⟫,
    ⟪_:"b0", #⟨"http://example.org/predicate"⟩, "hello"@"en"⟫,
  ]
  IO.println $ RDFSerialize triples "text/turtle"
```
