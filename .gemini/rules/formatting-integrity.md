# Formatting & Layout Integrity Rules

Standards for maintaining consistent file structure, line endings, and layout across the workspace.

## Newline & EOL Consistency

- **LF Enforcement**: This workspace strictly uses Unix-style line endings (`LF` / `\n`). 
- **Automatic Rectification**: If a `read_file` call reveals `\r` (CR) characters, you MUST normalize the entire file buffer back to its correct `LF` state.

## Tool-Specific Formatting Restrictions

- **Precision Replace**: When using the `replace` tool, you MUST include exactly 3 lines of context before and after the target text. This ensures precise targeting and prevents match failures due to minor whitespace differences.
- **Structural Write**: Use `write_file` for structural changes, complex documentation, or when creating new files. For editing existing code logic, prefer the `replace` tool to minimize the risk of "punctuation drops" in high-volume generation.
- **Buffer Integrity**: Before executing a `write_file` to fix formatting, ensure you have the complete and correct file content in your context to prevent accidental data loss or code elision.

