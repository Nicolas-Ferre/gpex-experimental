# Rust conventions

- Organize modules so that dependencies between them are as acyclic as possible.
- Function calls are qualified with parent module.
- Type references are not qualified with parent module.
- Do not repeat the parent module or type name in a function/method name.
