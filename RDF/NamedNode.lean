import Lean

structure NamedNode where
  iri : String
deriving DecidableEq, Lean.ToJson, Lean.FromJson

instance : ToString NamedNode where
  toString s := "<" ++ s.iri ++ ">"

instance : Repr NamedNode where
  reprPrec s _ := toString s

macro "#⟨" s:term "⟩" : term => `(NamedNode.mk $s)
