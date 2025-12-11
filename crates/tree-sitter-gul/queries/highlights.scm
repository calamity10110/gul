; GUL v2.1 Syntax Highlighting Query
; Tree-sitter highlights.scm

; ============================================
; Keywords
; ============================================

[
  "main"
  "fn"
  "async"
  "pub"
  "struct"
  "import"
  "const"
  "mut"
  "def"
  "if"
  "elif"
  "else"
  "for"
  "in"
  "while"
  "loop"
  "break"
  "continue"
  "return"
  "await"
  "try"
  "catch"
  "match"
  "extern"
  "and"
  "or"
  "not"
] @keyword

; Boolean literals
[
  "true"
  "false"
] @constant.builtin

; ============================================
; Types
; ============================================

(type (identifier) @type)
(generic_type (identifier) @type)
(struct_definition name: (identifier) @type)

; ============================================
; Functions
; ============================================

(function_definition
  name: (identifier) @function.definition)

(call_expression
  function: (identifier) @function.call)

(call_expression
  function: (member_expression
    property: (identifier) @function.method))

; ============================================
; Variables and Parameters
; ============================================

(parameter
  name: (identifier) @variable.parameter)

(assignment
  name: (identifier) @variable)

(identifier) @variable

; ============================================
; Literals
; ============================================

(number) @number
(string) @string
(comment) @comment

; ============================================
; Operators
; ============================================

[
  "+"
  "-"
  "*"
  "/"
  "%"
  "=="
  "!="
  "<"
  ">"
  "<="
  ">="
  "="
  "->"
] @operator

; ============================================
; Punctuation
; ============================================

; v2.1 Bracket Equivalence - all brackets equal
["(" "[" "{"] @punctuation.bracket
[")" "]" "}"] @punctuation.bracket

[","] @punctuation.delimiter
[":"] @punctuation.delimiter
["."] @punctuation.delimiter

; ============================================
; UI Components (v2.1 ^&^ syntax)
; ============================================

(ui_marker) @ui.marker

(ui_component_name) @ui.component

(ui_prop
  name: (identifier) @ui.property)

; ============================================
; File Type Extensions
; ============================================

; These patterns match file type literals in strings
; .mn - Main file
((string) @type.main
  (#match? @type.main "\\.\\.mn"))

; .def - Definition file
((string) @type.definition
  (#match? @type.definition "\\.\\.def"))

; .fnc - Function file
((string) @type.function
  (#match? @type.function "\\.\\.fnc"))

; .cs - Cross-Script file
((string) @type.crossscript
  (#match? @type.crossscript "\\.\\.cs"))

; .sct - Secret Credential file
((string) @type.secret
  (#match? @type.secret "\\.\\.sct"))

; ============================================
; Special Identifiers
; ============================================

; Self reference
((identifier) @variable.builtin
  (#match? @variable.builtin "^self$"))

; Built-in functions
((identifier) @function.builtin
  (#match? @function.builtin "^(print|len|range|type|str|int|float|list|dict|open|close|read|write)$"))

; ============================================
; Error highlighting
; ============================================

(ERROR) @error
