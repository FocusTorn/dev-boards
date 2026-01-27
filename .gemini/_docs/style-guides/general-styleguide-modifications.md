# General Style Guide Modifications & Overrides

This document defines universal standards and folding marker variants that apply across all programming languages and configuration files in this workspace.

## 1. Universal Folding Markers

### Overview
Folding markers are strictly mandatory for code organization. They must never be removed or altered unless explicitly requested. These markers are designed to work with custom IDE folding rules.

### Marker Variants

#### A. Short Comment Arrows
Used for quick blocks in most C-style, Shell, or Lisp-style comments.
- **Start**: `//>` or `#>` or `;>`
- **End**: `//<` or `#<` or `;<`
- **Example**:
  ```rust
  if condition { //>
      // ... block logic ...
  } //<
  ```

#### B. Directional Mid Arrows
Used for multi-section folding blocks.
- **Start**: `//|>`
- **Middle (Transition)**: `//||>`
- **End**: `//<|`

#### C. Long Comment Arrows
Used for high-level sectioning or large data structures. Requires at least two dashes or equal signs.
- **Start**: `-->>` or `==>>`
- **End**: `--<<` or `==<<`
- **Example**:
  ```json
  "LargeDataBlock": [ ==>>
      { ... },
      { ... }
  ] ==<<
  ```

---

## 2. Zero-Tolerance for Code Elision

### The Mandate
Never submit or modify code that replaces actual implementation with placeholder comments. Every tool call must result in substantially complete and functional code.

### Forbidden Practices
- **Code Removal**: Do not use `// ...`, `// implementation here`, or `/* rest of function */` to skip over existing or required logic. 
- **Incomplete Logic**: Do not use `todo!()`, `unimplemented!()`, or `panic!()` as a substitute for actual logic (unless `panic!` is the intended error-handling behavior for a specific edge case).

### Allowed Annotations
- **Task Tracking**: `// TODO` and `// FIXME` comments are acceptable for marking technical debt or future enhancements, provided they do not replace required functional code.

### Protocol for Large Tasks
If a requested change is too large to implement in a single turn without elision:
1. **Halt** the implementation immediately.
2. **Explain** the need for a multi-turn task breakdown to the user.
3. **Wait** for a refined task plan.
**NEVER** attempt to "save space" by deleting or commenting out functional code.

---

## 3. Newline & EOL Consistency

### The Mandate
This workspace strictly uses Unix-style line endings (`LF` / `\n`). To prevent "phantom" blank lines and ensure cross-platform compatibility, all file modifications must use the `LF` character.

### Modification Rules
- **Tool Selection**: For complex multi-line changes or when adding documentation blocks, prefer `write_file` over `replace`. `write_file` ensures a clean write of the entire content buffer.
- **Verification**: If blank lines or `\r` characters appear after an edit, the agent must immediately verify the file content and use `write_file` to normalize the entire file back to its correct `LF` state.
- **Git Config**: The workspace is configured with `core.autocrlf = input` to prevent automatic conversion back to CRLF on Windows.

---

## 4. Formatting Persistence
- **The Rule**: If you encounter these markers in existing code, preserve them exactly as they are.
- **The Rule**: Do not add extra whitespace between the comment character and the arrow (e.g., use `//>` not `// >`) unless the existing file context consistently uses a space.
