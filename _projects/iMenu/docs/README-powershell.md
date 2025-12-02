# PowerShell Support for prompt-wizard

The `prompt-wizard` Go executable can now be used from both **bash scripts** and **PowerShell scripts**.

## Quick Start

### 1. Source the PowerShell Functions

```powershell
# Source the helper functions
. .\wizard.ps1
```

### 2. Use the Wizard Functions

```powershell
# Simple input
$json = '[{"type":"input","title":"What is your name?","key":"name"}]'
$result = iwizard-RunInline -JsonString $json
$parsed = $result | ConvertFrom-Json
Write-Host "Hello, $($parsed.name)"
```

## Functions

### `iwizard-RunInline`

Run wizard with an inline JSON string.

**Usage:**
```powershell
iwizard-RunInline -JsonString '<json-string>' [-ResultFile 'path/to/result.json']
```

**Example:**
```powershell
$json = @'
[
  {
    "type": "select",
    "title": "Choose a color",
    "key": "color",
    "options": ["Red", "Blue", "Green"]
  }
]
'@

$result = iwizard-RunInline -JsonString $json
$parsed = $result | ConvertFrom-Json
Write-Host "You chose: $($parsed.color)"
```

### `iwizard-RunJson`

Run wizard with JSON input (auto-detects file path vs JSON string).

**Usage:**
```powershell
iwizard-RunJson -JsonInput '<json-string>' [-ResultFile 'path/to/result.json']
iwizard-RunJson -JsonInput '/path/to/file.json' [-ResultFile 'path/to/result.json']
```

**Example:**
```powershell
# From JSON string
$result = iwizard-RunJson -JsonInput '[{"type":"input","title":"Name","key":"name"}]'

# From file
$result = iwizard-RunJson -JsonInput ".\wizard-example.json"
```

## JSON Format

The JSON format is the same as for bash scripts:

```json
[
  {
    "type": "input",
    "title": "What is your name?",
    "description": "Enter your full name",
    "key": "name",
    "placeholder": "John Doe",
    "default": "Anonymous"
  },
  {
    "type": "select",
    "title": "Choose a color",
    "description": "Select one option",
    "key": "color",
    "options": ["Red", "Blue", "Green"]
  },
  {
    "type": "multiselect",
    "title": "Select hobbies",
    "description": "You can select multiple",
    "key": "hobbies",
    "options": ["Reading", "Gaming", "Sports"]
  },
  {
    "type": "confirm",
    "title": "Continue?",
    "description": "Final confirmation",
    "key": "continue"
  }
]
```

## Step Types

- **`input`**: Text input field
- **`select`**: Single selection from options
- **`multiselect`**: Multiple selections from options
- **`confirm`**: Yes/No confirmation

## Result Format

The wizard returns JSON with the results keyed by the `key` field from each step:

```json
{
  "name": "John Doe",
  "color": "Blue",
  "hobbies": ["Reading", "Gaming"],
  "continue": "yes"
}
```

**Note**: Confirm steps return string values `"yes"`/`"no"`, not boolean `true`/`false`. See [Wizard Return Value Types](#wizard-return-value-types) for details.

## Examples

### Example 1: Simple Input

```powershell
. .\wizard.ps1

$json = '[{"type":"input","title":"What is your name?","key":"name"}]'
$result = iwizard-RunInline -JsonString $json
$parsed = $result | ConvertFrom-Json
Write-Host "Hello, $($parsed.name)!"
```

### Example 2: Multi-Step Wizard

```powershell
. .\wizard.ps1

$json = @'
[
  {"type":"input","title":"Name","key":"name"},
  {"type":"select","title":"Color","key":"color","options":["Red","Blue"]},
  {"type":"confirm","title":"Continue?","key":"continue"}
]
'@

$result = iwizard-RunInline -JsonString $json
$parsed = $result | ConvertFrom-Json

# Handle confirm step return value (string "yes"/"no", not boolean)
if ($parsed.continue -eq "yes") {
    Write-Host "$($parsed.name) chose $($parsed.color)"
}
```

### Example 3: Save Results to File

```powershell
. .\wizard.ps1

$json = '[{"type":"input","title":"Repository name","key":"repo"}]'
$result = iwizard-RunInline -JsonString $json -ResultFile ".\result.json"

# Results are saved to result.json
$parsed = Get-Content ".\result.json" | ConvertFrom-Json
Write-Host "Repository: $($parsed.repo)"
```

## Auto-Build

The functions automatically build the `prompt-wizard` executable if it doesn't exist:

- On **Windows**: Builds `prompt-wizard.exe`
- On **Linux/Mac**: Builds `prompt-wizard`

The executable is built in the same directory as the PowerShell functions script.

## Comparison: Bash vs PowerShell

| Feature | Bash | PowerShell |
|---------|------|------------|
| Source functions | `source wizard.sh` | `. .\wizard.ps1` |
| Run inline | `iwizard_run_inline '<json>'` | `iwizard-RunInline -JsonString '<json>'` |
| Run from file | `iwizard_run_json '/path/to/file.json'` | `iwizard-RunJson -JsonInput '/path/to/file.json'` |
| Result file | `iwizard_run_inline '<json>' result.json` | `iwizard-RunInline -JsonString '<json>' -ResultFile 'result.json'` |
| Parse result | `echo "$result" \| jq` | `$result \| ConvertFrom-Json` |

## Demo Script

Run the demo script to see examples:

```powershell
.\demo-powershell.ps1
```

## Notes

- The Go executable is built automatically on first use if it doesn't exist
- The executable name is auto-detected (`prompt-wizard.exe` on Windows, `prompt-wizard` on Linux/Mac)
- Results are returned as JSON strings that can be parsed with `ConvertFrom-Json`
- The wizard runs interactively in the terminal (TUI interface)

## Common Pitfalls and Solutions

### PowerShell ConvertTo-Json Single Item Array Issue

PowerShell's `ConvertTo-Json` cmdlet creates a JSON object `{}` when converting a single-item array, instead of a JSON array `[{}]`. This causes parsing errors in the wizard executable which expects an array structure.

**Solution**: Always check if the JSON output starts with `{` (object) instead of `[` (array) and wrap it:

```powershell
$stepsArray = @($Steps)
$jsonContent = $stepsArray | ConvertTo-Json -Depth 10
$jsonContent = $jsonContent.Trim()

# PowerShell creates an object {} for single items instead of array [{}]
if ($jsonContent.StartsWith('{')) {
    $jsonContent = "[$jsonContent]"
}
```

**Example**:
```powershell
# Single step - PowerShell creates object, not array
$step = @{type="input"; title="Name"; key="name"}
$json = $step | ConvertTo-Json  # Results in: {"type":"input",...}
# Fix:
if ($json.StartsWith('{')) {
    $json = "[$json]"  # Now: [{"type":"input",...}]
}
```

### Execution Context Considerations

When calling the wizard from within PowerShell **functions**, the execution context can affect console attachment. The wizard may stall or fail to display when called using the direct call operator `&` from function context.

**Solution**: Use `Start-Process` with `-Wait -NoNewWindow -PassThru` when calling from functions:

```powershell
# Script level - direct call works fine
& $wizardBin $stepsFile --result-file $resultFile

# Function level - use Start-Process
$process = Start-Process -FilePath $wizardBin `
    -ArgumentList $stepsFile, "--result-file", $resultFile `
    -Wait -NoNewWindow -PassThru
$exitCode = $process.ExitCode
```

**Why**: The `-NoNewWindow` flag ensures the wizard attaches to the current console, allowing proper TUI interaction. The direct call operator `&` may not properly attach stdin/stdout when called from function context.

### Using Temp Files for JSON (Recommended)

While the wizard executable can accept JSON directly as a string argument, **PowerShell's argument parsing can cause issues** with special characters, quotes, and escaping. The demo files explicitly note: "Write JSON to temporary file to avoid command-line argument issues."

**Recommended Pattern**: Always use temporary files for passing JSON:

```powershell
# Create temp file with UTF-8 no BOM encoding
$stepsFile = [System.IO.Path]::GetTempFileName()
$utf8NoBom = New-Object System.Text.UTF8Encoding $false
[System.IO.File]::WriteAllText($stepsFile, $jsonContent, $utf8NoBom)

# Pass temp file to wizard
& $wizardBin $stepsFile --result-file $resultFile

# Clean up in finally block
Remove-Item $stepsFile -Force -ErrorAction SilentlyContinue
```

**Why**: Avoids PowerShell argument parsing issues with special characters, quotes, and complex JSON structures. More reliable and matches the pattern used in working demo files.

### Wizard Return Value Types

The wizard returns different value types depending on the step type:

- **`input`**: String value
- **`select`**: String value (selected option)
- **`multiselect`**: Array of strings
- **`confirm`**: **String "yes"/"no"** (not boolean!)

**Important**: Confirm steps return **string values "yes"/"no"**, not boolean `true`/`false`. Handle this in your code:

```powershell
$result = $wizardResult | ConvertFrom-Json

# Correct handling of confirm step
if ($result.continue -is [bool]) {
    $shouldContinue = $result.continue
} elseif ($result.continue -is [string]) {
    $value = $result.continue.ToLower()
    $shouldContinue = $value -eq "true" -or $value -eq "yes" -or $value -eq "y"
}
```

**Example Return Values**:
```json
{
  "name": "John Doe",           // string
  "color": "Blue",              // string
  "hobbies": ["Reading", "Gaming"],  // array
  "continue": "yes"             // string, not boolean!
}
```

### PowerShell Function Return Types

When using `Write-Output` in PowerShell functions, multi-line strings can be returned as **arrays** instead of single strings. This causes type conversion errors when passing results to functions expecting string parameters.

**Solution**: Explicitly cast return values to `[string]`:

```powershell
function Invoke-Wizard {
    # ... wizard invocation ...
    $result = Get-Content -Path $resultFilePath -Raw
    return [string]$result  # Explicit cast to string
}
```

**Alternative**: Handle array conversion in calling code:

```powershell
$wizardResult = Invoke-Wizard -Steps $steps
$jsonString = if ($wizardResult -is [array]) {
    ($wizardResult | Out-String).Trim()
} else {
    [string]$wizardResult
}
```

### Temp File Path Handling

Temporary files created with `GetTempFileName()` already return full paths and can be used directly. **Do not use `Resolve-Path` on temp files** - it's unnecessary and can cause issues.

**Correct**:
```powershell
$stepsFile = [System.IO.Path]::GetTempFileName()
# Use directly - no Resolve-Path needed
& $wizardBin $stepsFile --result-file $resultFile
```

**Incorrect**:
```powershell
$stepsFile = [System.IO.Path]::GetTempFileName()
$stepsFile = (Resolve-Path $stepsFile).Path  # Unnecessary and can cause issues
```

**When to use `Resolve-Path`**: Only for executables or user-provided paths that need validation, not for temp files.

### Path Resolution Strategy

For robust wizard executable discovery, check multiple locations:

1. Script directory
2. Parent directory (for nested script structures)
3. Provided path parameter

**Implementation Pattern**:
```powershell
$scriptDir = if ($PSScriptRoot) { $PSScriptRoot } else { Split-Path -Parent $MyInvocation.PSCommandPath }
$pathsToCheck = @()

if ($scriptDir) {
    $pathsToCheck += Join-Path $scriptDir "prompt-wizard.exe"
    $pathsToCheck += Join-Path $scriptDir "prompt-wizard"
    $parentDir = Split-Path -Parent $scriptDir
    if ($parentDir) {
        $pathsToCheck += Join-Path $parentDir "prompt-wizard.exe"
        $pathsToCheck += Join-Path $parentDir "prompt-wizard"
    }
}

$wizardBin = $null
foreach ($path in $pathsToCheck) {
    if (Test-Path $path) {
        $wizardBin = (Resolve-Path $path).Path
        break
    }
}

if (-not $wizardBin) {
    Write-Error "Wizard executable not found. Checked:`n$($pathsToCheck -join "`n")"
}
```

This makes wizard integration portable across different script organization structures.

## Troubleshooting

### Wizard Hangs or Doesn't Display

**Symptom**: Wizard is called but doesn't display or hangs silently.

**Solutions**:
1. If calling from a function, use `Start-Process` with `-NoNewWindow` instead of direct call operator `&`
2. Ensure no console output immediately precedes the wizard call
3. Verify the wizard executable path is correct
4. Check that the JSON file is valid and properly formatted

### "Cannot unmarshal object into Go value" Error

**Symptom**: Error when wizard tries to parse JSON steps.

**Solution**: Ensure single-item arrays are wrapped in array brackets. See [PowerShell ConvertTo-Json Single Item Array Issue](#powershell-convertto-json-single-item-array-issue).

### "Cannot convert value to type System.String" Error

**Symptom**: Error when passing wizard result to another function.

**Solution**: Explicitly cast function return values to `[string]` or handle array conversion. See [PowerShell Function Return Types](#powershell-function-return-types).

### Confirm Step Always Returns False

**Symptom**: Confirm step results don't match user selection.

**Solution**: Confirm steps return "yes"/"no" strings, not booleans. Handle string comparison. See [Wizard Return Value Types](#wizard-return-value-types).

### Wizard Executable Not Found

**Symptom**: Error message about wizard executable not being found.

**Solution**: Implement comprehensive path resolution checking script directory, parent directory, and provided paths. See [Path Resolution Strategy](#path-resolution-strategy).

