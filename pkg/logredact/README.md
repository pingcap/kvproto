# Generated message formatting

The generated sensitive message types use this package for `String`,
`GoString`, and `fmt.Formatter`. Value and pointer arguments, including
messages inside containers, share the same field redaction for ordinary
formatting and `%#v`.

Formatting preserves flags, width, and precision by applying them to the
redacted text. `%#v` retains the existing unquoted `GoString` result.
Nil message pointers still format as `<nil>`. Protobuf/JSON serialization
and the original message data are unchanged.

Only the sensitive fields listed in the generated table are protected.
Direct field access and explicit serialization require protection in the
calling application.
