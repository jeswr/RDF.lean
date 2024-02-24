structure NamedNode where
  iri : String
deriving DecidableEq

instance : ToString NamedNode where
  toString s := "<" ++ s.iri ++ ">"

instance : Repr NamedNode where
  reprPrec s _ := toString s
