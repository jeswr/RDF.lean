import Lean
import RDF.Triple

@[extern "parse"]
opaque parseFromRustBridge : String → String → String → Array String

def toTriple (s: String): Except String Triple := do return (← Lean.FromJson.fromJson? (← Lean.Json.parse s))
def RDFParse (s: String) (format: String) (base: String): Except String (Array Triple) := (parseFromRustBridge s format base).mapM toTriple
