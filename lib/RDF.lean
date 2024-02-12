inductive Subject where
  | NamedNode : String → Subject
  | BlankNode : String → Subject
deriving Repr

instance : ToString Subject where
  toString s := match s with
  | Subject.NamedNode s => s
  | Subject.BlankNode s => s

inductive Predicate where
  | NamedNode : String → Predicate
deriving Repr

instance : ToString Predicate where
  toString s := match s with
  | Predicate.NamedNode s => s

inductive Object where
  | NamedNode : String → Object
  | BlankNode : String → Object
  | Literal : String → Object
deriving Repr

instance : ToString Object where
  toString s := match s with
  | Object.NamedNode s => s
  | Object.BlankNode s => s
  | Object.Literal s => s

inductive Term where
  | NamedNode : String → Term
  | BlankNode : String → Term
  | Literal : String → Term
  | Variable : String → Term
  | DefaultGraph
deriving Inhabited, Repr

instance : ToString Term where
  toString s := match s with
  | Term.NamedNode s => s
  | Term.BlankNode s => s
  | Term.Literal s => s
  | Term.Variable s => s
  | Term.DefaultGraph => "DefaultGraph"

structure Triple where
  subject : Subject
  predicate : Predicate
  object : Object
deriving Repr

instance : ToString Triple where
  toString t := "(" ++ toString t.subject ++ ", " ++ toString t.predicate ++ ", " ++ toString t.object ++ ")"

instance : Coe Subject Term where
  coe s := match s with
  | Subject.NamedNode s => Term.NamedNode s
  | Subject.BlankNode s => Term.BlankNode s

instance : Coe Predicate Term where
  coe s := match s with
  | Predicate.NamedNode s => Term.NamedNode s

instance : Coe Object Term where
  coe s := match s with
  | Object.NamedNode s => Term.NamedNode s
  | Object.BlankNode s => Term.BlankNode s
  | Object.Literal s => Term.Literal s
