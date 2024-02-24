import RDF.Parse
import RDF.Serialize

def main : IO Unit := do
  let triples ← IO.ofExcept $ RDFParse "<http://example.org/a> <http://example.org/b> </c>, \"c\", \"c\", true, \"hello world!\"@en ." "text/turtle" "http://example.org"
  IO.println triples
  IO.println $ RDFSerialise triples "text/turtle"
