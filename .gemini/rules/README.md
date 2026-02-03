# 1. :: Workspace Rules Orchestrator

This document serves as the central hub for all development rules and integrity standards enforced within this workspace.

## 1.1. :: Rule Catalog

*   **[code-integrity.md](./code-integrity.md)**: Standards for ensuring code modifications are complete and functional without placeholders or elision.
*   **[formatting-integrity.md](./formatting-integrity.md)**: Rules for maintaining consistent line endings (LF), file layout, and tool-specific formatting.
*   **[markdown-formatting.md](./markdown-formatting.md)**: Strict standards for markdown documentation (plain text titles, sequential numbering).
*   **[stability.md](./stability.md)**: Policies for library versioning, preferring LTS, and prohibiting bleeding-edge dependencies.
*   **[tool-persistence.md](./tool-persistence.md)**: Standards for shell usage (PowerShell 7) and optimized tool selection.

## 1.2. :: Creating & Maintaining Rules

To maintain the high standard of this workspace, all new rules MUST adhere to the following:

1.  **Semantic Headers**: Use the `[Number]. :: [Title]` format (e.g., `## 1. :: Title`).
2.  **Plain Text Titles**: Titles MUST NOT contain bold formatting.
3.  **Specific & Actionable**: Rules must provide clear "MUST/NEVER" directives.
4.  **Hierarchical Numbering**: H3 headers must include the parent section number (e.g., `### 1.1. :: Subtitle`).
