import lib.BlankNode
import lib.NamedNode
import lib.Literal

inductive Subject where
  | NamedNode : NamedNode → Subject
  | BlankNode : BlankNode → Subject
deriving Repr, DecidableEq

instance : ToString Subject where
  toString : Subject → String
  | Subject.NamedNode s => toString s
  | Subject.BlankNode s => toString s

inductive Object where
  | NamedNode : NamedNode → Object
  | BlankNode : BlankNode → Object
  | Literal : Literal → Object
deriving Repr, DecidableEq

instance : ToString Object where
  toString : Object → String
  | Object.NamedNode s => toString s
  | Object.BlankNode s => toString s
  | Object.Literal s => toString s

structure Triple where
  subject : Subject
  predicate : NamedNode
  object : Object
deriving Repr, DecidableEq

instance : ToString Triple where
  toString triple := "(" ++ toString triple.subject ++ ", " ++ toString triple.predicate ++ ", " ++ toString triple.object ++ ")"

#eval toString (⟨Subject.NamedNode ⟨"http://example.com/subject"⟩, ⟨"http://example.com/predicate"⟩, Object.Literal ((1:):)⟩ : Triple)
