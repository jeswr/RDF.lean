import Lean
import RDF.BlankNode
import RDF.NamedNode
import RDF.Literal

inductive Subject where
  | NamedNode : NamedNode → Subject
  | BlankNode : BlankNode → Subject
deriving Repr, DecidableEq, Lean.ToJson, Lean.FromJson

instance : ToString Subject where
  toString : Subject → String
  | Subject.NamedNode s => toString s
  | Subject.BlankNode s => toString s

instance : Coe NamedNode Subject where
  coe n := Subject.NamedNode n
instance : Coe BlankNode Subject where
  coe n := Subject.BlankNode n

inductive Predicate where
  | NamedNode : NamedNode → Predicate
deriving Repr, DecidableEq, Lean.ToJson, Lean.FromJson

instance : ToString Predicate where
  toString : Predicate → String
  | Predicate.NamedNode s => toString s

instance : Coe NamedNode Predicate where
  coe n := Predicate.NamedNode n

inductive Object where
  | NamedNode : NamedNode → Object
  | BlankNode : BlankNode → Object
  | Literal : Literal → Object
deriving Repr, DecidableEq, Lean.ToJson, Lean.FromJson

instance : ToString Object where
  toString : Object → String
  | Object.NamedNode s => toString s
  | Object.BlankNode s => toString s
  | Object.Literal s => toString s

instance : Coe NamedNode Object where
  coe n := Object.NamedNode n
instance : Coe BlankNode Object where
  coe n := Object.BlankNode n
instance : Coe Literal Object where
  coe n := Object.Literal n

structure Triple where
  subject : Subject
  predicate : Predicate
  object : Object
deriving Repr, DecidableEq, Lean.ToJson, Lean.FromJson

instance : ToString Triple where
  toString triple := "(" ++ toString triple.subject ++ ", " ++ toString triple.predicate ++ ", " ++ toString triple.object ++ ")"
