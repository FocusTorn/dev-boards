# Tool & Shell Standards

Standards for tool selection, shell environment, and persistence to ensure maximum efficiency and accuracy.

## Shell Environment

- **GitBash Preference**: You MUST use `GitBash` (typically `sh` or `bash`) as the primary shell for all `run_shell_command` calls. 
- **Unix Syntax**: Always use Unix-style syntax (e.g., `ls -la`, `mkdir -p`, `rm -rf`) instead of PowerShell cmdlets to maintain consistency with `LF` line endings.
- **Path Handling**: Use forward slashes (`/`) for all paths in shell commands to ensure compatibility with GitBash.

## Preferred Tool Persistence

- **OPTIMIZED SEARCH**: `search_file_content` is the MANDATORY first choice for all codebase-wide searches.
- **ATOMIC REPLACEMENT**: Use the `replace` tool for targeted, atomic changes. Ensure `old_string` includes exactly 3 lines of context to guarantee a match. Avoid rewriting large files with `write_file` unless the modification is structural or formatting-based.
- **ERROR RECOVERY**: If a tool call fails with a syntax, regex, or usage error, you MUST analyze the error message and attempt at least one corrected call before pivoting to a less efficient tool.
- **CLOSED LOOP**: If a tool provides optimization suggestions, you MUST consider and apply them if they align with the task objective.
