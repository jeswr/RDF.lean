import RDF.NamedNode
import Lean.Elab.Command




open Lean
open Lean.Parser
open Lean.Elab
open Lean.Elab.Command

-- Define a simple macro that creates a namespace
syntax "create_namespace" ident : command
macro_rules
 | `(create_namespace $ident) => `(
  namespace $ident
    def x := 3
  end $ident
)

#eval `(namespace R def x := 3 end R)

create_namespace Y

namespace Y
  def x := 3
end Y

#eval Y.x

-- @[builtin_command_elab «create_namespace»] def elabCreateNamespace : CommandElab
--   | `(create_namespace $ns:ident) => do
--     let nsName := ns.getId
--     -- Here you can handle the creation of the namespace
--     -- For example, adding the namespace declaration to the environment
--     -- Note: This is a placeholder, actual implementation will vary
--     elabCommand <|← `(namespace $ns:ident end)
--   | _ => throwUnsupportedSyntax

-- -- Example usage
-- create_namespace MyNamespace


















-- macro "&NS" x: String => `(namespace $x end $x)

/- Declares a parser -/
-- syntax (priority := high) "&NS" term "<" term ">" : term

-- /- Declares two expansions/syntax transformers -/
-- macro_rules
--   | `(&NS$x <$val>) => `(namespace $x end $x)
--   -- | `({$x, $xs:term,*}) => `(Set.insert $x {$xs,*})

/- Declares a parser -/
-- macro "&NS" : id => `(namespace $x end $x)


-- macro "MK" "[" declId "]" := `(def $a := mk "hello")

-- macro_rules
--   | `(MK [ $a ]) => `(def $a := mk "hello")
  -- | `(MK [ $y, $ys:term,* ]) => `(def $a := mk $a)


-- syntax (priority := high) "NS" "[" ident "]" "[" term "]" "[" term,+ "]" : term

/- Declares two expansions/syntax transformers -/
-- macro_rules
  -- | `(NS [ $a ] [ $x ] [ $y ]) => `(namespace $a
  --   def mk (e: String) : NamedNode := ⟨"http://www.w3.org/1999/02/22-rdf-syntax-ns#".append e⟩
  -- end $a)
  -- | `(NS [ $a ] [ $x ] [ $y, $ys:term,* ]) => `(def $y := )

-- def ys `($elems: ident) := `(namespace elems end elems)



-- @[builtin_command_elab «ns»] def elabNs : CommandElab := fun stx =>
--   match stx with
--   | `(ns $n) => addNamespace n.getId
--   | _        => throwUnsupportedSyntax
-- @[builtin_command_elab «syntax»] def elabSyntax : CommandElab := fun stx => do


-- syntax (priority := high) "NS" ident "[" command "]" : group

-- macro_rules
--   | `(NS $x [ $y ]) => `(namespace $x $y end $x)

-- /- Declares a parser -/
-- syntax (priority := high) "{" command,+ "}" : [Lean.Parser.Term.structInstFieldAbbrev, Lean.Parser.Term.structInstField]

-- /- Declares two expansions/syntax transformers -/
-- macro_rules
--   | `({$x}) => `($x)
--   | `({$x, $xs:command,*}) => `($x {$xs,*})

-- {def x := 3, def y := 3}

-- namespace XSD
--   def mk (e: String) : NamedNode := ⟨"http://www.w3.org/2001/XMLSchema#".append e⟩
--   def integer := mk "integer"
--   def boolean := mk "boolean"
--   def float := mk "float"
--   def string := mk "string"
-- end XSD

-- syntax (priority := high) "NS" : term

-- syntax (priority := high) "NS" : term
-- macro_rules
--   | `(_) => `(namespace RDF2 def x := 4 end RDF2)
  -- | `(NS) => `(namespace RDF2 def x := 4 end RDF2)

-- def x := NS

-- NS T [
--   def mk (e: String) : NamedNode := ⟨"http://www.w3.org/1999/02/22-rdf-syntax-ns#".append e⟩
-- ]


namespace RDF
  def mk (e: String) : NamedNode := ⟨"http://www.w3.org/1999/02/22-rdf-syntax-ns#".append e⟩
  -- notation "NS" => `(def HTML := mk "HTML")


  -- def HTML := mk "HTML"
  -- def langString := mk "langString"
  -- def PlainLiteral := mk "PlainLiteral"
  -- def type := mk "type"
  -- def Property := mk "Property"
  -- def Statement := mk "Statement"
  -- def subject := mk "subject"
  -- def predicate := mk "predicate"
  -- def Statement := mk "Statement"
  -- def Statement := mk "Statement"
  -- def Statement := mk "Statement"
  -- def Statement := mk "Statement"
end RDF

def x := RDF.mk
