# Code Integrity & Anti-Elision Rules

Standards for ensuring that all code modifications result in complete, functional logic without placeholders.

## Code Modification Standards

- **Zero Code Elision**: NEVER use placeholders like `// ...`, `// implementation here`, or `/* rest of function */` to replace functional code. All code modifications must be substantially complete and functional.
- **Atomic Completeness**: Every tool call must result in a codebase that is either identical in functionality or improved. Partial rewrites that delete existing logic are strictly forbidden.
- **No todo!() as Logic**: NEVER use `todo!()`, `unimplemented!()`, or `panic!()` as a substitute for actual logic unless the specific edge case explicitly requires a panic as the intended behavior.
