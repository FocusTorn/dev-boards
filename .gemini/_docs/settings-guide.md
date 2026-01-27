# Gemini CLI Settings Guide

This document provides an exhaustive overview of the settings available in the Gemini CLI, their descriptions, and the hierarchy of how they are managed and persisted.

## Settings Hierarchy

Gemini CLI uses a hierarchical configuration system where settings from more specific scopes override those from more general ones.

1.  **Defaults (Built-in):** Baseline settings hardcoded into the CLI.
2.  **System Settings:** Machine-wide configurations for all users.
    *   **Path (Windows):** `C:\ProgramData\gemini-cli\settings.json`
    *   **Path (macOS/Linux):** `/etc/gemini-cli/settings.json`
3.  **User Settings:** Global configurations for the current user across all projects.
    *   **Path (Windows):** `%USERPROFILE%\.gemini\settings.json`
    *   **Path (macOS/Linux):** `~/.gemini/settings.json`
4.  **Workspace Settings:** Project-specific configurations for the current directory or repository.
    *   **Path:** `.gemini\settings.json` (relative to the project root).
5.  **Environment Variables:** Highest precedence overrides (e.g., `GEMINI_API_KEY`).

---

## Configuration Management

Settings can be viewed and modified using the `/settings` command. Changes can be saved to **System**, **User**, or **Workspace** scopes.

## Tool Approvals & Policy Engine

Permanent approvals (e.g., "Allow for all future sessions") are stored in the Policy Engine.

- **Storage Path:** `%USERPROFILE%\.gemini\policies\auto-saved.toml`
- **Format:** TOML.
- **Granularity:** Can store specific `commandPrefix` filters for generic tools like `run_shell_command`.

---

## Exhaustive Settings List

### 1. General Settings
| Key | Description | Default |
| :--- | :--- | :--- |
| `general.previewFeatures` | Enables experimental/preview features (e.g., preview models). | `false` |
| `general.vimMode` | Enables Vim keybindings in the terminal. | `false` |
| `general.disableAutoUpdate` | Disables automatic checks for CLI updates. | `false` |
| `general.enableAutoUpdateNotification` | Enables update notification prompts. | `true` |
| `general.enablePromptCompletion` | AI-powered prompt completion suggestions while typing. | `false` |
| `general.debugKeystrokeLogging` | Logs keystrokes to the console for debugging. | `false` |
| `general.retryFetchErrors` | Retries on network fetch failures (TypeError: fetch failed). | `false` |
| `general.preferredEditor` | The default editor to open files in. | `undefined` |
| `general.sessionRetention.enabled` | Enables automatic session cleanup. | `false` |
| `general.sessionRetention.maxAge` | Max age of sessions to keep (e.g., "30d", "7d", "24h"). | `undefined` |
| `general.sessionRetention.maxCount` | Max number of most recent sessions to keep. | `undefined` |
| `general.sessionRetention.minRetention` | Safety limit: Minimum duration to keep a session. | `"1d"` |
| `general.checkpointing.enabled` | Enables session checkpointing for recovery. | `false` |

### 2. UI & Output
| Key | Description | Default |
| :--- | :--- | :--- |
| `output.format` | CLI output format (`text` or `json`). | `text` |
| `ui.theme` | Visual theme for the CLI (e.g., `GitHub`, `Default`). | `Default` |
| `ui.customThemes` | Dictionary of custom theme definitions. | `{}` |
| `ui.hideWindowTitle` | Hides the terminal window title bar. | `false` |
| `ui.showStatusInTitle` | Shows model thoughts in the terminal title bar during work. | `false` |
| `ui.dynamicWindowTitle` | Updates title with status icons (◇, ✋, ✦). | `true` |
| `ui.showHomeDirectoryWarning` | Warning when running in the home directory. | `true` |
| `ui.hideTips` | Hides helpful "Did you know?" tips. | `false` |
| `ui.hideBanner` | Hides the startup application banner. | `false` |
| `ui.hideContextSummary` | Hides the context summary block above the input. | `false` |
| `ui.hideFooter` | Hides the bottom footer entirely. | `false` |
| `ui.showMemoryUsage` | Displays memory usage in the UI. | `false` |
| `ui.showLineNumbers` | Shows line numbers in chat blocks. | `true` |
| `ui.showCitations` | Shows citations for generated text. | `false` |
| `ui.showModelInfoInChat` | Shows the model name for every model turn. | `false` |
| `ui.useFullWidth` | Uses the entire width of the terminal for output. | `true` |
| `ui.useAlternateBuffer` | Uses alternate screen buffer (preserves shell history). | `false` |
| `ui.incrementalRendering` | Reduces flickering (requires alternate buffer). | `true` |
| `ui.footer.hideCWD` | Hides the current working directory path. | `false` |
| `ui.footer.hideSandboxStatus` | Hides the sandbox status indicator. | `false` |
| `ui.footer.hideModelInfo` | Hides model name and context usage. | `false` |
| `ui.footer.hideContextPercentage` | Hides context window remaining percentage. | `true` |
| `ui.accessibility.enableLoadingPhrases` | Enables natural language loading phrases. | `true` |
| `ui.accessibility.screenReader` | Plain-text output for screen reader compatibility. | `false` |

### 3. IDE Integration
| Key | Description | Default |
| :--- | :--- | :--- |
| `ide.enabled` | Enables IDE integration mode. | `false` |
| `ide.preferredEditor` | Specifies the preferred editor for viewing diffs. | `vscode` |

### 4. Privacy
| Key | Description | Default |
| :--- | :--- | :--- |
| `privacy.usageStatisticsEnabled` | Enables collection of usage metrics. | `true` |

### 5. Context Management
| Key | Description | Default |
| :--- | :--- | :--- |
| `contextFileName` | Filename(s) for context files (e.g., `GEMINI.md`). | `GEMINI.md` |
| `context.discoveryMaxDirs` | Max directories to scan for context/memory. | `200` |
| `context.loadMemoryFromIncludeDirectories` | Refresh memory from all include directories. | `false` |
| `context.fileFiltering.respectGitIgnore` | Respects `.gitignore` when discovering files. | `true` |
| `context.fileFiltering.respectGeminiIgnore` | Respects `.geminiignore` when discovering files. | `true` |
| `context.fileFiltering.enableRecursiveFileSearch` | Recursive search for `@` file references. | `true` |
| `context.fileFiltering.disableFuzzySearch` | Disables fuzzy search for file completion. | `false` |

### 6. Tools & Execution
| Key | Description | Default |
| :--- | :--- | :--- |
| `tools.autoAccept` | Automatically accept "safe" (read-only) tool calls. | `false` |
| `tools.useRipgrep` | Use `ripgrep` for optimized file searches. | `true` |
| `tools.enableToolOutputTruncation` | Truncate large tool outputs. | `true` |
| `tools.truncateToolOutputThreshold` | Character limit before truncation. | `4000000` |
| `tools.truncateToolOutputLines` | Lines to keep when truncating. | `1000` |
| `tools.disableLLMCorrection` | No self-correction on string match failure. | `false` |
| `tools.shell.executable` | Default shell executable (e.g., `pwsh.exe`, `bash`). | OS Default |
| `tools.shell.enableInteractiveShell` | Interactive experience for shell commands. | `true` |
| `tools.shell.showColor` | Show color in shell output. | `false` |
| `tools.sandbox` | Sandboxing mode (`true`, `docker`, `podman`). | `false` |
| `tools.toolDiscoveryCommand` | Custom command for project tool discovery. | `""` |
| `tools.toolCallCommand` | Custom command for calling discovered tools. | `""` |

### 7. Model Parameters
| Key | Description | Default |
| :--- | :--- | :--- |
| `model.maxSessionTurns` | Max turns to keep in context (-1 = unlimited). | `-1` |
| `model.compressionThreshold` | Fraction of context used before compression triggers. | `0.5` |
| `model.skipNextSpeakerCheck` | Skip check in multi-agent flows. | `true` |

### 8. Security
| Key | Description | Default |
| :--- | :--- | :--- |
| `security.disableYoloMode` | Force disable YOLO mode. | `false` |
| `security.enablePermanentToolApproval` | Enable "Allow for all future sessions" option. | `false` |
| `security.blockGitExtensions` | Block extensions loaded via Git. | `false` |
| `security.folderTrust.enabled` | Enable folder trust tracking. | `false` |
| `security.environmentVariableRedaction.enabled` | Redact secrets from variables in logs. | `false` |

### 9. Experimental & Agents
| Key | Description | Default |
| :--- | :--- | :--- |
| `experimental.skills` | Enables Agent Skills (extensible logic). | `false` |
| `experimental.useOSC52Paste` | Use OSC 52 sequence for clipboard (SSH). | `false` |
| `experimental.codebaseInvestigatorSettings.enabled` | Enable Codebase Investigator agent. | `true` |
| `experimental.codebaseInvestigatorSettings.maxNumTurns` | Max turns for investigator. | `10` |
| `experimental.cliHelpAgentSettings.enabled` | Enable CLI Help agent. | `true` |

### 10. Workspace Configuration
| Key | Description | Default |
| :--- | :--- | :--- |
| `coreTools.includeDirectories` | Define a multi-directory workspace. | `[]` |
| `excludeTools` | List of core tools to forbid the model from using. | `[]` |

---

## Extension-Specific Settings
Extensions like `conductor` store settings under their own keys:
- `conductor.autoSync`: (Example) Automatically sync tracks.
- `conductor.defaultModel`: (Example) Preferred model for planning.

---

## Troubleshooting
1. **Restart Required**: Most settings require a restart to apply.
2. **Precedence**: Workspace settings override User settings.
3. **JSON Syntax**: Manual edits must be valid JSON (use a linter).