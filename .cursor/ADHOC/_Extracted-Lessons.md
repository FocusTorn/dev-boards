# Extracted Lessons - Sync Manager Conflict Resolution Improvements

## 1. :: Shorthand Reference Mapping and Explanations <!-- Start Fold -->

### 1.1. :: Alias Mapping

- **_strat**: `docs/testing/_Testing-Strategy.md`
- **_ts**: `docs/testing/_Troubleshooting - Tests.md`
- **lib guide**: `docs/testing/Library-Testing-AI-Guide.md`
- **terminal-output**: `.cursor/rules/formatting/terminal-output.mdc`

### 1.2. :: Details for Shorthand Execution Details

#### Add to strat

You will understand that _add to strat_ means to do the following:

1. Add the needed documentation to **_strat**
2. Ensure there is a `### **Documentation References**` to **_strat** within **guide**
3. Add or modify a concise section with a pointer to the main file for more detail to **guide**

#### Add to trouble

You will understand that _add to trouble_ means to do the following:

1. Add the needed documentation to **_ts**
2. Ensure there is a `### **Documentation References**` to **_strat** within **guide**
3. Add or modify a concise section with a pointer to the main file for more detail to **guide**

---

<!-- Close Fold -->

## 2.0 :: IDE Detection for Diff Viewing <!-- Start Fold -->

- **Learning**: When opening diffs in external editors, the system should detect which IDE (Cursor or VS Code) is currently running and prioritize using the active IDE for better user experience.

- **Pattern**: Detect active IDE by checking running processes (Windows: `tasklist` for `Cursor.exe`/`Code.exe`, Unix: `pgrep` for process names), then try active IDE first, followed by Cursor, then VS Code as fallbacks.

- **Implementation**: Created `_detect_active_ide()` function that checks running processes and returns 'cursor', 'code', or None. Updated `_open_diff_in_editor()` to use detected active IDE first, with fallback order: active IDE → Cursor → VS Code.

- **Benefit**: Users get diffs opened in their current working IDE automatically, providing better context and workflow continuity. Reduces need to manually specify which IDE to use.

- **Not documented**: IDE detection patterns for command-line tools are not documented. The approach of checking running processes to determine active applications is not captured in any documentation.

- **Mistake/Assumption**: Initially assumed VS Code should always be tried first, without considering that the user might be actively working in Cursor.

- **Correction**: Implemented active IDE detection that checks running processes and prioritizes the active IDE, ensuring diffs open in the user's current working environment.

- **Recommendation**:
    - **AI Agent Rule**: 
        - **Action**: ADD to existing rule file
        - **Rule File Path**: `.cursor/rules/formatting/terminal-output.mdc`
        - **Rule Text**: "When opening external tools (IDEs, diff viewers, editors) from command-line scripts, ALWAYS detect and prioritize the active/currently running application. Check running processes to determine which application is active, then try active application first before falling back to alternatives. This ensures tools open in the user's current working environment for better workflow continuity."
        - **Section**: "FORMATTING PATTERNS" or create new subsection "External Tool Integration"
    - Document IDE detection patterns in sync-manager package documentation
    - Add examples of process detection for both Windows and Unix systems

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

## 2.1 :: Windows Command-Line Tools vs GUI Tools Distinction <!-- Start Fold -->

- **Learning**: Windows `fc` (file compare) command is a command-line tool that outputs to stdout, not a GUI application. It cannot be used for interactive diff viewing in the same way as GUI tools like WinMerge or VS Code.

- **Pattern**: When selecting diff tools, distinguish between command-line tools (that output text) and GUI tools (that open windows). Only GUI tools should be used for interactive diff viewing. Command-line tools like `fc` should be excluded from GUI tool lists.

- **Implementation**: Removed `fc` from the list of GUI diff tools in `_open_diff_in_editor()` function. Added proper GUI tool detection with full path checking for Windows tools like WinMerge and Beyond Compare.

- **Benefit**: Prevents confusion when a tool is reported as "opened" but nothing visible happens. Users get actual GUI applications that provide visual diff interfaces.

- **Not documented**: The distinction between command-line diff tools and GUI diff tools is not documented. The fact that some tools output to stdout while others open windows is not captured in any documentation.

- **Mistake/Assumption**: Initially assumed `fc` could be used as a GUI tool for diff viewing, leading to user confusion when it appeared to "open" but produced no visible output.

- **Correction**: Removed `fc` from GUI tool list and added proper detection for actual GUI tools (WinMerge, Beyond Compare, etc.) with full path checking and proper process launching.

- **Recommendation**:
    - **AI Agent Rule**: 
        - **Action**: ADD to existing rule file
        - **Rule File Path**: `.cursor/rules/formatting/terminal-output.mdc`
        - **Rule Text**: "When selecting external tools for GUI operations (diff viewing, file editing, etc.), ALWAYS distinguish between command-line tools (that output to stdout/stderr) and GUI tools (that open windows). NEVER include command-line-only tools in GUI tool lists. Verify that tools actually open windows before including them in GUI tool selection. Use process detection or command verification to confirm tool capabilities."
        - **Section**: "FORMATTING PATTERNS" > "External Tool Integration" (or create if doesn't exist)
    - Document the distinction between CLI and GUI tools in sync-manager package documentation
    - Add examples of proper GUI tool detection patterns

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

## 2.2 :: Batch Conflict Resolution with Git-Like View <!-- Start Fold -->

- **Learning**: When multiple file conflicts exist, presenting them in a git-like summary view (showing resolved vs pending) with interactive navigation provides much better UX than resolving conflicts one-by-one sequentially.

- **Pattern**: Collect all conflicts during first pass (file scanning), then present them in a structured view showing: resolved conflicts with checkmarks, pending conflicts numbered for selection, progress indicator (X/Y resolved), and batch action options (resolve all, skip all, open all in IDE).

- **Implementation**: Created `resolve_conflicts_batch()` function that maintains lists of resolved and pending conflicts, displays summary using `_show_conflict_summary()`, and allows users to select conflicts to resolve in any order. Integrated with two-pass sync approach where conflicts are detected during progress bar phase, then resolved interactively after.

- **Benefit**: Users can see all conflicts at once, understand overall progress, resolve conflicts in any order, and use batch operations when appropriate. Much more efficient than sequential one-by-one resolution.

- **Not documented**: Batch conflict resolution patterns and git-like summary views for interactive conflict resolution are not documented. The pattern of separating detection from resolution (two-pass approach) is not captured in documentation.

- **Mistake/Assumption**: None - this was a new feature request that was implemented correctly.

- **Correction**: N/A

- **Recommendation**:
    - **AI Agent Rule**: 
        - **Action**: ADD to existing rule file
        - **Rule File Path**: `.cursor/rules/formatting/terminal-output.mdc`
        - **Rule Text**: "When presenting multiple interactive items (conflicts, errors, choices) to users, ALWAYS provide a summary view showing: resolved/completed items, pending items numbered for selection, progress indicators (X/Y completed), and batch action options. Use a git-like view format that allows users to see all items at once and navigate/select in any order. Separate detection/collection phase from resolution/interaction phase when dealing with multiple items."
        - **Section**: "FORMATTING PATTERNS" > "Interactive Multi-Item Selection"
    - Document batch conflict resolution patterns in sync-manager package documentation
    - Add examples of git-like summary view implementation

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

## 2.3 :: Batch Opening Multiple Files in IDE Diff View <!-- Start Fold -->

- **Learning**: When multiple conflicts need to be resolved, opening all of them at once in the IDE's diff view allows users to resolve them efficiently in a familiar environment, rather than one at a time in the terminal.

- **Pattern**: For each conflict in a batch, open a separate diff window in the IDE using `cursor -d` or `code --diff` commands. Add small delays (0.2s) between opens to avoid overwhelming the IDE. Provide feedback on how many conflicts were opened.

- **Implementation**: Created `_open_all_conflicts_in_editor()` function that iterates through all conflicts, opens each as a separate diff window in the detected IDE, with delays between opens. Integrated into batch conflict resolution menu as first option.

- **Benefit**: Users can see all conflicts side-by-side in their IDE, resolve them using familiar IDE tools, and work more efficiently than terminal-based resolution. Maintains workflow continuity.

- **Not documented**: Patterns for batch opening multiple files in external tools are not documented. The approach of opening multiple diff windows with delays is not captured in documentation.

- **Mistake/Assumption**: None - this was a new feature request implemented correctly.

- **Correction**: N/A

- **Recommendation**:
    - **AI Agent Rule**: 
        - **Action**: ADD to existing rule file
        - **Rule File Path**: `.cursor/rules/formatting/terminal-output.mdc`
        - **Rule Text**: "When batch opening multiple files in external tools (IDEs, editors), ALWAYS open each file as a separate window/tab and add small delays (0.1-0.3s) between opens to avoid overwhelming the application. Provide feedback on how many items were successfully opened. Allow users to resolve items in the external tool, then continue in the terminal interface."
        - **Section**: "FORMATTING PATTERNS" > "External Tool Integration"
    - Document batch file opening patterns in sync-manager package documentation

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

## 2.4 :: Missing Function Implementation Error <!-- Start Fold -->

- **Learning**: When adding new functionality that requires a function, both the function definition AND all references to it must be added. Import statements will fail if the function doesn't exist, even if the code structure appears correct.

- **Pattern**: When creating new functions that are imported elsewhere, ensure the function is actually implemented in the source file before adding import statements. The function definition must exist before it can be imported.

- **Implementation**: Added `resolve_conflicts_batch()` function to `conflict_resolver.py` after it was already referenced in `file_sync.py` import statement, causing `ImportError: cannot import name 'resolve_conflicts_batch'`.

- **Benefit**: Prevents import errors and ensures code is actually executable. Maintains proper dependency order.

- **Not documented**: The pattern of ensuring function definitions exist before adding import statements is basic Python knowledge but not explicitly documented in project guidelines.

- **Mistake/Assumption**: Initially added the import statement and function call before actually implementing the function definition, assuming it would be added in the same operation.

- **Correction**: Added the complete `resolve_conflicts_batch()` function implementation to `conflict_resolver.py`, including all helper functions like `_show_conflict_summary()` and `_open_all_conflicts_in_editor()`.

- **Recommendation**:
    - **AI Agent Rule**: 
        - **Action**: ADD to existing rule file
        - **Rule File Path**: `.cursor/rules/by-language/python/code-organization.mdc`
        - **Rule Text**: "When adding new functions that will be imported by other modules, ALWAYS implement the complete function definition in the source file BEFORE adding import statements in dependent files. Verify the function exists and is complete before referencing it in imports. This prevents ImportError exceptions and ensures code is executable."
        - **Section**: "Import Management" or "Function Definition Order"
    - This is basic Python knowledge but could be emphasized in code organization guidelines

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

## 2.5 :: Diff Display Formatting Improvements <!-- Start Fold -->

- **Learning**: Git's word-level diff highlighting (`--word-diff=color`) provides much more readable diff output than standard unified diff format, especially for inline changes. Filtering out verbose git headers (diff --git, index, full paths) makes the output cleaner and easier to read.

- **Pattern**: Use git diff with word-level highlighting as primary method, filter out header lines (diff --git, index, --- a/, +++ b/), show only filenames instead of full paths, and add visual separators around diff output for clarity.

- **Implementation**: Updated `_show_diff()` function to use `git diff --no-index --color=always --word-diff=color --word-diff-regex=.` as primary method, filter header lines, extract just filenames from paths, and add separator lines around diff output.

- **Benefit**: Much more readable diff output that shows changes inline with color highlighting, similar to git's working tree view. Cleaner output without verbose headers.

- **Not documented**: Git diff formatting options and word-level highlighting patterns are not documented. The approach of filtering headers for cleaner output is not captured in documentation.

- **Mistake/Assumption**: Initially showed full git diff output with all headers, which was verbose and harder to read than necessary.

- **Correction**: Added header filtering to show only relevant diff content, extracted filenames from full paths, and added visual separators for better readability.

- **Recommendation**:
    - **AI Agent Rule**: 
        - **Action**: ADD to existing rule file
        - **Rule File Path**: `.cursor/rules/formatting/terminal-output.mdc`
        - **Rule Text**: "When displaying diff output in terminal, ALWAYS use git's word-level highlighting (`--word-diff=color`) when available for better readability. Filter out verbose headers (diff --git, index lines, full file paths) and show only filenames. Add visual separators (horizontal lines) around diff output for clarity. Prioritize readability over showing all available information."
        - **Section**: "FORMATTING PATTERNS" > "Diff Display Formatting"
    - Document git diff formatting best practices in sync-manager package documentation

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

## 2.6 :: Duplicate Prompt Output Issue <!-- Start Fold -->

- **Learning**: When using `prompt_toolkit.Application`, the layout already handles displaying the selected answer inline. Adding explicit print statements after `app.run()` causes duplicate output - the answer appears twice.

- **Pattern**: `prompt_toolkit.Application`'s built-in layout automatically displays the selected answer when `is_answered` is true. Do not add manual print statements to display the answer after `app.run()` returns, as this creates duplicates.

- **Implementation**: Removed redundant print statements in `pyprompt/prompts.py` that were printing the selected answer after `app.run()` completed, since the Application layout already handles this display.

- **Benefit**: Cleaner output without duplicate lines. Users see the answer once, as intended by prompt_toolkit's design.

- **Not documented**: The behavior of `prompt_toolkit.Application` regarding automatic answer display is not documented in the pyprompt package. The fact that manual printing causes duplicates is not captured.

- **Mistake/Assumption**: Initially added explicit print statements to show the selected answer, not realizing that `prompt_toolkit.Application` already handles this automatically through its layout system.

- **Correction**: Removed the redundant print statements that were causing duplicate output, relying on prompt_toolkit's built-in answer display mechanism.

- **Recommendation**:
    - **AI Agent Rule**: 
        - **Action**: ADD to existing rule file
        - **Rule File Path**: `.cursor/rules/by-language/python/code-organization.mdc`
        - **Rule Text**: "When using `prompt_toolkit.Application` for interactive prompts, NEVER add manual print statements to display the selected answer after `app.run()` returns. The Application's layout automatically handles displaying the answer inline when `is_answered` is true. Adding manual prints causes duplicate output."
        - **Section**: "Interactive Prompt Patterns" or "Library-Specific Patterns"
    - Document prompt_toolkit behavior in pyprompt package documentation
    - Add note about automatic answer display in Application layout

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

