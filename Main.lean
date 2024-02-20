import Lean
import Lean.Data.Json.Basic
import Lean.Data.Json.Parser
import Lean.Data.Json.Printer

open Lean Json ToJson FromJson

structure Entry: Type where
  entry_date: String
  description: String
  --debit: Float
  --credit: Float
deriving ToJson, FromJson, Inhabited, Repr

-- Check types
-- #check #["foo"]
-- #check 1000.00

structure Ledger : Type where
  account_name: String
  account_number: String
  entries: Array Entry
deriving Lean.ToJson, Lean.FromJson, Inhabited, Repr

def get_ledger_from_json_string (s: String): Except String Ledger := do
  let j : Json <- Json.parse s
  let ledger : Ledger <- fromJson? j
  return ledger

def ledger_account_string := "{     \"account_name\": \"Example Company\",     \"account_number\": \"1234567890\",     \"entries\": [       {         \"entry_date\": \"2023-10-14\",         \"description\": \"Opening Balance\",         \"debit\": 10000.00,         \"credit\": 0.00       },       {         \"entry_date\": \"2023-10-15\",         \"description\": \"Sale of Products\",         \"debit\": 0.00,         \"credit\": 5000.00       },       {         \"entry_date\": \"2023-10-16\",         \"description\": \"Purchase of Supplies\",         \"debit\": 3000.00,         \"credit\": 0.00       },       {         \"entry_date\": \"2023-10-17\",         \"description\": \"Payment from Customer\",         \"debit\": 2000.00,         \"credit\": 0.00       },       {         \"entry_date\": \"2023-10-18\",         \"description\": \"Utilities Expense\",         \"debit\": 0.00,         \"credit\": 1000.00       }     ]   }"
#eval (get_ledger_from_json_string ledger_account_string)
#eval toJson (get_ledger_from_json_string ledger_account_string).toOption.get!

inductive Subject where
  | NamedNode : String → Subject
  | BlankNode : String → Subject
deriving Lean.ToJson, Lean.FromJson, Repr

instance : ToString Subject where
  toString s := match s with
  | Subject.NamedNode s => s
  | Subject.BlankNode s => s

inductive Predicate where
  | NamedNode : String → Predicate
deriving Lean.ToJson, Lean.FromJson, Repr

instance : ToString Predicate where
  toString s := match s with
  | Predicate.NamedNode s => s

inductive Object where
  | NamedNode : String → Object
  | BlankNode : String → Object
  | Literal : String → String → Option String → Object
deriving Lean.ToJson, Lean.FromJson, Repr

inductive Literal where
  | Language : String → String → Literal
  | DataTyped : String → String → Literal

-- TODO: Mirror the rust literal coercions here.

-- instance : ToString Object where
--   toString s := match s with
--   | Object.NamedNode s => s
--   | Object.BlankNode s => s
--   | Object.Literal s => s

inductive Term where
  | NamedNode : String → Term
  | BlankNode : String → Term
  | Literal : String → String → Option String → Term
  | Variable : String → Term
  | DefaultGraph
deriving Lean.ToJson, Lean.FromJson, Repr

-- instance : ToString Term where
--   toString s := match s with
--   | Term.NamedNode s => s
--   | Term.BlankNode s => s
--   | Term.Literal s => s
--   | Term.Variable s => s
--   | Term.DefaultGraph => "DefaultGraph"

structure Triple where
  subject : Subject
  predicate : Predicate
  object : Object
deriving Lean.ToJson, Lean.FromJson, Repr


-- @[extern "input_test"]
-- opaque parseFromRust : Json → Json

@[extern "parse"]
opaque parseFromRustBridge : String → String → String → Array String

def toTriple (s: String): Except String Triple := do return (← fromJson? (← Json.parse s))

def parseFromRust (s: String) (format: String) (base: String): Except String (Array Triple) := (parseFromRustBridge s format base).mapM toTriple

-- def toTriple (s: String) : Except String Triple := fromJson? · fromString

def main := do
  -- let triples ← parseFromRust "<http://example.org/a> <http://example.org/b> <http://example.org/c>, \"c\", true, \"hello world!\"@en ." "text/turtle" "http://example.org"
  -- let startTime ← IO.monoMsNow
  -- let s ← IO.FS.readFile "ledger_account.json"
  -- -- Test Json Parser
  -- let ledger : Ledger <- IO.ofExcept (get_ledger_from_json_string s)
  -- IO.println (toJson ledger)
  -- IO.println (toJson $ (⟨ Subject.NamedNode "foo", Predicate.NamedNode "foo", Object.Literal "foo" "http://example.org/langstring" "en" ⟩ : Triple))
  -- IO.println (toJson $ (⟨ Subject.NamedNode "foo", Predicate.NamedNode "foo", Object.Literal "foo" "http://example.org/langstring" none ⟩ : Triple))
  -- IO.println $ parseFromRust (toJson $ (⟨ Subject.NamedNode "foo", Predicate.NamedNode "foo", Object.Literal "foo" "http://example.org/langstring" none ⟩ : Triple))
  -- IO.println $ parseFromRust (toString (toJson $ (⟨ Subject.NamedNode "foo", Predicate.NamedNode "foo", Object.Literal "foo" "http://example.org/langstring" none ⟩ : Triple)))

  -- IO.println (toJson $ (⟨ Subject.NamedNode "foo", Predicate.NamedNode "foo", Object.Literal "foo" "http://example.org/langstring" none ⟩ : Triple))

  -- timestamp
  IO.println (⟨ Subject.NamedNode "foo", Predicate.NamedNode "foo", Object.Literal "foo" "http://example.org/langstring" none ⟩: Triple)
  -- Unit.unit
