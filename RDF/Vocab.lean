import RDF.NamedNode

namespace XSD
  def mk (e: String) := #⟨"http://www.w3.org/2001/XMLSchema#" ++ e⟩
  def integer := mk "integer"
  def anyURI := mk "anyURI"
  def dateTime := mk "dateTime"
  def double := mk "double"
  def decimal := mk "decimal"
  def boolean := mk "boolean"
  def float := mk "float"
  def string := mk "string"
end XSD

namespace RDF
  def mk (e: String) := #⟨"http://www.w3.org/1999/02/22-rdf-syntax-ns#" ++ e⟩
  def HTML := mk "HTML"
  def langString := mk "langString"
  def PlainLiteral := mk "PlainLiteral"
  def type := mk "type"
  def Property := mk "Property"
  def Statement := mk "Statement"
  def subject := mk "subject"
  def predicate := mk "predicate"
  def object := mk "object"
  def Bag := mk "Bag"
  def Seq := mk "Seq"
  def Alt := mk "Alt"
  def value := mk "value"
  def List := mk "List"
  def nil := mk "nil"
  def first := mk "first"
  def rest := mk "rest"
  def XMLLiteral := mk "XMLLiteral"
  def JSON := mk "JSON"
  def CompoundLiteral := mk "CompoundLiteral"
  def language := mk "language"
  def direction := mk "direction"
end RDF

namespace RDFS
  def mk (e: String) := #⟨"http://www.w3.org/2000/01/rdf-schema#" ++ e⟩
  def Resource := mk "Resource"
  def Class := mk "Class"
  def subClassOf := mk "subClassOf"
  def subPropertyOf := mk "subPropertyOf"
  def comment := mk "comment"
  def label := mk "label"
  def domain := mk "domain"
  def seeAlso := mk "seeAlso"
  def isDefinedBy := mk "isDefinedBy"
  def Literal := mk "Literal"
  def Container := mk "Container"
  def ContainerMembershipProperty := mk "ContainerMembershipProperty"
  def member := mk "member"
  def Datatype := mk "Datatype"
end RDFS
