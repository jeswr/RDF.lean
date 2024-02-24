import Lean
import lib.NamedNode
import lib.vocab

structure LanguageTaggedStringLiteral where
  value : String
  language : String
deriving DecidableEq, Lean.ToJson, Lean.FromJson

instance : ToString LanguageTaggedStringLiteral where
  toString s := "\"" ++ s.value ++ "\"@" ++ s.language

instance : Repr LanguageTaggedStringLiteral where
  reprPrec s _ := toString s

structure TypedLiteral where
  value : String
  datatype : NamedNode
deriving DecidableEq, Lean.ToJson, Lean.FromJson

instance : ToString TypedLiteral where
  toString s := "\"" ++ s.value ++ "\"^^" ++ (toString s.datatype)

instance : Repr TypedLiteral where
  reprPrec s _ := toString s

inductive Literal where
  | LanguageTaggedString : LanguageTaggedStringLiteral → Literal
  | Typed : TypedLiteral → Literal
deriving Repr, DecidableEq, Lean.ToJson, Lean.FromJson

instance : ToString Literal where
  toString : Literal → String
    | Literal.LanguageTaggedString l => toString l
    | Literal.Typed l => toString l

def Literal.toTyped? : Literal → (Option TypedLiteral)
  | Literal.Typed l => some l
  | _ => none
instance : Coe Literal (Option TypedLiteral) where coe := Literal.toTyped?
instance : Coe TypedLiteral Literal where coe := Literal.Typed

def Literal.toLanguageTaggedString? : Literal → (Option LanguageTaggedStringLiteral)
  | Literal.LanguageTaggedString l => some l
  | _ => none
instance : Coe Literal (Option LanguageTaggedStringLiteral) where coe := Literal.toLanguageTaggedString?
instance : Coe LanguageTaggedStringLiteral Literal where coe := Literal.LanguageTaggedString

-- FIXME: This is to help along nested optionals
-- see https://leanprover.zulipchat.com/#narrow/stream/113488-general/topic/.E2.9C.94.20Coercing.20recursive.20.60Option.60s/near/422643899
instance [Coe TypedLiteral (Option α)] : Coe Literal (Option α) where
  coe a := match Literal.toTyped? a with
    | some l => l
    | _ => none

def Int.toTypedLiteral (i: Int): TypedLiteral := ⟨toString i, XSD.integer⟩
instance : Coe Int TypedLiteral where coe := Int.toTypedLiteral

def TypedLiteral.toInt? (literal: TypedLiteral): Option Int :=
  if literal.datatype == XSD.integer then String.toInt? literal.value else none
instance : Coe TypedLiteral (Option Int) where coe := TypedLiteral.toInt?

theorem stringToIntRoundTrip (x: Int) : (String.toInt? (toString x)) = some x := by sorry
theorem intInverse (x: Int) : (TypedLiteral.toInt? (Int.toTypedLiteral x)) = some x := by
  simp [TypedLiteral.toInt?, Int.toTypedLiteral]
  exact (stringToIntRoundTrip x)
theorem intEq (x y: Int) (h1: x = y) : Int.toTypedLiteral x = Int.toTypedLiteral y := by simp [Int.toTypedLiteral, h1]
theorem intEqInv (x y: Int) (h1: Int.toTypedLiteral x = Int.toTypedLiteral y) : x = y := by
  simp [Int.toTypedLiteral] at h1
  have h2: (String.toInt? (toString x)) = some x := stringToIntRoundTrip x
  rw [h1, stringToIntRoundTrip y] at h2;
  simp [some] at h2;
  exact h2.symm;

def Bool.toTypedLiteral (i: Bool): TypedLiteral := ⟨toString i, XSD.boolean⟩
instance : Coe Bool TypedLiteral where coe := Bool.toTypedLiteral

def TypedLiteral.toBool? (literal: TypedLiteral): Option Bool :=
  if literal.datatype = XSD.boolean then
    match literal.value.toLower with
    | "true" | "1" => some true
    | "false" | "0" => some false
    | _ => none
  else none
instance : Coe TypedLiteral (Option Bool) where coe := TypedLiteral.toBool?

theorem boolInverse (x: Bool) : (TypedLiteral.toBool? (Bool.toTypedLiteral x)) = some x := by cases x <;> exact rfl
-- TODO: Use notions of injectivity here
theorem boolEq (x y: Bool) (h1: x = y) : Bool.toTypedLiteral x = Bool.toTypedLiteral y := by simp [Bool.toTypedLiteral, h1]
theorem boolEqInv (x y: Bool) (h1: Bool.toTypedLiteral x = Bool.toTypedLiteral y) : x = y := by
  simp [Bool.toTypedLiteral] at h1
  cases x; cases y;
  simp [toString] at h1;
  simp;
  simp [toString] at h1;
  contradiction
  cases y;
  simp [toString] at h1;
  contradiction;
  simp [toString] at h1;
  simp;

instance : Coe Float TypedLiteral where coe s := ⟨toString s, XSD.float⟩

def String.toTypedLiteral (s: String): TypedLiteral := ⟨s, XSD.string⟩
instance : Coe String TypedLiteral where coe := String.toTypedLiteral

def TypedLiteral.toString? (literal: TypedLiteral): Option String := if literal.datatype = XSD.string then some literal.value else none
instance : Coe TypedLiteral (Option String) where coe := TypedLiteral.toString?

theorem stringInverse (x: String) : (TypedLiteral.toString? (String.toTypedLiteral x)) = some x := by exact rfl
theorem stringEq (x y: String) (h1: x = y) : String.toTypedLiteral x = String.toTypedLiteral y := by simp [String.toTypedLiteral, h1]
theorem stringEqInv (x y: String) (h1: String.toTypedLiteral x = String.toTypedLiteral y) : x = y := by simp [String.toTypedLiteral] at h1; exact h1

instance : Coe UInt8 TypedLiteral  where coe s := ⟨toString s, XSD.integer⟩
instance : Coe UInt16 TypedLiteral where coe s := ⟨toString s, XSD.integer⟩
instance : Coe UInt32 TypedLiteral where coe s := ⟨toString s, XSD.integer⟩
instance : Coe UInt64 TypedLiteral where coe s := ⟨toString s, XSD.integer⟩
