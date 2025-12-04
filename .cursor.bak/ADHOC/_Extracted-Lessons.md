# Extract - Lessons Learned: Prompt System Refactoring

## 1. :: Shorthand reference Mapping and Explanations <!-- Start Fold -->

### 1.1. :: Alias Mapping

- **\_strat**: `docs/testing/_Testing-Strategy.md`
- **\_ts**: `docs/testing/_Troubleshooting - Tests.md`
- **lib guide**: `docs/testing/Library-Testing-AI-Guide.md`

### 1.2. :: Details for shorthand execution details:

#### Add to strat

You will understand that _add to strat_ means to do the following:

1. Add the needed documentation to **\_strat**
2. Ensure there is a `### **Documentation References**` to **\_strat** within **guide**
3. Add or modify a concise section with a pointer to the main file for more detail to **guide**

#### Add to trouble

You will understand that _add to trouble_ means to do the following:

1. Add the needed documentation to **\_ts**
2. Ensure there is a `### **Documentation References**` to **\_strat** within **guide**
3. Add or modify a concise section with a pointer to the main file for more detail to **guide**

---

<!-- Close Fold -->

## 2.0 :: Prompt Toolkit Migration and Region System Integration <!-- Start Fold -->

- Learning: prompt_toolkit's Application class bypasses Python's stdout wrapper system, requiring manual indentation calculation when using custom output indentation systems like the region system
- Pattern: When using prompt_toolkit for interactive prompts within region contexts, manually calculate and apply region indentation using `get_region_indent()` rather than relying on automatic indentation
- Implementation: Import `get_region_indent()` from terminal module, calculate region indent before creating prompt_toolkit Application, and apply indent to pointer and choice positioning calculations
- Benefit: Ensures prompts align correctly with other output that uses the region system, maintaining visual consistency

- **Not documented**: The interaction between prompt_toolkit and custom stdout wrappers is not documented. The fact that prompt_toolkit writes directly to TTY bypassing IndentedOutput wrappers needs to be documented in terminal output formatting rules

- **Mistake/Assumption**: Initially assumed prompt_toolkit would automatically respect the region system's IndentedOutput wrapper, causing misaligned prompts
- **Correction**: Manually calculate region indent using `get_region_indent()` and apply it to prompt positioning calculations, ensuring pointer aligns with qmark position

- **Recommendation**:
    - **AI Agent Rule**: 
        - **Action**: MODIFY existing rule file
        - **Rule File Path**: `.cursor/rules/formatting/terminal-output.mdc`
        - **Rule Text**: "**CRITICAL**: When using prompt_toolkit Application class for interactive prompts (select, confirm), the Application bypasses Python's stdout wrapper system. ALWAYS manually calculate region indentation using `get_region_indent()` and apply it to pointer/choice positioning. NEVER assume prompt_toolkit will automatically respect IndentedOutput wrappers."
        - **Section**: "HEADER CODE BLOCKS AND REGION INDENTING > Interactive Prompt Indentation"
    - Update terminal output formatting rules to document prompt_toolkit's TTY bypass behavior
    - Add example showing manual indentation calculation for prompt_toolkit prompts

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

## 2.0 :: Dynamic Indentation Calculation for Select Prompts <!-- Start Fold -->

- Learning: Select prompt choices should align dynamically based on qmark position (if present) or message start position, with unselected choices aligning to the selected choice's first character position
- Pattern: Calculate indentation relative to region base: if qmark exists, pointer aligns to qmark; selected text starts after pointer+space; unselected text aligns to selected text's first character position
- Implementation: Extract pointer symbol (remove leading space), calculate `pointer_indent` from region base, calculate `selected_text_start_pos` as `len(region_indent) + len(pointer) + 1`, use this for unselected indent calculation
- Benefit: Ensures consistent visual alignment regardless of whether qmark is present, maintaining professional appearance

- **Not documented**: Dynamic indentation calculation pattern for select prompts is not documented. The rule that unselected choices should align to selected choice's first character position needs to be documented

- **Mistake/Assumption**: Initially used fixed indentation values (2 spaces, 4 spaces) without calculating based on actual prompt message positioning
- **Correction**: Implemented dynamic calculation that measures actual positions: pointer aligns to qmark (or message start), selected text position calculated from pointer+space, unselected aligns to selected text's first character

- **Recommendation**:
    - **AI Agent Rule**: 
        - **Action**: MODIFY existing rule file
        - **Rule File Path**: `.cursor/rules/formatting/terminal-output.mdc`
        - **Rule Text**: "**CRITICAL**: For select prompts, ALWAYS calculate indentation dynamically: (1) If qmark exists, pointer aligns to qmark position; if no qmark, pointer aligns to message start. (2) Selected text's first character position = region_indent + pointer_length + space. (3) Unselected choices MUST align to selected text's first character position. NEVER use fixed indentation values."
        - **Section**: "HEADER CODE BLOCKS AND REGION INDENTING > Interactive Prompt Indentation"
    - Document dynamic indentation calculation pattern in terminal output formatting rules
    - Add visual examples showing alignment calculations

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

## 2.0 :: Confirm Prompt Format Standardization <!-- Start Fold -->

- Learning: Confirm prompts should display as `{{qmark}} {{message}} [Y/n]? {{default_answer}}` where [Y/n] is dim grey, default answer is blue, and the prompt submits immediately on y/n/Enter keypress
- Pattern: Format prompt with qmark (pink), question text (white), [Y/n] indicator in dim grey, then default answer in blue. Use single-character input (tty.setraw on Unix, msvcrt.getch on Windows) for immediate submission without Enter key
- Implementation: Build prompt string with color codes, print with end='' flush=True, read single character in raw mode, handle y/n/Enter/Ctrl+C, restore terminal settings immediately after reading, display chosen answer in blue
- Benefit: Provides clear visual indication of default, immediate feedback, and consistent formatting across all confirm prompts

- **Not documented**: The confirm prompt format specification ([Y/n] in dim grey, default in blue, immediate submission) is not documented in terminal output formatting rules

- **Mistake/Assumption**: Initially used questionary's confirm which had different formatting and required Enter key. Also assumed Windows would handle single-character input the same as Unix
- **Correction**: Implemented custom confirm function using prompt_toolkit patterns with manual formatting, added Windows support using msvcrt.getch(), ensured immediate submission on y/n keypress

- **Recommendation**:
    - **AI Agent Rule**: 
        - **Action**: MODIFY existing rule file
        - **Rule File Path**: `.cursor/rules/formatting/terminal-output.mdc`
        - **Rule Text**: "**CRITICAL**: Confirm prompts MUST display as `{{qmark}} {{message}} [Y/n]? {{default_answer}}` where: (1) [Y/n] is in dim grey (DIM_GREY color), (2) default answer (y/n) is in blue (ANSWER_COLOR), (3) prompt submits immediately on y/n keypress (no Enter required), (4) Enter key submits default. ALWAYS use single-character input (tty.setraw on Unix, msvcrt.getch on Windows) for immediate submission."
        - **Section**: "HEADER CODE BLOCKS AND REGION INDENTING > Interactive Prompt Indentation > Confirmation Prompt Formatting"
    - Update terminal output formatting rules with confirm prompt format specification
    - Document Windows vs Unix input handling differences

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

## 2.0 :: Warning Message Formatting Standards <!-- Start Fold -->

- Learning: Warning messages must have a blank line above them and the entire warning line (including icon and text) should be colored yellow, not just the icon
- Pattern: Print blank line before warning, apply COLOR_YELLOW at start of warning message, include entire message text within yellow color, apply COLOR_RESET only at end of message
- Implementation: `print()` for blank line, then `print(f"{COLOR_YELLOW}⚠ {message_text}{COLOR_RESET}")` ensuring COLOR_RESET is at the very end
- Benefit: Improves visual separation and makes warnings more prominent and readable

- **Not documented**: Warning formatting standards (blank line above, entire line yellow) are not documented in terminal output formatting rules

- **Mistake/Assumption**: Initially only colored the warning icon (⚠) yellow, leaving the message text in default color
- **Correction**: Moved COLOR_RESET to end of entire warning message, ensuring both icon and text are yellow. Added blank line before all warnings for visual separation

- **Recommendation**:
    - **AI Agent Rule**: 
        - **Action**: MODIFY existing rule file
        - **Rule File Path**: `.cursor/rules/formatting/terminal-output.mdc`
        - **Rule Text**: "**CRITICAL**: Warning messages MUST: (1) Have a blank line (`print()`) above them, (2) Apply COLOR_YELLOW to the ENTIRE warning line (icon + text), (3) Place COLOR_RESET only at the end of the complete message. NEVER color only the warning icon - the entire line must be yellow."
        - **Section**: "COLORATION AND ICONS FOR MESSAGES > Status Symbol Colors > Icon Usage"
    - Add warning formatting standards to terminal output formatting rules
    - Include examples showing correct vs incorrect warning formatting

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

## 2.0 :: Backward Compatibility Code Removal <!-- Start Fold -->

- Learning: After refactoring prompt system from questionary to prompt_toolkit, all backward compatibility code (aliases, legacy function names, compatibility constants) should be removed immediately to avoid confusion and maintain clean codebase
- Pattern: Remove legacy aliases (confirm_custom → confirm), remove compatibility constants (HAS_TERMINAL_MENU → HAS_PROMPT_TOOLKIT), remove unused style parameters, update all imports and function calls throughout codebase
- Implementation: Search codebase for all references to legacy names, replace with new names, remove legacy function definitions, update __init__.py exports, remove compatibility comments
- Benefit: Eliminates confusion about which functions to use, reduces code complexity, prevents accidental use of deprecated patterns

- **Not documented**: The principle that backward compatibility code should be removed after refactoring (not kept "just in case") is not documented

- **Mistake/Assumption**: Initially kept backward compatibility aliases thinking they might be useful, but user explicitly requested removal
- **Correction**: Removed all legacy code: confirm_custom function, HAS_TERMINAL_MENU constant, MENU_STYLE constant, updated all imports and calls throughout codebase

- **Recommendation**:
    - **AI Agent Rule**: 
        - **Action**: CREATE new rule file
        - **Rule File Path**: `.cursor/rules/code-maintenance.mdc`
        - **Rule Text**: "**CRITICAL**: After refactoring code to use new patterns or libraries, ALWAYS remove ALL backward compatibility code (aliases, legacy function names, compatibility constants) immediately. NEVER keep backward compatibility code 'just in case' - it creates confusion and technical debt. Update all imports, function calls, and exports throughout the entire codebase in a single pass."
        - **Section**: N/A (new file)
    - Document code maintenance principles around backward compatibility removal

- **Response**: ✏️❓❓❌⚠️✅ No action required

---

<!-- Close Fold -->

## 2.0 :: Nested Try-Except Block Indentation Patterns <!-- Start Fold -->

- Learning: When nesting try-except-finally blocks, the except clause must be at the same indentation level as its corresponding try block, not nested inside finally blocks
- Pattern: Structure nested exception handling as: outer try → inner try → finally (for cleanup) → except (for outer try, same level as outer try). The finally block belongs to the inner try, the except belongs to the outer try
- Implementation: Ensure except clauses align with their corresponding try statements. Use finally for cleanup that must always execute, use except for handling exceptions from the try block
- Benefit: Prevents syntax errors and ensures proper exception handling flow

- **Not documented**: Nested try-except-finally block structure and indentation rules are not documented

- **Mistake/Assumption**: Placed except block after finally block at incorrect indentation level, causing syntax error
- **Correction**: Moved except block to same indentation level as outer try block, ensuring proper exception handling structure

- **Recommendation**:
    - **AI Agent Rule**: 
        - **Action**: CREATE new rule file
        - **Rule File Path**: `.cursor/rules/code-structure.mdc`
        - **Rule Text**: "**CRITICAL**: When nesting try-except-finally blocks, ALWAYS ensure except clauses are at the same indentation level as their corresponding try blocks. NEVER place except blocks after finally blocks or at incorrect indentation levels. The structure MUST be: `try:` → `try:` → `finally:` → `except:` where the except aligns with the outer try."
        - **Section**: N/A (new file)
    - Document Python exception handling structure patterns

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

## 2.0 :: Region System Indent Parameter Handling <!-- Start Fold -->

- Learning: When calling prompt functions from within region contexts (using write_header), the indent parameter should be empty string ("") because the region system automatically adds base indentation via stdout wrapper
- Pattern: Inside `with write_header()` blocks, pass `indent=""` to prompt functions. The region system's IndentedOutput wrapper automatically adds 2 spaces per active region. Only add manual indent if prompt_toolkit bypasses the wrapper
- Implementation: Remove `indent=get_region_indent()` calls from within region contexts, use `indent=""` instead. For prompt_toolkit Application class, manually calculate indent since it bypasses wrapper
- Benefit: Prevents double indentation, ensures consistent alignment with other region output

- **Not documented**: The rule that indent parameter should be empty within region contexts is not clearly documented. The distinction between region-aware prompts (empty indent) and prompt_toolkit Application (manual indent) needs clarification

- **Mistake/Assumption**: Initially passed `get_region_indent()` to confirm_custom calls within region contexts, causing double indentation (region system + manual indent)
- **Correction**: Changed to `indent=""` for confirm calls within regions, allowing region system to handle base indentation automatically

- **Recommendation**:
    - **AI Agent Rule**: 
        - **Action**: MODIFY existing rule file
        - **Rule File Path**: `.cursor/rules/formatting/terminal-output.mdc`
        - **Rule Text**: "**CRITICAL**: When calling prompt functions (text, select, confirm) from within `with write_header()` region contexts, ALWAYS pass `indent=""` (empty string). The region system's IndentedOutput wrapper automatically adds base indentation. ONLY manually calculate indent for prompt_toolkit Application class which bypasses the wrapper. NEVER pass `get_region_indent()` to prompt functions within regions - this causes double indentation."
        - **Section**: "HEADER CODE BLOCKS AND REGION INDENTING > Interactive Prompt Indentation"
    - Clarify indent parameter usage in terminal output formatting rules
    - Add examples showing correct vs incorrect indent usage within regions

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

## 2.0 :: Single Header Per Section Pattern <!-- Start Fold -->

- Learning: Each functional section (e.g., "Local Repository", "Remote Repository") should have only one header that wraps all related prompts, summaries, and confirmations, not multiple headers for different parts of the same section
- Pattern: Consolidate all section logic (prompts, checks, summaries, confirmations) within a single `with write_header("Section Name")` block. Remove duplicate headers that were created for different parts of the same workflow
- Implementation: Move all prompts, summary displays, and confirmations into one header context, ensuring logical flow within single section boundary
- Benefit: Provides clear visual organization, prevents header duplication, maintains consistent user experience

- **Not documented**: The pattern of single header per functional section is not documented

- **Mistake/Assumption**: Created multiple headers for same section (e.g., one for prompts, another for summary), causing visual duplication
- **Correction**: Consolidated all section logic into single header block, removing duplicate headers

- **Recommendation**:
    - **AI Agent Rule**: 
        - **Action**: MODIFY existing rule file
        - **Rule File Path**: `.cursor/rules/formatting/terminal-output.mdc`
        - **Rule Text**: "**CRITICAL**: Each functional section (e.g., 'Local Repository', 'Remote Repository') MUST have only ONE header that wraps ALL related functionality (prompts, checks, summaries, confirmations). NEVER create multiple headers for different parts of the same section. Consolidate all section logic within a single `with write_header()` block."
        - **Section**: "HEADER CODE BLOCKS AND REGION INDENTING > Header Functions"
    - Document single header per section pattern in terminal output formatting rules

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

