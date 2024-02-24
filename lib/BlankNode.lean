structure BlankNode where
  id : String
deriving Repr, DecidableEq

instance : ToString BlankNode where
  toString s := "_:" ++ s.id

instance : Repr BlankNode where
  reprPrec s _ := toString s
