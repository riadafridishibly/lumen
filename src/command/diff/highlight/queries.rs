pub const TS_HIGHLIGHTS: &str = r#"
(comment) @comment
(string) @string
(template_string) @string
(number) @number
(true) @constant.builtin
(false) @constant.builtin
(null) @constant.builtin
(undefined) @constant.builtin
(regex) @string.special

["const" "let" "var" "function" "class" "interface" "type" "enum" "namespace" "module" "declare" "implements" "extends" "public" "private" "protected" "readonly" "static" "abstract" "async" "await" "return" "if" "else" "for" "while" "do" "switch" "case" "default" "break" "continue" "try" "catch" "finally" "throw" "new" "delete" "typeof" "instanceof" "in" "of" "as" "is" "import" "export" "from" "default" "void"] @keyword

(type_identifier) @type
(predefined_type) @type.builtin

(function_declaration name: (identifier) @function)
(method_definition name: (property_identifier) @function.method)
(call_expression function: (identifier) @function)
(call_expression function: (member_expression property: (property_identifier) @function.method))
(arrow_function) @function

(property_identifier) @property
(shorthand_property_identifier) @property
(shorthand_property_identifier_pattern) @property

["(" ")" "[" "]" "{" "}"] @punctuation.bracket
["." "," ";" ":"] @punctuation.delimiter
"#;

pub const TSX_HIGHLIGHTS: &str = r#"
(comment) @comment
(string) @string
(template_string) @string
(number) @number
(true) @constant.builtin
(false) @constant.builtin
(null) @constant.builtin
(undefined) @constant.builtin
(regex) @string.special

["const" "let" "var" "function" "class" "interface" "type" "enum" "namespace" "module" "declare" "implements" "extends" "public" "private" "protected" "readonly" "static" "abstract" "async" "await" "return" "if" "else" "for" "while" "do" "switch" "case" "default" "break" "continue" "try" "catch" "finally" "throw" "new" "delete" "typeof" "instanceof" "in" "of" "as" "is" "import" "export" "from" "default" "void"] @keyword

(type_identifier) @type
(predefined_type) @type.builtin

(function_declaration name: (identifier) @function)
(method_definition name: (property_identifier) @function.method)
(call_expression function: (identifier) @function)
(call_expression function: (member_expression property: (property_identifier) @function.method))
(arrow_function) @function

(property_identifier) @property
(shorthand_property_identifier) @property
(shorthand_property_identifier_pattern) @property

(jsx_element open_tag: (jsx_opening_element name: (identifier) @tag))
(jsx_element close_tag: (jsx_closing_element name: (identifier) @tag))
(jsx_self_closing_element name: (identifier) @tag)
(jsx_attribute (property_identifier) @attribute)

["(" ")" "[" "]" "{" "}"] @punctuation.bracket
["." "," ";" ":"] @punctuation.delimiter
"#;

pub const JS_HIGHLIGHTS: &str = r#"
(comment) @comment
(string) @string
(template_string) @string
(number) @number
(true) @constant.builtin
(false) @constant.builtin
(null) @constant.builtin
(undefined) @constant.builtin
(regex) @string.special

["const" "let" "var" "function" "class" "extends" "async" "await" "return" "if" "else" "for" "while" "do" "switch" "case" "default" "break" "continue" "try" "catch" "finally" "throw" "new" "delete" "typeof" "instanceof" "in" "of" "import" "export" "from" "default" "void"] @keyword

(function_declaration name: (identifier) @function)
(method_definition name: (property_identifier) @function.method)
(call_expression function: (identifier) @function)
(call_expression function: (member_expression property: (property_identifier) @function.method))
(arrow_function) @function

(property_identifier) @property
(shorthand_property_identifier) @property

(jsx_element open_tag: (jsx_opening_element name: (identifier) @tag))
(jsx_element close_tag: (jsx_closing_element name: (identifier) @tag))
(jsx_self_closing_element name: (identifier) @tag)
(jsx_attribute (property_identifier) @attribute)

["(" ")" "[" "]" "{" "}"] @punctuation.bracket
["." "," ";" ":"] @punctuation.delimiter
"#;

pub const RUST_HIGHLIGHTS: &str = r#"
; Comments
; Regular comments (line_comment and block_comment capture the entire comment)
(line_comment) @comment
(block_comment) @comment
; Doc comment parts need explicit captures to prevent operator conflicts
; The "/" in "///" and "!" in "//!" would otherwise match operator patterns
(outer_doc_comment_marker) @comment
(inner_doc_comment_marker) @comment
(doc_comment) @comment

; Strings and literals
(string_literal) @string
(raw_string_literal) @string
(char_literal) @string
(integer_literal) @number
(float_literal) @number
(boolean_literal) @constant.builtin

; Types
(type_identifier) @type
(primitive_type) @type.builtin

; Functions
(function_item (identifier) @function)
(function_signature_item (identifier) @function)
(call_expression function: (identifier) @function)
(call_expression function: (field_expression field: (field_identifier) @function.method))
(call_expression function: (scoped_identifier name: (identifier) @function))
(generic_function function: (identifier) @function)
(generic_function function: (scoped_identifier name: (identifier) @function))

; Macros
(macro_invocation macro: (identifier) @function.macro "!" @function.macro)
(macro_definition "macro_rules!" @function.macro)

; Fields and properties
(field_identifier) @variable.member
(shorthand_field_identifier) @variable.member

; Labels and lifetimes
(lifetime (identifier) @label)

; Parameters
(parameter (identifier) @variable.parameter)

; Modules
(mod_item name: (identifier) @module)
(scoped_identifier path: (identifier) @module)

; Self, crate, and special
(self) @variable.builtin
(crate) @keyword
(super) @keyword
(mutable_specifier) @keyword

; Keywords
"as" @keyword
"async" @keyword
"await" @keyword
"break" @keyword
"const" @keyword
"continue" @keyword
"dyn" @keyword
"else" @keyword
"enum" @keyword
"extern" @keyword
"fn" @keyword
"for" @keyword
"if" @keyword
"impl" @keyword
"in" @keyword
"let" @keyword
"loop" @keyword
"match" @keyword
"mod" @keyword
"move" @keyword
"pub" @keyword
"ref" @keyword
"return" @keyword
"static" @keyword
"struct" @keyword
"trait" @keyword
"type" @keyword
"unsafe" @keyword
"use" @keyword
"where" @keyword
"while" @keyword

; Operators
; Note: "/" and "!" are not matched globally to avoid conflicts with doc comments
; They are highlighted via binary_expression and unary_expression patterns below
"*" @operator
"&" @operator
"=" @operator
"+" @operator
"-" @operator
"%" @operator
"<" @operator
">" @operator
"==" @operator
"!=" @operator
"<=" @operator
">=" @operator
"&&" @operator
"||" @operator
"+=" @operator
"-=" @operator
"*=" @operator
"/=" @operator
".." @operator
"..=" @operator
"=>" @operator
"->" @operator
"?" @operator

; Division and negation operators in specific contexts
(binary_expression "/" @operator)
(unary_expression "!" @operator)

; Punctuation
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"::" @punctuation.delimiter
":" @punctuation.delimiter
"#;

pub const RUBY_HIGHLIGHTS: &str = r#"
; Comments
(comment) @comment

; Strings and symbols
(string) @string
(bare_string) @string
(subshell) @string
(heredoc_body) @string
(heredoc_beginning) @string
(simple_symbol) @string.special
(delimited_symbol) @string.special
(hash_key_symbol) @string.special
(bare_symbol) @string.special
(regex) @string.special

; Literals
(integer) @number
(float) @number
(nil) @constant.builtin
(true) @constant.builtin
(false) @constant.builtin

; Constants
(constant) @type

; Variables
(instance_variable) @property
(class_variable) @property
(global_variable) @variable.builtin
(self) @variable.builtin
(super) @variable.builtin

; Parameters
(block_parameter (identifier) @variable.parameter)
(block_parameters (identifier) @variable.parameter)
(method_parameters (identifier) @variable.parameter)
(keyword_parameter name: (identifier) @variable.parameter)
(optional_parameter name: (identifier) @variable.parameter)
(splat_parameter (identifier) @variable.parameter)
(hash_splat_parameter (identifier) @variable.parameter)

; Functions and methods
(method name: (identifier) @function)
(method name: (constant) @function)
(singleton_method name: (identifier) @function)
(call method: (identifier) @function.method)
(call method: (constant) @function.method)

; Keywords
"alias" @keyword
"and" @keyword
"begin" @keyword
"break" @keyword
"case" @keyword
"class" @keyword
"def" @keyword
"do" @keyword
"else" @keyword
"elsif" @keyword
"end" @keyword
"ensure" @keyword
"for" @keyword
"if" @keyword
"in" @keyword
"module" @keyword
"next" @keyword
"or" @keyword
"rescue" @keyword
"retry" @keyword
"return" @keyword
"then" @keyword
"unless" @keyword
"until" @keyword
"when" @keyword
"while" @keyword
"yield" @keyword
"not" @keyword
"defined?" @keyword

; Operators
"=" @operator
"=>" @operator
"->" @operator
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
"**" @operator
"==" @operator
"!=" @operator
"<" @operator
">" @operator
"<=" @operator
">=" @operator
"<=>" @operator
"&&" @operator
"||" @operator
"!" @operator
"&" @operator
"|" @operator
"^" @operator
"~" @operator
"<<" @operator
">>" @operator
".." @operator
"..." @operator

; Punctuation
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"," @punctuation.delimiter
";" @punctuation.delimiter
"." @punctuation.delimiter
":" @punctuation.delimiter
"::" @punctuation.delimiter
"#;

pub const JSON_HIGHLIGHTS: &str = r#"
(string) @string
(number) @number
(true) @constant.builtin
(false) @constant.builtin
(null) @constant.builtin
(pair key: (string) @property)

"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
":" @punctuation.delimiter
"," @punctuation.delimiter
"#;

pub const PYTHON_HIGHLIGHTS: &str = r#"
; Comments and strings
(comment) @comment
(string) @string
(escape_sequence) @string.special

; Literals
(integer) @number
(float) @number
(none) @constant.builtin
(true) @constant.builtin
(false) @constant.builtin

; Types and attributes
(type (identifier) @type)
(attribute attribute: (identifier) @property)

; Functions
(function_definition name: (identifier) @function)
(call function: (identifier) @function)
(call function: (attribute attribute: (identifier) @function.method))
(decorator) @function
(decorator (identifier) @function)

; Keywords
"as" @keyword
"assert" @keyword
"async" @keyword
"await" @keyword
"break" @keyword
"class" @keyword
"continue" @keyword
"def" @keyword
"del" @keyword
"elif" @keyword
"else" @keyword
"except" @keyword
"finally" @keyword
"for" @keyword
"from" @keyword
"global" @keyword
"if" @keyword
"import" @keyword
"lambda" @keyword
"nonlocal" @keyword
"pass" @keyword
"raise" @keyword
"return" @keyword
"try" @keyword
"while" @keyword
"with" @keyword
"yield" @keyword
"match" @keyword
"case" @keyword
"and" @operator
"or" @operator
"not" @operator
"in" @operator
"is" @operator

"#;

pub const GO_HIGHLIGHTS: &str = r#"
; Comments and strings
(comment) @comment
(interpreted_string_literal) @string
(raw_string_literal) @string
(rune_literal) @string

; Literals
(int_literal) @number
(float_literal) @number
(true) @constant.builtin
(false) @constant.builtin
(nil) @constant.builtin

; Types
(type_identifier) @type
(type_spec name: (type_identifier) @type)

; Functions
(function_declaration name: (identifier) @function)
(method_declaration name: (field_identifier) @function.method)
(call_expression function: (identifier) @function)
(call_expression function: (selector_expression field: (field_identifier) @function.method))

; Fields
(field_identifier) @property

; Package
(package_identifier) @module

; Keywords
"break" @keyword
"case" @keyword
"chan" @keyword
"const" @keyword
"continue" @keyword
"default" @keyword
"defer" @keyword
"else" @keyword
"fallthrough" @keyword
"for" @keyword
"func" @keyword
"go" @keyword
"goto" @keyword
"if" @keyword
"import" @keyword
"interface" @keyword
"map" @keyword
"package" @keyword
"range" @keyword
"return" @keyword
"select" @keyword
"struct" @keyword
"switch" @keyword
"type" @keyword
"var" @keyword

; Operators
"=" @operator
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
"!" @operator
"<" @operator
">" @operator
"&" @operator
"|" @operator
"^" @operator
":=" @operator
"==" @operator
"!=" @operator
"<=" @operator
">=" @operator
"&&" @operator
"||" @operator
"++" @operator
"--" @operator
"+=" @operator
"-=" @operator
"<-" @operator

; Punctuation
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"." @punctuation.delimiter
"," @punctuation.delimiter
";" @punctuation.delimiter
":" @punctuation.delimiter
"#;

pub const CSS_HIGHLIGHTS: &str = r#"
(comment) @comment
(string_value) @string
(integer_value) @number
(float_value) @number
(color_value) @constant
(property_name) @property
(tag_name) @tag
(class_name) @type
(id_name) @constant
(at_keyword) @keyword
"#;

pub const HTML_HIGHLIGHTS: &str = r#"
(comment) @comment
(quoted_attribute_value) @string
(tag_name) @tag
(attribute_name) @attribute
"#;

pub const TOML_HIGHLIGHTS: &str = r#"
(comment) @comment
(string) @string
(integer) @number
(float) @number
(boolean) @constant.builtin
(bare_key) @property
(dotted_key) @property
"#;

pub const BASH_HIGHLIGHTS: &str = r#"
(comment) @comment
(string) @string
(raw_string) @string
(number) @number
(command_name) @function
(variable_name) @variable
"#;

pub const MD_HIGHLIGHTS: &str = r#"
(atx_heading) @keyword
(setext_heading) @keyword
(thematic_break) @punctuation.delimiter
(fenced_code_block) @string
(indented_code_block) @string
(block_quote) @comment
(list_marker_plus) @punctuation
(list_marker_minus) @punctuation
(list_marker_star) @punctuation
(list_marker_dot) @punctuation
(list_marker_parenthesis) @punctuation
(link_destination) @string
(link_title) @string
"#;

pub const CSHARP_HIGHLIGHTS: &str = r#"
; Comments
(comment) @comment

; Strings and literals
(string_literal) @string
(verbatim_string_literal) @string
(interpolated_string_expression) @string
(character_literal) @string
(integer_literal) @number
(real_literal) @number
(boolean_literal) @constant.builtin
(null_literal) @constant.builtin

; Types - C# doesn't have type_identifier, types are represented by identifier in context
; or predefined_type for built-in types
(predefined_type) @type.builtin

; Namespaces and usings
(namespace_declaration name: (qualified_name) @module)
(namespace_declaration name: (identifier) @module)
(using_directive (identifier) @module)
(using_directive (qualified_name) @module)

; Classes, structs, interfaces, enums
(class_declaration name: (identifier) @type)
(struct_declaration name: (identifier) @type)
(interface_declaration name: (identifier) @type)
(enum_declaration name: (identifier) @type)
(record_declaration name: (identifier) @type)

; Methods and functions
(method_declaration name: (identifier) @function)
(local_function_statement name: (identifier) @function)
(constructor_declaration name: (identifier) @function)
(destructor_declaration name: (identifier) @function)
(invocation_expression function: (identifier) @function)
(invocation_expression function: (member_access_expression name: (identifier) @function.method))

; Properties and fields
(property_declaration name: (identifier) @property)
(field_declaration (variable_declaration (variable_declarator (identifier) @variable.member)))

; Parameters
(parameter name: (identifier) @variable.parameter)

; Attributes
(attribute) @attribute
(attribute_list) @attribute

; Keywords
"abstract" @keyword
"as" @keyword
"async" @keyword
"await" @keyword
"base" @keyword
"break" @keyword
"case" @keyword
"catch" @keyword
"checked" @keyword
"class" @keyword
"const" @keyword
"continue" @keyword
"default" @keyword
"delegate" @keyword
"do" @keyword
"else" @keyword
"enum" @keyword
"event" @keyword
"explicit" @keyword
"extern" @keyword
"finally" @keyword
"fixed" @keyword
"for" @keyword
"foreach" @keyword
"goto" @keyword
"if" @keyword
"implicit" @keyword
"in" @keyword
"interface" @keyword
"internal" @keyword
"is" @keyword
"lock" @keyword
"namespace" @keyword
"new" @keyword
"operator" @keyword
"out" @keyword
"override" @keyword
"params" @keyword
"private" @keyword
"protected" @keyword
"public" @keyword
"readonly" @keyword
"record" @keyword
"ref" @keyword
"return" @keyword
"sealed" @keyword
"sizeof" @keyword
"stackalloc" @keyword
"static" @keyword
"struct" @keyword
"switch" @keyword
"this" @keyword
"throw" @keyword
"try" @keyword
"typeof" @keyword
"unchecked" @keyword
"unsafe" @keyword
"using" @keyword
"var" @keyword
"virtual" @keyword
"volatile" @keyword
"when" @keyword
"where" @keyword
"while" @keyword
"yield" @keyword
"get" @keyword
"set" @keyword
"init" @keyword
"add" @keyword
"remove" @keyword
"partial" @keyword
"global" @keyword
"required" @keyword
"file" @keyword
"scoped" @keyword

; Operators
"=" @operator
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
"!" @operator
"<" @operator
">" @operator
"&" @operator
"|" @operator
"^" @operator
"~" @operator
"?" @operator
"==" @operator
"!=" @operator
"<=" @operator
">=" @operator
"&&" @operator
"||" @operator
"+=" @operator
"-=" @operator
"*=" @operator
"/=" @operator
"??" @operator
"??=" @operator
"=>" @operator
"->" @operator
"++" @operator
"--" @operator

; Punctuation
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"." @punctuation.delimiter
"," @punctuation.delimiter
";" @punctuation.delimiter
":" @punctuation.delimiter
"#;

/*
Portions of `ZIG_HIGHLIGHTS` are adapted from tree-sitter-zig 1.1.2
`queries/highlights.scm` and remapped to Lumen's supported capture names.

Upstream project: https://github.com/tree-sitter-grammars/tree-sitter-zig

MIT License

Copyright (c) 2024 Amaan Qureshi <amaanq12@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
pub const ZIG_HIGHLIGHTS: &str = r#"
; Variables
(identifier) @variable

; Parameters
(parameter
  name: (identifier) @variable.parameter)

((payload
  (identifier) @variable.parameter)
  (#set! "priority" 110))

; Types
(parameter
  type: (identifier) @type)

((identifier) @type
  (#match? @type "^[A-Z_][a-zA-Z0-9_]*$"))

(variable_declaration
  (identifier) @type
  "="
  [
    (struct_declaration)
    (enum_declaration)
    (union_declaration)
    (opaque_declaration)
  ])

[
  (builtin_type)
  "anyframe"
] @type.builtin

; Constants
((identifier) @constant
  (#match? @constant "^[A-Z][A-Z_0-9]+$"))

[
  "null"
  "unreachable"
  "undefined"
] @constant.builtin

(field_expression
  .
  member: (identifier) @constant)

(enum_declaration
  (container_field
    type: (identifier) @constant))

; Labels
(block_label (identifier) @label)

(break_label (identifier) @label)

; Fields
(field_initializer
  .
  (identifier) @variable.member)

(field_expression
  (_)
  member: (identifier) @variable.member)

(container_field
  name: (identifier) @variable.member)

(initializer_list
  (assignment_expression
      left: (field_expression
              .
              member: (identifier) @variable.member)))

; Functions
(builtin_identifier) @function.builtin

(call_expression
  function: (identifier) @function)

(call_expression
  function: (field_expression
    member: (identifier) @function.method))

(function_declaration
  name: (identifier) @function)

; Modules
(variable_declaration
  (identifier) @module
  (builtin_function
    (builtin_identifier) @keyword
    (#any-of? @keyword "@import" "@cImport")))

; Builtins
[
  "c"
  "..."
] @variable.builtin

((identifier) @variable.builtin
  (#eq? @variable.builtin "_"))

(calling_convention
  (identifier) @variable.builtin)

; Keywords
[
  "asm"
  "defer"
  "errdefer"
  "test"
  "error"
  "const"
  "var"
  "struct"
  "union"
  "enum"
  "opaque"
  "async"
  "await"
  "suspend"
  "nosuspend"
  "resume"
  "fn"
  "and"
  "or"
  "orelse"
  "return"
  "if"
  "else"
  "switch"
  "for"
  "while"
  "break"
  "continue"
  "usingnamespace"
  "export"
  "try"
  "catch"
  "volatile"
  "allowzero"
  "noalias"
  "addrspace"
  "align"
  "callconv"
  "linksection"
  "pub"
  "inline"
  "noinline"
  "extern"
  "comptime"
  "packed"
  "threadlocal"
] @keyword

; Operators
[
  "="
  "*="
  "*%="
  "*|="
  "/="
  "%="
  "+="
  "+%="
  "+|="
  "-="
  "-%="
  "-|="
  "<<="
  "<<|="
  ">>="
  "&="
  "^="
  "|="
  "!"
  "~"
  "-"
  "-%"
  "&"
  "=="
  "!="
  ">"
  ">="
  "<="
  "<"
  "^"
  "|"
  "<<"
  ">>"
  "<<|"
  "+"
  "++"
  "+%"
  "+|"
  "*"
  "/"
  "%"
  "**"
  "*%"
  "*|"
  "||"
  ".*"
  ".?"
  "?"
  ".."
] @operator

; Literals
(character) @string.special

([
  (string)
  (multiline_string)
] @string
  (#set! "priority" 95))

(integer) @number

(float) @number

(boolean) @constant.builtin

(escape_sequence) @string.special

; Punctuation
[
  "["
  "]"
  "("
  ")"
  "{"
  "}"
] @punctuation.bracket

[
  ";"
  "."
  ","
  ":"
  "=>"
  "->"
] @punctuation.delimiter

(payload "|" @punctuation.bracket)

; Comments
(comment) @comment
"#;

pub const C_HIGHLIGHTS: &str = r##"
; Comments
(comment) @comment

; Strings and literals
(string_literal) @string
(system_lib_string) @string
(char_literal) @string
(number_literal) @number
(true) @constant.builtin
(false) @constant.builtin
(null) @constant.builtin
((identifier) @constant
 (#match? @constant "^[A-Z][A-Z0-9_]*$"))

; Types
(type_identifier) @type
(primitive_type) @type.builtin
(sized_type_specifier) @type.builtin

; Fields
(field_identifier) @variable.member

; Labels
(statement_identifier) @label

; Functions
(call_expression function: (identifier) @function)
(call_expression function: (field_expression field: (field_identifier) @function.method))
(function_declarator declarator: (identifier) @function)
(function_declarator declarator: (field_identifier) @function.method)
(preproc_function_def name: (identifier) @function.macro)

; Preprocessor directives
"#define" @function.macro
"#include" @function.macro
"#if" @function.macro
"#ifdef" @function.macro
"#ifndef" @function.macro
"#else" @function.macro
"#elif" @function.macro
"#endif" @function.macro
(preproc_directive) @function.macro

; Keywords
"break" @keyword
"case" @keyword
"const" @keyword
"continue" @keyword
"default" @keyword
"do" @keyword
"else" @keyword
"enum" @keyword
"extern" @keyword
"for" @keyword
"goto" @keyword
"if" @keyword
"inline" @keyword
"register" @keyword
"restrict" @keyword
"return" @keyword
"sizeof" @keyword
"static" @keyword
"struct" @keyword
"switch" @keyword
"typedef" @keyword
"union" @keyword
"volatile" @keyword
"while" @keyword

; Operators
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
"=" @operator
"==" @operator
"!=" @operator
"<" @operator
">" @operator
"<=" @operator
">=" @operator
"&&" @operator
"||" @operator
"!" @operator
"&" @operator
"|" @operator
"^" @operator
"~" @operator
"<<" @operator
">>" @operator
"+=" @operator
"-=" @operator
"*=" @operator
"/=" @operator
"++" @operator
"--" @operator
"->" @operator
"?" @operator

; Punctuation
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"." @punctuation.delimiter
"," @punctuation.delimiter
";" @punctuation.delimiter
":" @punctuation.delimiter
"##;

pub const CPP_HIGHLIGHTS: &str = r##"
; Comments
(comment) @comment

; Strings and literals
(string_literal) @string
(raw_string_literal) @string
(system_lib_string) @string
(char_literal) @string
(number_literal) @number
(true) @constant.builtin
(false) @constant.builtin
(null) @constant.builtin
((identifier) @constant
 (#match? @constant "^[A-Z][A-Z0-9_]*$"))

; Types
(type_identifier) @type
(primitive_type) @type.builtin
(sized_type_specifier) @type.builtin
(auto) @type.builtin
((namespace_identifier) @type
 (#match? @type "^[A-Z]"))
(namespace_identifier) @module

; Fields
(field_identifier) @variable.member

; Special variables
(this) @variable.builtin

; Functions
(call_expression function: (identifier) @function)
(call_expression function: (field_expression field: (field_identifier) @function.method))
(call_expression function: (qualified_identifier name: (identifier) @function))
(template_function name: (identifier) @function)
(template_method name: (field_identifier) @function.method)
(function_declarator declarator: (identifier) @function)
(function_declarator declarator: (field_identifier) @function.method)
(function_declarator declarator: (qualified_identifier name: (identifier) @function))
(preproc_function_def name: (identifier) @function.macro)

; Preprocessor directives
"#define" @function.macro
"#include" @function.macro
"#if" @function.macro
"#ifdef" @function.macro
"#ifndef" @function.macro
"#else" @function.macro
"#elif" @function.macro
"#endif" @function.macro
(preproc_directive) @function.macro

; Keywords
"break" @keyword
"case" @keyword
"catch" @keyword
"class" @keyword
"co_await" @keyword
"co_return" @keyword
"co_yield" @keyword
"concept" @keyword
"const" @keyword
"constexpr" @keyword
"consteval" @keyword
"constinit" @keyword
"continue" @keyword
"decltype" @keyword
"default" @keyword
"delete" @keyword
"do" @keyword
"else" @keyword
"enum" @keyword
"explicit" @keyword
"extern" @keyword
"final" @keyword
"for" @keyword
"friend" @keyword
"goto" @keyword
"if" @keyword
"inline" @keyword
"mutable" @keyword
"namespace" @keyword
"new" @keyword
"noexcept" @keyword
"operator" @keyword
"override" @keyword
"private" @keyword
"protected" @keyword
"public" @keyword
"register" @keyword
"requires" @keyword
"return" @keyword
"sizeof" @keyword
"static" @keyword
"static_assert" @keyword
"struct" @keyword
"switch" @keyword
"template" @keyword
"throw" @keyword
"try" @keyword
"typedef" @keyword
"typename" @keyword
"union" @keyword
"using" @keyword
"virtual" @keyword
"volatile" @keyword
"while" @keyword

; Operators
"+" @operator
"-" @operator
"*" @operator
"/" @operator
"%" @operator
"=" @operator
"==" @operator
"!=" @operator
"<" @operator
">" @operator
"<=" @operator
">=" @operator
"&&" @operator
"||" @operator
"!" @operator
"&" @operator
"|" @operator
"^" @operator
"~" @operator
"<<" @operator
">>" @operator
"+=" @operator
"-=" @operator
"*=" @operator
"/=" @operator
"++" @operator
"--" @operator
"->" @operator
"?" @operator

; Punctuation
"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket
"::" @punctuation.delimiter
"." @punctuation.delimiter
"," @punctuation.delimiter
";" @punctuation.delimiter
":" @punctuation.delimiter
"##;

// Adapted from tree-sitter-sequel queries/highlights.scm (MIT), remapped onto
// HIGHLIGHT_NAMES. Numeric literals need the #match? refinements because the
// grammar lumps every scalar into (literal).
pub const SQL_HIGHLIGHTS: &str = r##"
(object_reference
  name: (identifier) @type)

(object_reference
  schema: (identifier) @module)

(object_reference
  database: (identifier) @module)

(column_definition
  name: (identifier) @variable.member)

(invocation
  (object_reference
    name: (identifier) @function))

[
  (keyword_gist)
  (keyword_btree)
  (keyword_hash)
  (keyword_spgist)
  (keyword_gin)
  (keyword_brin)
  (keyword_array)
  (keyword_object_id)
] @function

(relation
  alias: (identifier) @variable)

(field
  name: (identifier) @variable.member)

(term
  alias: (identifier) @variable)

((term
   value: (cast
    name: (keyword_cast) @function
    parameter: [(literal)]?)))

(literal) @string
(comment) @comment
(marginalia) @comment

((literal) @number
  (#match? @number "^[-+]?[0-9]+$"))

((literal) @number
  (#match? @number "^[-+]?[0-9]*[.][0-9]+([eE][-+]?[0-9]+)?$"))

(parameter) @variable.parameter

[
 (keyword_true)
 (keyword_false)
 (keyword_null)
] @constant.builtin

[
 (keyword_asc)
 (keyword_desc)
 (keyword_terminated)
 (keyword_escaped)
 (keyword_unsigned)
 (keyword_nulls)
 (keyword_last)
 (keyword_delimited)
 (keyword_replication)
 (keyword_auto_increment)
 (keyword_default)
 (keyword_collate)
 (keyword_concurrently)
 (keyword_engine)
 (keyword_always)
 (keyword_generated)
 (keyword_preceding)
 (keyword_following)
 (keyword_first)
 (keyword_current_timestamp)
 (keyword_immutable)
 (keyword_atomic)
 (keyword_parallel)
 (keyword_leakproof)
 (keyword_safe)
 (keyword_cost)
 (keyword_strict)
] @attribute

[
 (keyword_materialized)
 (keyword_recursive)
 (keyword_temp)
 (keyword_temporary)
 (keyword_unlogged)
 (keyword_external)
 (keyword_parquet)
 (keyword_csv)
 (keyword_rcfile)
 (keyword_textfile)
 (keyword_orc)
 (keyword_avro)
 (keyword_jsonfile)
 (keyword_sequencefile)
 (keyword_volatile)
] @keyword

[
 (keyword_case)
 (keyword_when)
 (keyword_then)
 (keyword_else)
] @keyword

[
  (keyword_select)
  (keyword_from)
  (keyword_where)
  (keyword_index)
  (keyword_join)
  (keyword_primary)
  (keyword_delete)
  (keyword_create)
  (keyword_show)
  (keyword_unload)
  (keyword_insert)
  (keyword_merge)
  (keyword_distinct)
  (keyword_replace)
  (keyword_update)
  (keyword_into)
  (keyword_overwrite)
  (keyword_matched)
  (keyword_values)
  (keyword_value)
  (keyword_attribute)
  (keyword_set)
  (keyword_left)
  (keyword_right)
  (keyword_outer)
  (keyword_inner)
  (keyword_full)
  (keyword_order)
  (keyword_partition)
  (keyword_group)
  (keyword_with)
  (keyword_without)
  (keyword_as)
  (keyword_having)
  (keyword_limit)
  (keyword_offset)
  (keyword_table)
  (keyword_tables)
  (keyword_key)
  (keyword_references)
  (keyword_foreign)
  (keyword_constraint)
  (keyword_force)
  (keyword_use)
  (keyword_for)
  (keyword_if)
  (keyword_exists)
  (keyword_column)
  (keyword_columns)
  (keyword_cross)
  (keyword_lateral)
  (keyword_natural)
  (keyword_alter)
  (keyword_drop)
  (keyword_add)
  (keyword_view)
  (keyword_end)
  (keyword_is)
  (keyword_using)
  (keyword_between)
  (keyword_window)
  (keyword_no)
  (keyword_data)
  (keyword_type)
  (keyword_rename)
  (keyword_to)
  (keyword_schema)
  (keyword_owner)
  (keyword_authorization)
  (keyword_all)
  (keyword_any)
  (keyword_some)
  (keyword_returning)
  (keyword_begin)
  (keyword_commit)
  (keyword_rollback)
  (keyword_transaction)
  (keyword_only)
  (keyword_like)
  (keyword_similar)
  (keyword_over)
  (keyword_change)
  (keyword_modify)
  (keyword_after)
  (keyword_before)
  (keyword_range)
  (keyword_rows)
  (keyword_groups)
  (keyword_exclude)
  (keyword_current)
  (keyword_ties)
  (keyword_others)
  (keyword_zerofill)
  (keyword_format)
  (keyword_fields)
  (keyword_row)
  (keyword_sort)
  (keyword_compute)
  (keyword_comment)
  (keyword_location)
  (keyword_cached)
  (keyword_uncached)
  (keyword_lines)
  (keyword_stored)
  (keyword_virtual)
  (keyword_partitioned)
  (keyword_analyze)
  (keyword_explain)
  (keyword_verbose)
  (keyword_truncate)
  (keyword_rewrite)
  (keyword_optimize)
  (keyword_vacuum)
  (keyword_cache)
  (keyword_language)
  (keyword_called)
  (keyword_conflict)
  (keyword_declare)
  (keyword_filter)
  (keyword_function)
  (keyword_input)
  (keyword_name)
  (keyword_oid)
  (keyword_oids)
  (keyword_precision)
  (keyword_regclass)
  (keyword_regnamespace)
  (keyword_regproc)
  (keyword_regtype)
  (keyword_restricted)
  (keyword_return)
  (keyword_returns)
  (keyword_separator)
  (keyword_setof)
  (keyword_stable)
  (keyword_support)
  (keyword_tblproperties)
  (keyword_trigger)
  (keyword_unsafe)
  (keyword_admin)
  (keyword_connection)
  (keyword_cycle)
  (keyword_database)
  (keyword_encrypted)
  (keyword_increment)
  (keyword_logged)
  (keyword_none)
  (keyword_owned)
  (keyword_password)
  (keyword_reset)
  (keyword_role)
  (keyword_sequence)
  (keyword_start)
  (keyword_restart)
  (keyword_tablespace)
  (keyword_until)
  (keyword_user)
  (keyword_valid)
  (keyword_action)
  (keyword_definer)
  (keyword_invoker)
  (keyword_security)
  (keyword_extension)
  (keyword_version)
  (keyword_out)
  (keyword_inout)
  (keyword_variadic)
  (keyword_ordinality)
  (keyword_session)
  (keyword_isolation)
  (keyword_level)
  (keyword_serializable)
  (keyword_repeatable)
  (keyword_read)
  (keyword_write)
  (keyword_committed)
  (keyword_uncommitted)
  (keyword_deferrable)
  (keyword_names)
  (keyword_zone)
  (keyword_immediate)
  (keyword_deferred)
  (keyword_constraints)
  (keyword_snapshot)
  (keyword_characteristics)
  (keyword_off)
  (keyword_follows)
  (keyword_precedes)
  (keyword_each)
  (keyword_instead)
  (keyword_of)
  (keyword_initially)
  (keyword_old)
  (keyword_new)
  (keyword_referencing)
  (keyword_statement)
  (keyword_execute)
  (keyword_procedure)
  (keyword_copy)
  (keyword_delimiter)
  (keyword_encoding)
  (keyword_escape)
  (keyword_force_not_null)
  (keyword_force_null)
  (keyword_force_quote)
  (keyword_freeze)
  (keyword_header)
  (keyword_match)
  (keyword_program)
  (keyword_quote)
  (keyword_stdin)
  (keyword_extended)
  (keyword_main)
  (keyword_plain)
  (keyword_storage)
  (keyword_compression)
  (keyword_duplicate)
] @keyword

[
 (keyword_restrict)
 (keyword_unbounded)
 (keyword_unique)
 (keyword_cascade)
 (keyword_delayed)
 (keyword_high_priority)
 (keyword_low_priority)
 (keyword_ignore)
 (keyword_nothing)
 (keyword_check)
 (keyword_option)
 (keyword_local)
 (keyword_cascaded)
 (keyword_wait)
 (keyword_nowait)
 (keyword_metadata)
 (keyword_incremental)
 (keyword_bin_pack)
 (keyword_noscan)
 (keyword_stats)
 (keyword_statistics)
 (keyword_maxvalue)
 (keyword_minvalue)
] @type

[
  (keyword_int)
  (keyword_boolean)
  (keyword_binary)
  (keyword_varbinary)
  (keyword_image)
  (keyword_bit)
  (keyword_inet)
  (keyword_character)
  (keyword_smallserial)
  (keyword_serial)
  (keyword_bigserial)
  (keyword_smallint)
  (keyword_mediumint)
  (keyword_bigint)
  (keyword_tinyint)
  (keyword_decimal)
  (keyword_float)
  (keyword_double)
  (keyword_numeric)
  (keyword_real)
  (double)
  (keyword_money)
  (keyword_smallmoney)
  (keyword_char)
  (keyword_nchar)
  (keyword_varchar)
  (keyword_nvarchar)
  (keyword_varying)
  (keyword_text)
  (keyword_string)
  (keyword_uuid)
  (keyword_json)
  (keyword_jsonb)
  (keyword_xml)
  (keyword_bytea)
  (keyword_enum)
  (keyword_date)
  (keyword_datetime)
  (keyword_time)
  (keyword_datetime2)
  (keyword_datetimeoffset)
  (keyword_smalldatetime)
  (keyword_timestamp)
  (keyword_timestamptz)
  (keyword_geometry)
  (keyword_geography)
  (keyword_box2d)
  (keyword_box3d)
  (keyword_interval)
] @type.builtin

[
  (keyword_in)
  (keyword_and)
  (keyword_or)
  (keyword_not)
  (keyword_by)
  (keyword_on)
  (keyword_do)
  (keyword_union)
  (keyword_except)
  (keyword_intersect)
] @keyword

[
  "+"
  "-"
  "*"
  "/"
  "%"
  "^"
  ":="
  "="
  "<"
  "<="
  "!="
  ">="
  ">"
  "<>"
  (op_other)
  (op_unary_other)
] @operator

[
  "("
  ")"
] @punctuation.bracket

[
  ";"
  ","
  "."
] @punctuation.delimiter
"##;

pub const ASTRO_HIGHLIGHTS: &str = r#"
(comment) @comment
(doctype) @tag
(tag_name) @tag
(erroneous_end_tag_name) @tag
(attribute_name) @attribute
(quoted_attribute_value) @string
(attribute_value) @string

"=" @operator
"---" @punctuation.delimiter
["<" ">" "</" "/>" "<!"] @tag
["{" "}"] @punctuation.bracket
"#;

// Frontmatter, <script>, <style> and expressions are separate languages; without
// these the whole body of an .astro file renders unhighlighted.
pub const ASTRO_INJECTIONS: &str = r#"
(frontmatter
  (frontmatter_js_block) @injection.content
  (#set! injection.language "typescript"))

(script_element
  (raw_text) @injection.content
  (#set! injection.language "typescript"))

(style_element
  (raw_text) @injection.content
  (#set! injection.language "css"))

(attribute_interpolation
  (attribute_js_expr) @injection.content
  (#set! injection.language "typescript"))

(html_interpolation
  (permissible_text) @injection.content
  (#set! injection.language "typescript"))
"#;

// Adapted from tree-sitter-yaml queries/highlights.scm (MIT): only @boolean,
// which has no HIGHLIGHT_NAMES equivalent, is remapped.
pub const YAML_HIGHLIGHTS: &str = r#"
(boolean_scalar) @constant.builtin

(null_scalar) @constant.builtin

[
  (double_quote_scalar)
  (single_quote_scalar)
  (block_scalar)
  (string_scalar)
] @string

[
  (integer_scalar)
  (float_scalar)
] @number

(comment) @comment

[
  (anchor_name)
  (alias_name)
] @label

(tag) @type

[
  (yaml_directive)
  (tag_directive)
  (reserved_directive)
] @attribute

(block_mapping_pair
  key: (flow_node
    [
      (double_quote_scalar)
      (single_quote_scalar)
    ] @property))

(block_mapping_pair
  key: (flow_node
    (plain_scalar
      (string_scalar) @property)))

(flow_mapping
  (_
    key: (flow_node
      [
        (double_quote_scalar)
        (single_quote_scalar)
      ] @property)))

(flow_mapping
  (_
    key: (flow_node
      (plain_scalar
        (string_scalar) @property))))

[
  ","
  "-"
  ":"
  ">"
  "?"
  "|"
] @punctuation.delimiter

[
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  "*"
  "&"
  "---"
  "..."
] @punctuation.special
"#;
