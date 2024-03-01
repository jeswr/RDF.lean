import Lean
import RDF.BlankNode
import RDF.NamedNode
import RDF.Literal
import RDF.Vocab

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

instance : Coe Subject Object where
  coe
    | Subject.NamedNode n => Object.NamedNode n
    | Subject.BlankNode n => Object.BlankNode n

instance : Coe Predicate Object where
  coe
    | Predicate.NamedNode n => Object.NamedNode n

structure Triple where
  subject : Subject
  predicate : Predicate
  object : Object
deriving Repr, DecidableEq, Lean.ToJson, Lean.FromJson

instance : ToString Triple where
  toString triple := "⟪" ++ toString triple.subject ++ " " ++ toString triple.predicate ++ " " ++ toString triple.object ++ "⟫"

structure RDFList where
  head : Subject
  triples : List Triple

macro "⟪" s:term "," p:term "," o:term "⟫" : term => `(Triple.mk $s $p $o)

-- FIXME: Improve blank node generation to prevent collisions
def toList : List Object → RDFList
  | [] => ⟨RDF.nil, []⟩
  | x::xs =>
    let ⟨head, triples⟩ := toList xs
    let nextHead := BlankNode.mk ("b" ++ toString xs.length)
    ⟨nextHead, ⟪nextHead, RDF.first, x⟫::⟪nextHead, RDF.rest, head⟫::triples⟩

def tripleToList (s: Subject) (p: Predicate) (o: List Object) : List Triple :=
  let ⟨head, triples⟩ := toList o
  ⟪s, p, head⟫::triples

syntax "⟪" term "," term "," "[" term,* "]" : term
macro_rules | `(⟪ $s, $p, [ $o,* ] ⟫) => `(tripleToList $s $p [$o,*])
