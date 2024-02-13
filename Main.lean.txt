import Lean.Data.Json.FromToJson

@[extern "parse_from_rust"]
opaque parseFromRust : String → String → String → Array (Array String)

inductive Subject where
  | NamedNode : String → Subject
  | BlankNode : String → Subject
deriving Repr

instance : ToString Subject where
  toString s := match s with
  | Subject.NamedNode s => "<" ++ s ++ ">"
  | Subject.BlankNode s => "_:" ++ s

inductive Predicate where
  | NamedNode : String → Predicate
deriving Repr

instance : ToString Predicate where
  toString s := match s with
  | Predicate.NamedNode s => "<" ++ s ++ ">"

inductive Object where
  | NamedNode : String → Object
  | BlankNode : String → Object
  | Literal : String → Object
deriving Repr

instance : ToString Object where
  toString s := match s with
  | Object.NamedNode s => "<" ++ s ++ ">"
  | Object.BlankNode s => "_:" ++ s
  | Object.Literal s => "\"" ++ s ++ "\""

inductive Term where
  | NamedNode : String → Term
  | BlankNode : String → Term
  | Literal : String → Term
  | Variable : String → Term
  | DefaultGraph
deriving Inhabited, Repr

def Term.value : Term → String
  | Term.NamedNode s => s
  | Term.BlankNode s => s
  | Term.Literal s => s
  | Term.Variable s => s
  | Term.DefaultGraph => "DefaultGraph"

def Term.termType : Term → String
  | Term.NamedNode _ => "NamedNode"
  | Term.BlankNode _ => "BlankNode"
  | Term.Literal _ => "Literal"
  | Term.Variable _ => "Variable"
  | Term.DefaultGraph => "DefaultGraph"

instance : ToString Term where
  toString s := match s with
  | Term.NamedNode s => "<" ++ s ++ ">"
  | Term.BlankNode s => "_:" ++ s
  | Term.Literal s => "\"" ++ s ++ "\""
  | Term.Variable s => "?" ++ s
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


def toSubject (t s : String) : Except String Subject :=
  if t = "NamedNode" then Except.ok $ Subject.NamedNode s
  else if t = "BlankNode" then Except.ok $ Subject.BlankNode s
  else Except.error ("Invalid Subject type [" ++ t ++ "]")

def toPredicate (t s : String) : Except String Predicate :=
  if t = "NamedNode" then Except.ok $ Predicate.NamedNode s
  else Except.error ("Invalid Predicate type [" ++ t ++ "]")

def toObject (t s : String) : Except String Object :=
  if t = "NamedNode" then Except.ok $ Object.NamedNode s
  else if t = "BlankNode" then Except.ok $ Object.BlankNode s
  else if t = "Literal" then Except.ok $ Object.Literal s
  else Except.error ("Invalid Object type [" ++ t ++ "]")

def toTriple (t : Array String) : Except String Triple := do
  if h: t.size = 6 then
    pure ⟨
      (← toSubject (t.get ⟨0, by rw [h]; simp (config := {decide := true})⟩) (t.get ⟨1, by rw [h]; simp (config := {decide := true})⟩)),
      (← toPredicate (t.get ⟨2, by rw [h]; simp (config := {decide := true})⟩) (t.get ⟨3, by rw [h]; simp (config := {decide := true})⟩)),
      (← toObject (t.get ⟨4, by rw [h]; simp (config := {decide := true})⟩) (t.get ⟨5, by rw [h]; simp (config := {decide := true})⟩))
    ⟩
  else Except.error ("Invalid Triple length [" ++ toString t.size ++ "]")

def fromTriple (t : Triple) : Array String := #[
  Term.termType t.subject, Term.value t.subject,
  Term.termType t.predicate, Term.value t.predicate,
  Term.termType t.object, Term.value t.object
]

def convert (str: Array (Array String)): Except String (Array Triple) := str.mapM toTriple

def parse (a b c: String) := convert $ parseFromRust a b c

def main : IO Unit :=
  -- IO.println $ addFromRust "Hello from Lean!"
  -- IO.println $ parse "@prefix e: <http://e/> . <http://example.org/a> <http://example.org/b> \"c\", \"d\", \"f\", \"f\"@en, [], e:x ." "text/n3" "http://example.org/my/base"
  IO.println $ Lean.FromJson.fromJson? "}|"
