# Rust conventions

- Organize modules so that dependencies between them are as acyclic as possible.
- Qualify calls to functions of other modules with their parent module.
- Do not qualify type references with their parent module.
- Do not repeat the parent module or type name in a function/method name.
- Prefer slices (`&[T]`) over fixed-size arrays for `const` and `static` lists, unless their length
  matters.
