@[extern "add_from_rust"]
opaque addFromRust : String → Array String

def main : IO Unit :=
  IO.println $ addFromRust "Hello from Lean!"
