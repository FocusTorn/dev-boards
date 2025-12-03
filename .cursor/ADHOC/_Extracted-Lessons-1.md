# GitHub Setup Script Refactoring - Lessons Learned

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

## 2.0 :: Wizard Invocation from PowerShell Functions <!-- Start Fold -->

- Learning: Invoking interactive CLI tools (like prompt-wizard.exe) from within PowerShell functions can cause stalling issues due to console attachment problems. The wizard needs proper console attachment when called from functions, not just at script level.

- Pattern: Use `Start-Process` with `-Wait -NoNewWindow -PassThru` instead of direct call operator (`&`) when invoking the wizard from within PowerShell functions. This ensures proper console attachment and prevents stalling.

- Implementation: Changed from `& $wizardBin $stepsFile $resultFile` to `Start-Process -FilePath $wizardBin -ArgumentList $stepsFileFullPath, "--result-file", $resultFilePath -Wait -NoNewWindow -PassThru` in the `Invoke-iWizard` function.

- Benefit: Prevents script stalling and ensures the wizard displays correctly when called from functions, maintaining consistent behavior whether called from script level or function level.

- **Not documented**: The difference in behavior between invoking interactive CLI tools at script level vs function level, and the need for explicit console attachment when using `Start-Process`.

- **Mistake/Assumption**: Initially assumed the wizard could be called the same way from functions as from script level. Also tried removing `Resolve-Path` on temp files thinking it was causing the stall, but the real issue was console attachment.

- **Correction**: Switched to `Start-Process` with proper flags to ensure console attachment, and kept temp file usage (which was correct) rather than trying to pass JSON directly as arguments.

- **Recommendation**:
    - **AI Agent Rule**: "ALWAYS use `Start-Process -Wait -NoNewWindow -PassThru` when invoking interactive CLI tools (like wizards) from within PowerShell functions, never use direct call operator (`&`) for such tools when called from functions."
    - Document the console attachment requirement in PowerShell wizard integration documentation
    - Add troubleshooting section covering stalling issues and console attachment

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

## 2.1 :: Wizard Result Type Handling for Confirm Steps <!-- Start Fold -->

- Learning: Wizard confirm steps can return values as strings ("yes"/"no"), booleans (true/false), or numeric values (1/0), and the parsing logic must handle all these cases to correctly interpret user selections.

- Pattern: Always convert wizard result values to string first using `.ToString().Trim()`, then check for multiple possible representations (case-insensitive): "true", "yes", "y", "1" for true, and "false", "no", "n", "0" for false. Also handle boolean types directly as a fallback.

- Implementation: Created robust parsing logic that checks type first (bool, string, numeric), converts to string, normalizes case, and checks against multiple possible values before defaulting to boolean type check.

- Benefit: Prevents false negatives when user selects "yes" but script thinks it's "no" due to type mismatches, ensuring wizard selections are correctly interpreted regardless of how the wizard returns the value.

- **Not documented**: The various return type formats from wizard confirm steps and the need for robust type-agnostic parsing. Current documentation assumes consistent return types.

- **Mistake/Assumption**: Initially assumed confirm steps would always return boolean values, leading to "create repo" selections being ignored when wizard returned "yes" as a string.

- **Correction**: Implemented comprehensive type checking that handles strings, booleans, and numeric values, with case-insensitive string matching for common true/false representations.

- **Recommendation**:
    - **AI Agent Rule**: "ALWAYS parse wizard confirm step results by first converting to string with `.ToString().Trim()`, then checking for multiple representations (case-insensitive): 'true'/'yes'/'y'/'1' for true, 'false'/'no'/'n'/'0' for false, and also handle boolean types directly."
    - Document wizard return type variations in wizard integration documentation
    - Create a helper function for parsing confirm step results that can be reused

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

## 2.2 :: Output Formatting Standards and Hierarchy <!-- Start Fold -->

- Learning: Consistent output formatting with proper color hierarchy (DarkGray for info, DarkGreen for intermediate success, Green for final success) and indentation (2 spaces for section content, 4 spaces for sub-items) creates a professional, scannable output that clearly communicates status.

- Pattern: Use a three-tier color system: `DarkGray` for informational messages and intermediate steps, `DarkGreen` for intermediate success messages (like "✓ Set Git user name"), and `Green` for final success messages (like "✓ Git repository initialized successfully"). Use consistent indentation: 2 spaces for section-level content, 4 spaces for sub-items.

- Implementation: Applied consistent formatting across all setup scripts (SSH, local repo, remote repo) with boxed headers for main sections (`Write-BoxedHeader`) and simple headers for sub-sections (`Write-Header`), with proper indentation and color coding throughout.

- Benefit: Users can quickly scan output to understand what's happening (info), what succeeded (intermediate), and what completed (final), creating a clear visual hierarchy that improves user experience.

- **Not documented**: The output formatting standards, color hierarchy, and indentation patterns are not documented, making it difficult to maintain consistency across scripts.

- **Mistake/Assumption**: Initially used bright colors (`Cyan`, `Green`) for all messages, making it hard to distinguish between informational, intermediate success, and final success messages.

- **Correction**: Established clear hierarchy: DarkGray for info, DarkGreen for intermediate success, Green for final success, with consistent 2-space/4-space indentation pattern.

- **Recommendation**:
    - **AI Agent Rule**: "ALWAYS use the three-tier color system for PowerShell script output: `DarkGray` for informational messages, `DarkGreen` for intermediate success messages (operations that completed but aren't final), and `Green` for final success messages (major milestones). Use 2-space indentation for section content and 4-space indentation for sub-items."
    - Document output formatting standards in a style guide
    - Create helper functions for consistent formatting (e.g., `Write-Info`, `Write-IntermediateSuccess`, `Write-FinalSuccess`)

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

## 2.3 :: GitHub CLI Detection and Installation Flow <!-- Start Fold -->

- Learning: GitHub CLI detection must check both installation (`Get-Command gh`) and authentication status (`gh auth status`) separately, and should provide installation prompts using `winget` when available, with fallback to manual instructions.

- Pattern: Check for GitHub CLI in two stages: first verify installation with `Get-Command gh -ErrorAction SilentlyContinue`, then check authentication with `gh auth status` using `Start-Process` with redirected output to avoid interactive prompts. If not installed, prompt user to install via `winget` if available, otherwise show manual installation instructions.

- Implementation: Created detection logic that uses `Start-Process` to check `gh auth status` exit code non-interactively, prompts for installation using `winget install GitHub.cli` when available, and provides clear fallback instructions.

- Benefit: Provides seamless installation experience when possible, while gracefully handling cases where installation isn't possible, ensuring users aren't stuck without clear next steps.

- **Not documented**: The two-stage detection pattern (install + auth), the use of `Start-Process` for non-interactive auth checking, and the installation prompt flow with `winget` integration.

- **Mistake/Assumption**: Initially only checked for `gh` command existence, assuming if it exists it's authenticated, leading to failures when CLI was installed but not authenticated.

- **Correction**: Separated installation check from authentication check, using `Start-Process` to check auth status non-interactively, and added installation prompt with `winget` support.

- **Recommendation**:
    - **AI Agent Rule**: "ALWAYS check GitHub CLI in two stages: first verify installation with `Get-Command gh`, then check authentication with `gh auth status` using `Start-Process` with redirected output to avoid interactive prompts. If not installed, prompt for installation via `winget` when available."
    - Document the two-stage detection pattern
    - Create a reusable function for GitHub CLI detection and installation prompting

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

## 2.4 :: Repository Creation Flow Pattern <!-- Start Fold -->

- Learning: The repository creation flow should ask users if they want to create a repo first, then check if it exists, then ask to remove it if it does exist. This prevents unnecessary checks and provides a clear decision path.

- Pattern: Wizard asks "Create repository on GitHub?" first. After wizard completes, if user wants to create, check if repo exists using `Test-RepoExists`. If it exists, show a second confirm step asking "Repository already exists. Remove it?" Only then proceed with removal and creation.

- Implementation: Added "Create repository on GitHub?" step to wizard, then after wizard completion, conditionally check repository existence and show removal confirmation if needed, before passing `CreateRepo` and `RemoveExisting` parameters to the remote setup script.

- Benefit: Users make the creation decision upfront, avoiding unnecessary existence checks if they don't want to create, and get a chance to confirm removal of existing repos before destructive actions.

- **Not documented**: The recommended flow pattern for repository creation (ask → check → confirm removal) and the conditional checking logic.

- **Mistake/Assumption**: Initially checked repository existence before asking if user wanted to create, and didn't provide a removal confirmation step, leading to potential data loss or confusion.

- **Correction**: Reordered flow to ask creation intent first, then conditionally check existence, then confirm removal if needed, providing clear decision points at each stage.

- **Recommendation**:
    - **AI Agent Rule**: "ALWAYS follow the repository creation flow pattern: ask user if they want to create first (in wizard), then conditionally check existence only if creation is requested, then show removal confirmation if repo exists, before proceeding with creation."
    - Document the repository creation flow pattern
    - Create a reusable function for the creation flow that can be applied to other similar scenarios

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

## 2.5 :: Error Suppression Patterns for Setup Scripts <!-- Start Fold -->

- Learning: Setup scripts that run before full configuration (like SSH setup) will encounter expected errors (permission denied, host key verification) that should be suppressed to avoid alarming users. These errors are expected and will be resolved by the setup process itself.

- Pattern: Use `$ErrorActionPreference = "SilentlyContinue"` at script level for setup scripts, redirect stdout/stderr to temp files using `Start-Process` with `-RedirectStandardOutput` and `-RedirectStandardError` for external commands, and use environment variables like `GIT_SSH_COMMAND` with non-interactive SSH options to suppress prompts and errors.

- Implementation: Applied error suppression throughout remote setup script using script-level `$ErrorActionPreference`, `Start-Process` with redirected output for `git` commands, and `GIT_SSH_COMMAND="ssh -o BatchMode=yes -o StrictHostKeyChecking=no -o ConnectTimeout=5 -o LogLevel=ERROR"` for repository existence checks.

- Benefit: Users see clean, informative output without being alarmed by expected errors that will be resolved by the setup process, creating a smoother user experience.

- **Not documented**: The pattern of suppressing expected errors in setup scripts, the use of `GIT_SSH_COMMAND` for non-interactive Git operations, and when it's appropriate to suppress vs display errors.

- **Mistake/Assumption**: Initially tried to handle SSH errors by catching and displaying them, but this alarmed users with "Permission denied" errors even though SSH setup hadn't run yet, which is expected.

- **Correction**: Implemented comprehensive error suppression for expected errors (SSH not configured, host key verification, permission denied) while still allowing actual errors to surface through proper error handling.

- **Recommendation**:
    - **AI Agent Rule**: "ALWAYS suppress expected errors in setup scripts that occur before full configuration (like SSH permission denied before SSH setup). Use `$ErrorActionPreference = 'SilentlyContinue'` at script level and redirect external command output to temp files using `Start-Process` with `-RedirectStandardOutput` and `-RedirectStandardError`."
    - Document error suppression patterns and when to use them
    - Create guidelines for distinguishing expected vs unexpected errors in setup scripts

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

## 2.6 :: Orchestrator Pattern for Multi-Script Workflows <!-- Start Fold -->

- Learning: When combining multiple related setup scripts into a unified workflow, use an orchestrator script with a single wizard that collects all inputs upfront, then passes parameters to sub-modules that no longer have their own wizards. This centralizes user interaction and ensures consistent flow.

- Pattern: Create a main orchestrator script (`ps-setup-github.ps1`) that runs a unified wizard to collect all necessary inputs, then conditionally invokes sub-modules (`ps-setup-ssh.ps1`, `ps-setup-local-repo.ps1`, `ps-setup-remote.ps1`) using parameter splatting. Sub-modules accept script-level parameters and remove their internal wizards and header displays.

- Implementation: Refactored GitHub setup into orchestrator pattern where `ps-setup-github.ps1` contains the unified wizard and calls sub-modules with hashtable parameters using splatting (`@Parameters`), while sub-modules were updated to accept parameters and remove `Write-BoxedHeader` calls and internal wizard logic.

- Benefit: Single point of user interaction, consistent flow, easier to maintain, and sub-modules can still be run standalone if needed (with their own parameter handling).

- **Not documented**: The orchestrator pattern for combining multiple scripts, parameter splatting with hashtables, and the refactoring approach for moving wizard logic to orchestrator.

- **Mistake/Assumption**: Initially tried to keep wizards in sub-modules and have orchestrator skip them, but this created complexity and inconsistent user experience.

- **Correction**: Moved all wizard logic to orchestrator, refactored sub-modules to accept parameters, and used parameter splatting to pass values cleanly between scripts.

- **Recommendation**:
    - **AI Agent Rule**: "ALWAYS use an orchestrator pattern when combining multiple related setup scripts: create a main script with unified wizard that collects all inputs, then pass parameters to sub-modules using hashtable splatting (`@Parameters`). Sub-modules should accept script-level parameters and not contain their own wizards."
    - Document the orchestrator pattern and parameter passing approach
    - Create a template for orchestrator scripts that can be reused

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

## 2.7 :: Verification Report Pattern After Setup <!-- Start Fold -->

- Learning: After completing setup operations, running comprehensive verification checks and displaying a formatted report helps users understand what's working and what needs attention, providing confidence that setup completed successfully.

- Pattern: After all setup steps complete, run verification checks (SSH auth, local repo existence/path, remote repo existence, GitHub CLI status, git-crypt status) and display results in a formatted report with clear indicators (✓ for success, ✗ for failure, ⚠ for warnings, ℹ for info).

- Implementation: Created `Show-VerificationReport` function that takes a hashtable of check results and displays them with appropriate icons and colors, called at the end of the orchestrator script after all setup steps complete.

- Benefit: Users get immediate feedback on setup status, can identify any issues that need attention, and have confidence that the setup process completed as expected.

- **Not documented**: The verification report pattern, the check functions (`Test-SshConnection`, `Get-LocalRepoPath`, `Test-RepoExists`, `Test-GitHubCli`, `Test-GitCrypt`), and when to run verification vs during setup.

- **Mistake/Assumption**: None - this was a new feature requested by the user.

- **Correction**: N/A

- **Recommendation**:
    - **AI Agent Rule**: "ALWAYS run comprehensive verification checks after setup operations complete and display a formatted report with clear indicators (✓/✗/⚠/ℹ) showing what's working and what needs attention."
    - Document the verification report pattern and check function implementations
    - Create reusable verification check functions that can be used across different setup scripts

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

## 2.8 :: Git Error Handling in Empty Repositories <!-- Start Fold -->

- Learning: Git commands like `git rev-parse HEAD` and `git branch --set-upstream-to` fail in empty repositories (no commits yet), and these errors should be handled gracefully by checking for commits first or suppressing the error output.

- Pattern: Before running Git commands that require commits, check if commits exist using `$null = git rev-parse HEAD 2>$null; $hasCommits = $LASTEXITCODE -eq 0`, then conditionally run commit-dependent commands. Alternatively, suppress error output for expected failures using `$ErrorActionPreference = "SilentlyContinue"` temporarily.

- Implementation: Added commit checks before `git branch --set-upstream-to` and wrapped `git rev-parse HEAD` calls with error suppression, displaying informative messages like "Upstream will be set on first push (no commits yet)" when appropriate.

- Benefit: Prevents alarming error messages for expected situations (empty repos), provides clear guidance on what will happen, and maintains clean output.

- **Not documented**: The pattern of checking for commits before running commit-dependent Git commands, and the use of `$LASTEXITCODE` to check command success when suppressing output.

- **Mistake/Assumption**: Initially assumed repositories would always have commits, leading to "fatal: no commit on branch 'main' yet" errors that alarmed users unnecessarily.

- **Correction**: Added commit existence checks and conditional execution of commit-dependent commands, with informative messages explaining what will happen when commits are made.

- **Recommendation**:
    - **AI Agent Rule**: "ALWAYS check for existing commits using `$null = git rev-parse HEAD 2>$null; $hasCommits = $LASTEXITCODE -eq 0` before running Git commands that require commits (like `git branch --set-upstream-to`), and provide informative messages when operations will happen later (e.g., 'Upstream will be set on first push')."
    - Document Git error handling patterns for empty repositories
    - Create helper functions for commit checking and conditional Git command execution

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

## 2.9 :: PowerShell Script Encoding Requirements <!-- Start Fold -->

- Learning: PowerShell scripts on Windows that contain Unicode characters (like checkmarks ✓, box-drawing characters ╭─╮) require UTF-8 with BOM encoding to display correctly. UTF-8 without BOM can cause display errors.

- Pattern: Use UTF-8 with BOM encoding for all PowerShell scripts that contain Unicode characters. Create `.editorconfig` file with `charset = utf-8-bom` for `*.ps1` files to enforce consistent encoding across editors.

- Implementation: Created `.editorconfig` file in `bootstraps/github/` directory with `[*.ps1]` section specifying `charset = utf-8-bom` to ensure all PowerShell scripts use the correct encoding.

- Benefit: Prevents encoding-related display errors, ensures Unicode characters render correctly, and maintains consistency across different editors and environments.

- **Not documented**: The UTF-8 BOM requirement for PowerShell scripts with Unicode characters, and the use of `.editorconfig` to enforce encoding standards.

- **Mistake/Assumption**: Initially didn't realize encoding was causing issues, thinking it was a different problem until user reported needing to convert scripts to UTF-8 BOM.

- **Correction**: Created `.editorconfig` to enforce UTF-8 BOM encoding for all PowerShell scripts, ensuring consistent encoding going forward.

- **Recommendation**:
    - **AI Agent Rule**: "ALWAYS use UTF-8 with BOM encoding for PowerShell scripts that contain Unicode characters. Ensure `.editorconfig` file specifies `charset = utf-8-bom` for `*.ps1` files."
    - Document encoding requirements for PowerShell scripts
    - Add encoding check to pre-commit hooks or linting rules

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->

## 2.10 :: Wizard UI Rendering Issues and Fixes <!-- Start Fold -->

- Learning: Wizard UI components (input field borders, confirm step yes/no visibility) can have rendering issues that require fixes in the Go source code of the wizard tool itself, not just in the PowerShell scripts that use it.

- Pattern: When wizard UI issues occur (missing borders, options not showing), the fixes must be made in the wizard's Go source code (`main.go` in the iMenu project). Use `lipgloss.RoundedBorder()` for input fields, ensure `View()` function renders all options immediately, and use `tea.Batch()` in `Init()` to force initial render.

- Implementation: Modified `main.go` to use `RoundedBorder()` for input fields, ensured confirm step yes/no options are rendered in `View()` function immediately, and added `tea.Batch()` to `Init()` to force initial render. Built and copied executable to bootstraps directory.

- Benefit: Fixes UI issues at the source, ensuring all scripts that use the wizard benefit from the fixes, and maintains consistent UI behavior.

- **Not documented**: The process of fixing wizard UI issues in the Go source code, building the executable, and copying it to the bootstraps directory. The relationship between wizard source code and PowerShell script usage.

- **Mistake/Assumption**: Initially tried to fix UI issues in PowerShell scripts, but these were rendering issues in the Go wizard tool itself that required source code changes.

- **Correction**: Identified that UI fixes needed to be in the Go source code, made changes to `main.go`, rebuilt executable, and copied to bootstraps directory.

- **Recommendation**:
    - **AI Agent Rule**: "ALWAYS fix wizard UI rendering issues (borders, option visibility) in the Go source code of the wizard tool (`main.go` in iMenu project), not in the PowerShell scripts that use it. After making changes, rebuild the executable and copy it to the bootstraps directory."
    - Document the wizard development and build process
    - Create a build script to automate rebuilding and copying the wizard executable

- **Response**: ✏️❓❌⚠️✅ No action required

---

<!-- Close Fold -->
