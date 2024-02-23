
instance : Coe (UInt8 ⊕ UInt16) Int where coe s := String.toInt! (toString s)
#eval ((2 : UInt8) : Int)

def fn (x : UInt8) : Int := String.toInt! (toString x)
#eval fn (2: UInt8)

def fn2 (x : UInt8 ⊕ UInt16) : Int := String.toInt! (toString x)
#eval fn2 (2: UInt8)

-- instance : Coe UInt8 TypedLiteral  where coe s := ⟨toString s, integer⟩
-- instance : Coe UInt16 TypedLiteral where coe s := ⟨toString s, integer⟩
-- instance : Coe UInt32 TypedLiteral where coe s := ⟨toString s, integer⟩
-- instance : Coe UInt64 TypedLiteral where coe s := ⟨toString s, integer⟩


-- #eval ((1 : UInt8) : Int)
