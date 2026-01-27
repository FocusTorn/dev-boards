https://github.com/google-gemini/gemini-cli/blob/main/docs/index.md


# Gemini CLI Settings Guide

This document provides a comprehensive overview of the settings available in the Gemini CLI, their descriptions, and the hierarchy of how they are managed and persisted.

## Settings Hierarchy

Gemini CLI uses a hierarchical configuration system where settings from more specific scopes override those from more general ones.

1.  **Defaults (Built-in):** These are the baseline settings hardcoded into the Gemini CLI.
2.  **System Settings:** Machine-wide configurations that apply to all users on the system.
    *   **Path:** `C:\ProgramData\gemini-cli\settings.json` (Windows) or `/etc/gemini-cli/settings.json` (macOS/Linux).
3.  **User Settings:** Global configurations that apply to all projects for the current user.
    *   **Path:** `%USERPROFILE%\.gemini\settings.json` (Windows) or `~/.gemini/settings.json` (macOS/Linux).
4.  **Workspace Settings:** Project-specific configurations that apply only to the current working directory or repository.
    *   **Path:** `.gemini\settings.json` (relative to the project root).
5.  **Environment Variables:** Certain settings can be overridden by environment variables (e.g., `GEMINI_API_KEY`), which typically take the highest precedence.

---

## Configuration Management

Settings can be viewed and modified using the `/settings` command within the Gemini CLI. When changing a setting, you can choose which scope (**System**, **User**, or **Workspace**) to save the value to.

## Tool Approvals & Policy Engine

When you approve a command with "Allow for all future sessions," the Gemini CLI persists this approval through its **Policy Engine**.

- **Storage Location:** `%USERPROFILE%\.gemini\policies\auto-saved.toml`
- **Format:** TOML (Tom's Obvious, Minimal Language).
- **How it Works:** Each permanent approval creates a `[[rule]]` entry in the policy file. These rules specify the `toolName` (e.g., `write_file`, `run_shell_command`) and the `decision` (usually `allow`).
- **Granularity:** For generic tools like `run_shell_command`, the engine can store specific `commandPrefix` filters (e.g., allowing only `cargo` or `ls` commands automatically).
- **Priority:** Rules have a `priority` value (default is `100`) that determine which rule wins if multiple match a single request.

### Session vs. Permanent Approvals
1.  **This Time:** Approval lasts only for the single execution of that specific command.
2.  **This Session:** Approval is cached in memory and lasts until you exit the current Gemini CLI session.
3.  **All Sessions:** Approval is written to the `auto-saved.toml` policy file and persists permanently.

- **Atomic Saves:** When a setting is updated via the `/settings` UI or CLI commands, the corresponding `settings.json` file is overwritten atomically to prevent corruption.
- **Immediate Effect:** Most UI and tool-related settings take effect immediately. However, core engine settings (like model parameters or security overrides) may require a session restart or a new model turn to be fully active.
- **Manual Editing:** While it is possible to manually edit `settings.json` files, it is recommended to use the `/settings` command to ensure the JSON structure remains valid and compatible with the current version of the CLI.
- **Precedence Logic:** Upon startup, the CLI loads the default settings, then layers the **System** settings, followed by the **User** settings, and finally the **Workspace** settings. The last value loaded for any given key is the one used during the session.




---

## Available Settings

### General
| Key | Description | Default |
| :--- | :--- | :--- |
| `general.previewFeatures` | Enable experimental/preview features (e.g., preview models). | `false` |
| `general.vimMode` | Enable Vim keybindings in the terminal. | `false` |
| `general.disableAutoUpdate` | Disable automatic checks for CLI updates. | `false` |
| `general.enablePromptCompletion` | Enable AI-powered prompt completion suggestions while typing. | `false` |
| `general.debugKeystrokeLogging` | Enable debug logging of keystrokes to the console. | `false` |
| `general.sessionRetention.enabled` | Enable automatic session cleanup. | `false` |

### UI & Output
| Key | Description | Default |
| :--- | :--- | :--- |
| `output.format` | Format of the CLI output (`text` or `json`). | `text` |
| `ui.theme` | The visual theme for the CLI (e.g., `GitHub`). | `Default` |
| `ui.hideWindowTitle` | Hide the window title bar. | `false` |
| `ui.showStatusInTitle` | Show model thoughts/status in the terminal title bar. | `false` |
| `ui.dynamicWindowTitle` | Update title with status icons (◇, ✋, ✦). | `true` |
| `ui.hideContextSummary` | Hide the context summary (GEMINI.md, MCP) above input. | `false` |
| `ui.footer.hideCWD` | Hide the current working directory in the footer. | `false` |
| `ui.footer.hideModelInfo` | Hide model name and context usage in the footer. | `false` |
| `ui.footer.hideContextPercentage` | Hide the context window remaining percentage. | `true` |
| `ui.showMemoryUsage` | Display memory usage information in the UI. | `false` |
| `ui.showLineNumbers` | Show line numbers in the chat interface. | `true` |
| `ui.useFullWidth` | Use the entire width of the terminal for output. | `true` |
| `ui.useAlternateBuffer` | Use an alternate screen buffer (preserves shell history). | `false` |
| `ui.incrementalRendering` | Reduce flickering (requires alternate buffer). | `true` |

### IDE
| Key | Description | Default |
| :--- | :--- | :--- |
| `ide.enabled` | Enable IDE integration mode. | `false` |

### Model & Context
| Key | Description | Default |
| :--- | :--- | :--- |
| `model.maxSessionTurns` | Max turns to keep in a session (-1 for unlimited). | `-1` |
| `model.compressionThreshold` | Context usage fraction at which to trigger compression. | `0.5` |
| `model.skipNextSpeakerCheck` | Skip the next speaker check in multi-agent flows. | `true` |
| `context.discoveryMaxDirs` | Max directories to search for memory/context files. | `200` |
| `context.loadMemoryFromIncludeDirectories` | Load memory from include directories in `/memory refresh`. | `false` |
| `context.fileFiltering.respectGitIgnore` | Respect `.gitignore` patterns when searching. | `true` |
| `context.fileFiltering.respectGeminiIgnore` | Respect `.geminiignore` patterns when searching. | `true` |
| `context.fileFiltering.enableRecursiveFileSearch` | Enable recursive search for `@` file references. | `true` |
| `context.fileFiltering.disableFuzzySearch` | Disable fuzzy search when searching for files. | `false` |

### Tools
| Key | Description | Default |
| :--- | :--- | :--- |
| `tools.shell.enableInteractiveShell` | Use an interactive shell experience for shell commands. | `true` |
| `tools.shell.showColor` | Show color in shell output. | `false` |
| `tools.autoAccept` | Automatically accept "safe" tool calls (read-only). | `false` |
| `tools.useRipgrep` | Use `ripgrep` for faster file content searches. | `true` |
| `tools.enableToolOutputTruncation` | Truncate large tool outputs. | `true` |
| `tools.truncateToolOutputThreshold` | Max character limit for tool output before truncation. | `4000000` |
| `tools.truncateToolOutputLines` | Number of lines to keep when truncating tool output. | `1000` |
| `tools.disableLLMCorrection` | Fail immediately on string match failure (no self-correction). | `false` |

### Security
| Key | Description | Default |
| :--- | :--- | :--- |
| `security.disableYoloMode` | Force disable YOLO mode. | `false` |
| `security.enablePermanentToolApproval` | Enable "Allow for all future sessions" in tool prompts. | `false` |
| `security.blockGitExtensions` | Block installing/loading extensions from Git. | `false` |
| `security.folderTrust.enabled` | Enable folder trust tracking. | `false` |
| `security.environmentVariableRedaction.enabled` | Redact secrets from environment variables in logs. | `false` |

### Experimental
| Key | Description | Default |
| :--- | :--- | :--- |
| `experimental.skills` | Enable Agent Skills (extension functionality). | `false` |
| `experimental.codebaseInvestigatorSettings.enabled` | Enable the Codebase Investigator agent. | `true` |
| `experimental.codebaseInvestigatorSettings.maxNumTurns` | Max turns for Codebase Investigator. | `10` |
| `experimental.cliHelpAgentSettings.enabled` | Enable the CLI Help Agent. | `true` |
| `experimental.useOSC52Paste` | Use OSC 52 sequence for pasting (useful for SSH). | `false` |

### Hooks
| Key | Description | Default |
| :--- | :--- | :--- |
| `hooks.notifications` | Show visual indicators when hooks are executing. | `true` |

---

## Extension-Specific Settings
Some extensions (like `conductor` or `rust-agentic-skills`) may add their own settings to the `settings.json` files. These are typically stored under their respective keys (e.g., `conductor.*`).

## Troubleshooting
If settings are not taking effect:
1.  **Restart the CLI:** Most settings require a restart to be fully applied.
2.  **Check Precedence:** Ensure a Workspace setting isn't overriding your User setting.
3.  **Validate JSON:** Manual edits to `settings.json` must be valid JSON.




---

## Shown Tips

1. 
