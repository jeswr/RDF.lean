import RDF

instance : Coe Int Literal where
  coe s := Literal.DataTyped (toString s) "http://www.w3.org/2001/XMLSchema#integer"

instance : Coe Bool Literal where
  coe s := Literal.DataTyped (toString s) "http://www.w3.org/2001/XMLSchema#boolean"

instance : Coe Float Literal where
  coe s := Literal.DataTyped (toString s) "http://www.w3.org/2001/XMLSchema#float"

instance : Coe String Literal where
  coe s := Literal.DataTyped s "http://www.w3.org/2001/XMLSchema#string"
