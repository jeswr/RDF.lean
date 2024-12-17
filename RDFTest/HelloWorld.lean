import RDF
open RDF

/-! A test of the current tokenization scheme. -/
/-- info: "([5, 3], 4 + (2 * 1))" -/
#guard_msgs in
#eval RDFSerialize #[ ⟪_:"b0", #⟨"http://example.org/predicate"⟩, true⟫ ] "text/turtle"
