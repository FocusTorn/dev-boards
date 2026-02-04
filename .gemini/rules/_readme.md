# 1. :: Workspace Rules Orchestrator

Workspace Scoped: This document serves as the central hub for all development rules and integrity standards enforced within this workspace.

## 1.1. :: Rule Catalog

*   **[codebase-integrity-and-standards.md](./codebase-integrity-and-standards.md)**: Standards for code quality, anti-elision, and library stability.
*   **[documentation-and-formatting.md](./documentation-and-formatting.md)**: Standards for markdown documentation, file layout, and line endings (LF).
*   **[environment-and-execution.md](./environment-and-execution.md)**: Standards for managing the development environment, `WORKSPACE_ROOT` resolution, and secret management.
*   **[tooling-and-interaction.md](./tooling-and-interaction.md)**: Standards for tool selection, shell usage (pwsh), and precision execution.

## 1.2. :: Creating & Maintaining Rules

To maintain the high standard of this workspace, all new rules (regardless of scope) MUST adhere to the following:

1.  **Rule Containment**: Rule files should act as containers for related concepts. Avoid creating single-issue files.
2.  **Semantic Headers**: Use the `[Number]. :: [Title]` format (e.g., `## 1. :: Title`).
3.  **Plain Text Titles**: Titles MUST NOT contain bold formatting.
4.  **Specific & Actionable**: Rules must provide clear "MUST/NEVER" directives.
5.  **Hierarchical Numbering**: H3 headers must include the parent section number (e.g., `### 1.1. :: Subtitle`).