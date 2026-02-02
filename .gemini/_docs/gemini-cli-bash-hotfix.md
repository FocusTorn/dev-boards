# Hot-Fix: Support Git Bash as Default Shell on Windows

## Problem Statement
The Gemini CLI currently hardcodes `powershell.exe` as the default shell for all Windows environments. While it checks `ComSpec`, it only accepts `powershell.exe` or `pwsh.exe`. If any other shell is detected (like Git Bash or CMD), it defaults back to Windows PowerShell 5.1. This causes significant issues with Unix-style commands, escaping, and modern shell features like `&&`.

## Root Cause Analysis
In `@google/gemini-cli-core/dist/src/utils/shell-utils.js`, the `getShellConfiguration()` function is implemented as follows:

```javascript
export function getShellConfiguration() {
    if (isWindows()) {
        const comSpec = process.env['ComSpec'];
        if (comSpec) {
            const executable = comSpec.toLowerCase();
            if (executable.endsWith('powershell.exe') ||
                executable.endsWith('pwsh.exe')) {
                return {
                    executable: comSpec,
                    argsPrefix: ['-NoProfile', '-Command'],
                    shell: 'powershell',
                };
            }
        }
        // Default to PowerShell for all other Windows configurations.
        return {
            executable: 'powershell.exe',
            argsPrefix: ['-NoProfile', '-Command'],
            shell: 'powershell',
        };
    }
    return { executable: 'bash', argsPrefix: ['-c'], shell: 'bash' };
}
```

## Proposed Solution
Patch the local installation of `@google/gemini-cli-core` to respect the `SHELL` environment variable. If `SHELL` points to a `bash` or `sh` executable, the CLI should treat it as a bash environment even on Windows.

### Proposed Code Change
```javascript
export function getShellConfiguration() {
    // Check for SHELL override first (supports Git Bash on Windows)
    const shellEnv = process.env['SHELL'];
    if (shellEnv && (shellEnv.toLowerCase().endsWith('bash.exe') || shellEnv.toLowerCase().endsWith('sh.exe') || shellEnv.toLowerCase().endsWith('bash') || shellEnv.toLowerCase().endsWith('sh'))) {
        return { executable: shellEnv, argsPrefix: ['-c'], shell: 'bash' };
    }

    if (isWindows()) {
        const comSpec = process.env['ComSpec'];
        // ... (rest of original logic)
    }
    // ...
}
```

## Implementation Plan
1. **Locate Target File**: Confirm path to `C:/Users/slett/AppData/Roaming/npm/node_modules/@google/gemini-cli/node_modules/@google/gemini-cli-core/dist/src/utils/shell-utils.js`.
2. **Apply Patch**: Use `replace` or `write_file` to update the `getShellConfiguration` function.
3. **Verify**: Run a command like `ls -la && echo "Success"` without any manual wrapping.

## Benefits
- Eliminates manual `pwsh -c` or `bash -c` wrapping.
- Prevents complex double-escaping issues.
- Aligns tool behavior with the project's `.gemini/rules/tool-persistence.md` (GitBash Preference).
