import lib.NamedNode
import lib.vocab
import LeanCopilot

structure LanguageTaggedStringLiteral where
  value : String
  language : String
deriving Repr, BEq

structure TypedLiteral where
  value : String
  datatype : NamedNode
deriving Repr, BEq

inductive Literal where
  | LanguageTaggedString : LanguageTaggedStringLiteral → Literal
  | Typed : TypedLiteral → Literal
deriving Repr, BEq

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
  coe a := match (a: Option TypedLiteral) with
    | some l => l
    | _ => none

def Int.toTypedLiteral (i: Int): TypedLiteral := ⟨toString i, XSD.integer⟩
instance : Coe Int TypedLiteral where coe := Int.toTypedLiteral

def TypedLiteral.toInt? (literal: TypedLiteral): Option Int :=
  if literal.datatype == XSD.integer then String.toInt? literal.value else none
instance : Coe TypedLiteral (Option Int) where coe := TypedLiteral.toInt?

theorem stringToIntRoundtrip (x: Int) : (String.toInt? (toString x)) = some x := by sorry

-- theorem stringToIntRoundtrip (x: Int) (i: String.Pos) (h1: y = toString x) (h2: i. ≤ y.length) (h3: i ≥ 0) : (String.toInt? (y.)) = some x := by
--   simp [String.toInt?]
--   induction x
--   case zero => exact rfl
--   case succ x ih => exact ih

-- theorem isNatRoundTrip (x: Nat) : (String.isNat (toString x)) = true := by
--   cases x
--   case zero => exact rfl
--   case succ x ih => simp [toString, Nat.repr, String.isNat]
  -- simp [ toString, Nat.repr ]

  -- simp [toString, Nat.repr, Nat.toDigits, Nat.toDigitsCore, Nat.digitChar, Char.ofNat, String.utf8ByteSize, Nat.isValidChar, String.isNat, Char.isDigit, String.all, String.isEmpty, String.endPos, List.asString, Nat.toDigits, Nat.toDigitsCore, Nat.digitChar, Char.ofNat, String.utf8ByteSize, Nat.isValidChar, String.utf8ByteSize.go, Char.ofNatAux, UInt32.size]
  -- simp
  -- exact rfl


  -- search_proof
  -- simp [String.isNat, Char.isDigit, String.all, String.isEmpty, String.endPos, List.asString, Nat.toDigits, Nat.toDigitsCore, Nat.digitChar, Char.ofNat, String.utf8ByteSize, Nat.isValidChar];
  -- decide
  -- exact ⟨by decide, by decide ⟩
  -- cases toString x <;>
  -- simp [toString ]
  -- simp [toString, String.all ]
  -- simp [String.isEmpty ]
  -- simp [String.all ]
  -- unfold  String.isEmpty
  -- simp [toString, toString ]
  -- simp [String.all, toString ]
  -- unfold  String.all
  -- unfold  toString
  -- cases x <;>  decide
  -- cases toString  x
  -- cases toString x <;>  simp!
  -- constructor <;>  intro
  --  trivial
  -- constructor <;> intro  h
  -- constructor <;>  simp
  --  constructor
  -- cases  x
  -- suggest_tactics
  -- suggest_tactics
  -- induction x
  -- case zero => exact rfl
  -- case succ x ih => simp [toString, Nat.repr]

  -- simp [String.isNat, Char.isDigit];
  -- -- simp [String.isNat, toString, Nat.repr, String.isEmpty, String.endPos, List.asString, Nat.toDigits, Nat.toDigitsCore, Nat.digitChar, Char.ofNat, String.utf8ByteSize, Nat.isValidChar, ]

  -- -- simp [String.isNat, toString, Nat.repr, String.isEmpty, String.endPos, List.asString, Nat.toDigits, Nat.toDigitsCore, Nat.digitChar, Char.ofNat, String.utf8ByteSize, Nat.isValidChar, ]

  -- induction x
  -- case zero => exact rfl
  -- case succ x ih => exact ih

-- theorem stringToNatRoundtrip (x: Nat) : (String.toNat? (toString x)) = some x := by
--   simp [String.toNat?]
--   induction x
--   case zero => exact rfl
--   case succ x ih =>

  -- cases (String.toNat? (toString x)) with
  --   | none => exact rfl
  --   | some y => exact ih
  -- exact (stringToNatRoundtrip x)


theorem intInverse (x: Int) : (TypedLiteral.toInt? (Int.toTypedLiteral x)) = some x := by
  simp [TypedLiteral.toInt?, Int.toTypedLiteral]
  exact (stringToIntRoundtrip x)

def Bool.toTypedLiteral (i: Bool): TypedLiteral := ⟨toString i, XSD.boolean⟩
instance : Coe Bool TypedLiteral where coe := Bool.toTypedLiteral

def TypedLiteral.toBool? (literal: TypedLiteral): Option Bool :=
  if literal.datatype == XSD.boolean then
    match literal.value.toLower with
    | "true" | "1" => some true
    | "false" | "0" => some false
    | _ => none
  else none
instance : Coe TypedLiteral (Option Bool) where coe := TypedLiteral.toBool?

theorem boolInverse (x: Bool) : (TypedLiteral.toBool? (Bool.toTypedLiteral x)) = some x := by cases x <;> exact rfl

instance : Coe Float TypedLiteral where coe s := ⟨toString s, XSD.float⟩
-- FIXME: Implement reverse direction

def String.toTypedLiteral (s: String): TypedLiteral := ⟨s, XSD.string⟩
instance : Coe String TypedLiteral where coe := String.toTypedLiteral

def TypedLiteral.toString? (literal: TypedLiteral): Option String :=
  if literal.datatype == XSD.string then some literal.value else none
instance : Coe TypedLiteral (Option String) where coe := TypedLiteral.toString?

theorem stringInverse (x: String) : (TypedLiteral.toString? (String.toTypedLiteral x)) = some x := by exact rfl

instance : Coe UInt8 TypedLiteral  where coe s := ⟨toString s, XSD.integer⟩
instance : Coe UInt16 TypedLiteral where coe s := ⟨toString s, XSD.integer⟩
instance : Coe UInt32 TypedLiteral where coe s := ⟨toString s, XSD.integer⟩
instance : Coe UInt64 TypedLiteral where coe s := ⟨toString s, XSD.integer⟩
