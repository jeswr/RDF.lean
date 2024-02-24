import Lean
import RDF.Triple

@[extern "serialize"]
opaque serializeFromRustBridge : Array String → String → String

def RDFSerialise (triples: Array Triple) (format: String) : String := serializeFromRustBridge (triples.map (toString ∘ Lean.ToJson.toJson)) format
