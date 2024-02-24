import Lean
import lib.BlankNode
import lib.NamedNode
import lib.Literal
import Lean.Data.Json.Basic
import Lean.Data.Json.Parser
import Lean.Data.Json.Printer

-- open Lean Json ToJson FromJson

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
  predicate : NamedNode
  object : Object
deriving Repr, DecidableEq, Lean.ToJson, Lean.FromJson

instance : ToString Triple where
  toString triple := "(" ++ toString triple.subject ++ ", " ++ toString triple.predicate ++ ", " ++ toString triple.object ++ ")"

open Lean Json ToJson FromJson

#eval toString (⟨Subject.NamedNode ⟨"http://example.com/subject"⟩, ⟨"http://example.com/predicate"⟩, Object.Literal ((1:):)⟩ : Triple)
#eval (toJson (⟨Subject.NamedNode ⟨"http://example.com/subject"⟩, ⟨"http://example.com/predicate"⟩, Object.Literal ((1:):)⟩ : Triple))
