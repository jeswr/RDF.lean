@[extern "add_from_rust"]
opaque addFromRust : UInt32 → UInt32 → Array String

def main : IO Unit :=
  IO.println $ addFromRust 1 2
