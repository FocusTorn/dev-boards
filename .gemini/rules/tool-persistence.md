# Tool & Shell Standards

Standards for tool selection, shell environment, and persistence to ensure maximum efficiency and accuracy.

## Shell Environment

- **PowerShell 7 (pwsh) Preference**: You MUST use `pwsh` as the primary shell.
- **PowerShell Syntax**: Use standard PowerShell syntax (e.g., `&&`, `;`, `Test-Path`) and avoid legacy CMD or Bash-specific wrappers unless executing a script that requires them.
- **Absolute Pathing**: When referencing the shell for environment setup, always use the absolute path to the `pwsh.exe` executable to ensure consistent process resolution.

This ensures the CLI's internal `getShellConfiguration()` logic detects a valid PowerShell executable while maintaining the full environment context required for server connectivity.

## Preferred Tool Persistence

- **OPTIMIZED SEARCH**: `search_file_content` is the MANDATORY first choice for all codebase-wide searches.
- **ATOMIC REPLACEMENT**: Use the `replace` tool for targeted, atomic changes. Ensure `old_string` includes exactly 3 lines of context to guarantee a match. Avoid rewriting large files with `write_file` unless the modification is structural or formatting-based.
- **ERROR RECOVERY**: If a tool call fails with a syntax, regex, or usage error, you MUST analyze the error message and attempt at least one corrected call before pivoting to a less efficient tool.
- **CLOSED LOOP**: If a tool provides optimization suggestions, you MUST consider and apply them if they align with the task objective.
