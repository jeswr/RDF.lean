import RDF.NamedNode

namespace XSD
  def mk (e: String) : NamedNode := ⟨"http://www.w3.org/2001/XMLSchema#".append e⟩
  def integer := mk "integer"
  def boolean := mk "boolean"
  def float := mk "float"
  def string := mk "string"
end XSD
