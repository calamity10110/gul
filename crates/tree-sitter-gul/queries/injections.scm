; GUL v2.1 Injections Query
; Tree-sitter injections.scm

; Inject language for string content in `code` UI component
(ui_element
  (ui_component_name) @_name
  (#eq? @_name "code")
  (ui_props
    (ui_prop
      name: (identifier) @_prop
      (#eq? @_prop "language")
      value: (string) @injection.language))
  (ui_props
    (ui_prop
      name: (identifier) @_content
      (#eq? @_content "content")
      value: (string) @injection.content)))

; Inject markdown for `markdown` UI component
(ui_element
  (ui_component_name) @_name
  (#eq? @_name "markdown")
  (ui_props
    (ui_prop
      name: (identifier) @_prop
      (#eq? @_prop "content")
      value: (string) @injection.content))
  (#set! injection.language "markdown"))

; Python FFI blocks
(call_expression
  function: (member_expression
    object: (identifier) @_obj
    (#eq? @_obj "python")
    property: (identifier) @_method
    (#eq? @_method "exec"))
  arguments: (argument_list
    (string) @injection.content
    (#set! injection.language "python")))

; JavaScript FFI blocks  
(call_expression
  function: (member_expression
    object: (identifier) @_obj
    (#eq? @_obj "js")
    property: (identifier) @_method
    (#eq? @_method "exec"))
  arguments: (argument_list
    (string) @injection.content
    (#set! injection.language "javascript")))

; SQL queries
(call_expression
  function: (member_expression
    object: (identifier) @_obj
    property: (identifier) @_method
    (#match? @_method "^(query|execute|sql)$"))
  arguments: (argument_list
    (string) @injection.content
    (#set! injection.language "sql")))
