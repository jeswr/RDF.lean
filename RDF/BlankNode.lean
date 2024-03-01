import Lean

structure BlankNode where
  id : String
deriving Repr, DecidableEq, Lean.ToJson, Lean.FromJson

instance : ToString BlankNode where
  toString s := "_:" ++ s.id

instance : Repr BlankNode where
  reprPrec s _ := toString s

macro "_:" s:term : term => `(BlankNode.mk $s)
