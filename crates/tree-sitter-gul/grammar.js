/// Tree-sitter GUL v2.1 Grammar
/// 
/// This is the grammar definition for the GUL programming language.
/// See: https://tree-sitter.github.io/tree-sitter/creating-parsers

module.exports = grammar({
  name: 'gul',

  extras: $ => [
    /\s/,
    $.comment,
  ],

  externals: $ => [
    $._indent,
    $._dedent,
    $._newline,
  ],

  rules: {
    // Entry point
    source_file: $ => repeat($._statement),

    // Statements
    _statement: $ => choice(
      $.import_statement,
      $.function_definition,
      $.struct_definition,
      $.main_block,
      $.expression_statement,
      $.assignment,
      $.if_statement,
      $.for_statement,
      $.while_statement,
      $.return_statement,
    ),

    // Import statement
    import_statement: $ => seq(
      'import',
      $.identifier,
      optional($.import_list),
    ),

    import_list: $ => seq(
      $._open_bracket,
      commaSep($.identifier),
      $._close_bracket,
    ),

    // Main block (v2.1 syntax)
    main_block: $ => seq(
      'main',
      ':',
      $._newline,
      $._indent,
      repeat($._statement),
      $._dedent,
    ),

    // Function definition
    function_definition: $ => seq(
      optional('async'),
      optional('pub'),
      'fn',
      $.identifier,
      $._open_bracket,
      optional($.parameter_list),
      $._close_bracket,
      optional($.return_type),
      ':',
      $._newline,
      $._indent,
      repeat($._statement),
      $._dedent,
    ),

    parameter_list: $ => commaSep1($.parameter),

    parameter: $ => seq(
      $.identifier,
      optional(seq(':', $.type)),
    ),

    return_type: $ => seq(
      '->',
      $.type,
    ),

    // Struct definition
    struct_definition: $ => seq(
      optional('pub'),
      'struct',
      $.identifier,
      ':',
      $._newline,
      $._indent,
      repeat($.field_definition),
      $._dedent,
    ),

    field_definition: $ => seq(
      $.identifier,
      ':',
      $.type,
      $._newline,
    ),

    // Control flow
    if_statement: $ => seq(
      'if',
      $.expression,
      ':',
      $._newline,
      $._indent,
      repeat($._statement),
      $._dedent,
      repeat($.elif_clause),
      optional($.else_clause),
    ),

    elif_clause: $ => seq(
      'elif',
      $.expression,
      ':',
      $._newline,
      $._indent,
      repeat($._statement),
      $._dedent,
    ),

    else_clause: $ => seq(
      'else',
      ':',
      $._newline,
      $._indent,
      repeat($._statement),
      $._dedent,
    ),

    for_statement: $ => seq(
      'for',
      $.identifier,
      'in',
      $.expression,
      ':',
      $._newline,
      $._indent,
      repeat($._statement),
      $._dedent,
    ),

    while_statement: $ => seq(
      'while',
      $.expression,
      ':',
      $._newline,
      $._indent,
      repeat($._statement),
      $._dedent,
    ),

    return_statement: $ => seq(
      'return',
      optional($.expression),
      $._newline,
    ),

    // Expressions
    expression_statement: $ => seq(
      $.expression,
      $._newline,
    ),

    expression: $ => choice(
      $.primary,
      $.binary_expression,
      $.unary_expression,
      $.call_expression,
      $.member_expression,
      $.index_expression,
      $.ui_component,
    ),

    primary: $ => choice(
      $.identifier,
      $.number,
      $.string,
      $.boolean,
      $.list,
      $.dict,
      $.grouped_expression,
    ),

    grouped_expression: $ => seq(
      $._open_bracket,
      $.expression,
      $._close_bracket,
    ),

    binary_expression: $ => prec.left(1, seq(
      $.expression,
      choice('+', '-', '*', '/', '%', '==', '!=', '<', '>', '<=', '>=', 'and', 'or'),
      $.expression,
    )),

    unary_expression: $ => prec.right(2, seq(
      choice('not', '-'),
      $.expression,
    )),

    call_expression: $ => prec(3, seq(
      $.expression,
      $._open_bracket,
      optional($.argument_list),
      $._close_bracket,
    )),

    member_expression: $ => prec(4, seq(
      $.expression,
      '.',
      $.identifier,
    )),

    index_expression: $ => prec(4, seq(
      $.expression,
      '[',
      $.expression,
      ']',
    )),

    argument_list: $ => commaSep1($.argument),

    argument: $ => choice(
      $.expression,
      $.keyword_argument,
    ),

    keyword_argument: $ => seq(
      $.identifier,
      ':',
      $.expression,
    ),

    assignment: $ => seq(
      choice('const', 'mut', 'def'),
      $.identifier,
      optional(seq(':', $.type)),
      '=',
      $.expression,
      $._newline,
    ),

    // Collections (v2.1: bracket equivalence)
    list: $ => seq(
      $._open_bracket,
      optional(commaSep($.expression)),
      $._close_bracket,
    ),

    dict: $ => seq(
      $._open_bracket,
      optional(commaSep($.dict_entry)),
      $._close_bracket,
    ),

    dict_entry: $ => seq(
      choice($.identifier, $.string),
      ':',
      $.expression,
    ),

    // UI Components (v2.1 ^&^ syntax)
    ui_component: $ => seq(
      $.ui_marker,
      '[',
      $.ui_element,
      ']',
    ),

    ui_marker: $ => '^&^',

    ui_element: $ => seq(
      $.ui_component_name,
      optional($.ui_props),
      optional($.ui_children),
    ),

    ui_component_name: $ => choice(
      // Basic Input
      'button', 'input', 'textarea', 'checkbox', 'radio', 'select', 'slider', 'toggle',
      // Display
      'label', 'text', 'bigtext', 'paragraph', 'sparkline', 'gauge', 'barchart', 'canvas',
      // Container
      'container', 'block', 'row', 'column', 'grid', 'stack', 'split', 'tabs', 'scrollview', 'popup',
      // Data
      'table', 'list', 'tree', 'calendar', 'chart',
      // Feedback
      'spinner', 'throbber', 'progress', 'toast', 'alert', 'badge',
      // Navigation
      'menu', 'menubar', 'contextmenu', 'breadcrumb', 'pagination',
      // Media
      'image', 'video', 'audio', 'markdown', 'code',
      // Prompt
      'prompt',
    ),

    ui_props: $ => seq(
      '{',
      optional(commaSep($.ui_prop)),
      '}',
    ),

    ui_prop: $ => seq(
      $.identifier,
      ':',
      $.expression,
    ),

    ui_children: $ => seq(
      '[',
      repeat($.ui_element),
      ']',
    ),

    // Types
    type: $ => choice(
      $.identifier,
      $.generic_type,
      $.optional_type,
    ),

    generic_type: $ => seq(
      $.identifier,
      '<',
      commaSep1($.type),
      '>',
    ),

    optional_type: $ => seq(
      $.type,
      '?',
    ),

    // Brackets (v2.1: all equivalent)
    _open_bracket: $ => choice('(', '[', '{'),
    _close_bracket: $ => choice(')', ']', '}'),

    // Literals
    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_]*/,

    number: $ => choice(
      /\d+/,
      /\d+\.\d+/,
      /0x[0-9a-fA-F]+/,
      /0b[01]+/,
      /\d+e[+-]?\d+/,
    ),

    string: $ => choice(
      seq('"', /[^"]*/, '"'),
      seq("'", /[^']*/, "'"),
      seq('f"', /[^"]*/, '"'),  // f-string
    ),

    boolean: $ => choice('true', 'false'),

    // Comments
    comment: $ => /#.*/,
  },
});

// Helper functions
function commaSep(rule) {
  return optional(commaSep1(rule));
}

function commaSep1(rule) {
  return seq(rule, repeat(seq(',', rule)));
}
