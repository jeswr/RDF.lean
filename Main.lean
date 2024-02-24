import Lean
import Lean.Data.Json.Basic
import Lean.Data.Json.Parser
import Lean.Data.Json.Printer
import lib.Triple

open Lean Json ToJson FromJson

@[extern "parse"]
opaque parseFromRustBridge : String → String → String → Array String

@[extern "serialize"]
opaque serializeFromRustBridge : Array String → String → String

def toTriple (s: String): Except String Triple := do return (← fromJson? (← Json.parse s))
def parseFromRust (s: String) (format: String) (base: String): Except String (Array Triple) := (parseFromRustBridge s format base).mapM toTriple
def serializeFromRust (triples: Array Triple) (format: String) : String := serializeFromRustBridge ((triples.map toJson).map toString) format

def main : IO Unit := do
  let str: String := "<http://example.org/a> <http://example.org/b> </c>, \"c\", \"c\", true, \"hello world!\"@en ."
  let triples ← IO.ofExcept (parseFromRust str "text/turtle" "http://example.org")
  IO.println triples
  IO.println $ serializeFromRust triples "text/turtle"
