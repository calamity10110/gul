; GUL v2.1 Locals Query
; Tree-sitter locals.scm

; ============================================
; Scopes
; ============================================

; Function creates a new scope
(function_definition) @local.scope

; Main block creates a scope
(main_block) @local.scope

; Struct creates a scope
(struct_definition) @local.scope

; Control flow creates scopes
(if_statement) @local.scope
(elif_clause) @local.scope
(else_clause) @local.scope
(for_statement) @local.scope
(while_statement) @local.scope

; ============================================
; Definitions
; ============================================

; Function definition
(function_definition
  name: (identifier) @local.definition.function)

; Parameter definition
(parameter
  name: (identifier) @local.definition.parameter)

; Variable assignment
(assignment
  name: (identifier) @local.definition.var)

; For loop variable
(for_statement
  variable: (identifier) @local.definition.var)

; Struct field
(field_definition
  name: (identifier) @local.definition.field)

; Import
(import_statement
  module: (identifier) @local.definition.import)

(import_list
  (identifier) @local.definition.import)

; ============================================
; References
; ============================================

(identifier) @local.reference
