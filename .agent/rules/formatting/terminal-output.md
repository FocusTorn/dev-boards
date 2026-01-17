---
trigger: always_on
---

# Terminal Output Formatting Rules

## **CRITICAL EXECUTION DIRECTIVE**

**AI Agent Directive**: Follow terminal output formatting rules exactly for all command-line scripts and terminal interfaces.

**MANDATORY EXECUTION PROTOCOL**:

1. **NO DEVIATION**: All formatting rules must be followed exactly as written
2. **NO SKIPPING**: No steps may be skipped, abbreviated, or modified
3. **NO SELECTIVE COMPLIANCE**: All rules apply to all terminal output
4. **FAILURE TO COMPLY**: Violating these rules constitutes a critical protocol violation

## **PLATFORM-SPECIFIC IMPLEMENTATION**

### **Decision Matrix**

| Platform | Implementation |
|----------|---------------|
| Python | Use `outerm` package (preferred) |
| Non-Python | Use ANSI codes matching `outerm` output |

---

## **PYTHON IMPLEMENTATION (outerm Package)**

### **1. :: Installation and Import**

**✅ CORRECT - Import from outerm**:

```python
from outerm import (
    # Status message functions
    error,
    warning,
    info,
    success,
    action,
    highlight,
    dim,
    
    # Header functions
    write_header,
    write_header_fat,
    write_header_double,
    write_boxed_header,
    
    # Region management
    start_region,
    end_region,
    get_region_indent,
    
    # Palette for direct color access
    Palette,
)
```

**❌ INCORRECT - Don't use raw ANSI codes in Python**:

```python
# Wrong: Using raw ANSI codes when outerm is available
print("\x1B[38;5;196m✗\x1B[0m Error message")

# Wrong: Using legacy constants
COLOR_ERROR = "\x1B[38;5;196m"
print(f"{COLOR_ERROR}✗ Error{COLOR_RESET}")
```

### **2. :: Status Message Functions**

**✅ CORRECT - Use outerm status functions**:

```python
from outerm import error, warning, info, success, action, highlight, dim

# Error message (red ✗)
print(error("ERROR: Something went wrong"))
print(error("ERROR", "Something went wrong"))  # Colored prefix, white suffix

# Warning message (yellow ⚠) - caller must add blank line above
print()
print(warning("WARNING: Check this"))

# Info message (blue ｉ, bold icon, no space after icon)
print(info("INFO: For your information"))

# Success message (green ✔)
print(success("SUCCESS: Operation completed"))

# Action message (teal ⮻)
print(action("ACTION: Doing something"))

# Highlight (gold text, no icon)
print(highlight("Important value"))

# Dim (gray text, no icon)
print(dim("Secondary information"))
```

**Function Signatures**:

```python
def error(colored_text: str, non_colored_text: str = '') -> str: ...
def warning(colored_text: str, non_colored_text: str = '') -> str: ...
def info(colored_text: str, non_colored_text: str = '') -> str: ...
def success(colored_text: str, non_colored_text: str = '') -> str: ...
def action(colored_text: str, non_colored_text: str = '') -> str: ...
def highlight(colored_text: str, non_colored_text: str = '') -> str: ...
def dim(colored_text: str, non_colored_text: str = '') -> str: ...
```

### **3. :: Header Functions**

**✅ CORRECT - Use outerm header functions with context managers**:

```python
from outerm import write_header, write_header_fat, write_header_double, write_boxed_header

# Standard header (┌─) with automatic indentation
with write_header("Step 1: Configuration"):
    print("This content is automatically indented")
    print(success("Configuration complete"))

# Fat header (┏━) for main sections
with write_header_fat("Main Section"):
    print("Content under main section")
    with write_header("Nested Section"):
        print("Nested content")

# Double-line header (╔═) for prominent sections
with write_header_double("Important Section"):
    print("Content here")

# Boxed header for titles
write_boxed_header("Application Title", width=80)
```

**Header Width Auto-Adjustment**:

- Base width: 75 characters
- Reduces by 10 characters per nesting level (75 → 65 → 55 → ...)
- Minimum width: 35 characters

**Header Styles**:

| Function | Box Chars | Color | Use Case |
|----------|-----------|-------|----------|
| `write_header()` | `┌─` / `└─` | Beige (code 27) | Standard sections |
| `write_header_fat()` | `┏━` / `┗━` | Warm gray (code 27) | Main sections |
| `write_header_double()` | `╔═` / `╚═` | Beige (code 27) | Prominent sections |
| `write_boxed_header()` | `┏┓┃┗┛` | Cyan (code 144) | Top-level titles |

### **4. :: Region Management**

**✅ CORRECT - Use regions for automatic indentation**:

```python
from outerm import start_region, get_region_indent, get_nesting_level

# Context manager for regions
with start_region("Processing"):
    print("Indented by 2 spaces")
    with start_region("Sub-process"):
        print("Indented by 4 spaces")

# Get current indentation string
indent = get_region_indent()

# Get nesting level (number of active regions)
level = get_nesting_level()
```

### **5. :: Direct Palette Access**

**✅ CORRECT - Access colors directly from Palette**:

```python
from outerm import Palette

# Access ANSI color codes
error_color = Palette.ERROR['ansi']
success_icon = Palette.SUCCESS['iChar']
reset = Palette.RESET

# Build custom formatted output
print(f"{error_color}Custom error text{reset}")
```

**Palette Structure**:

```python
Palette.ERROR = {
    'code': 196,           # ANSI 256-color code
    'ansi': '\x1B[38;5;196m',  # Generated ANSI escape
    'rgb': (255, 0, 0),
    'hex': '#ff0000',
    'iChar': '✗',          # Icon character (U+2717)
    'iBold': False,        # Icon is not bold
    'iSpacing': ' ',       # Space after icon
}
```

---

## **NON-PYTHON IMPLEMENTATION (ANSI Codes)**

For non-Python CLI tools, use these ANSI codes to match `outerm` output exactly.

### **1. :: Color Constants**

**Status Message Colors (ANSI 256-color)**:

| Type | Code | ANSI Escape | Hex | RGB |
|------|------|-------------|-----|-----|
| ERROR | 196 | `\x1B[38;5;196m` | #ff0000 | (255, 0, 0) |
| WARNING | 220 | `\x1B[38;5;220m` | #ffd700 | (255, 215, 0) |
| INFO | 39 | `\x1B[38;5;39m` | #0099ff | (0, 153, 255) |
| SUCCESS | 46 | `\x1B[38;5;46m` | #00ff00 | (0, 255, 0) |
| ACTION | 66 | `\x1B[38;5;66m` | - | - |
| HIGHLIGHT | 179 | `\x1B[38;5;179m` | #d7af5f | (215, 175, 95) |
| DIM | 240 | `\x1B[38;5;240m` | - | - |

**Header Colors**:

| Type | Code | ANSI Escape | Use Case |
|------|------|-------------|----------|
| MAIN_TITLE | 144 | `\x1B[38;5;144m` | Top-level boxed headers |
| HEADER_FAT | 27 | `\x1B[38;5;27m` | Main section headers |
| HEADER_MINOR | 27 | `\x1B[38;5;27m` | Standard section headers |

**Control Codes**:

| Code | ANSI Escape | Purpose |
|------|-------------|---------|
| RESET | `\x1B[0m` | Reset all formatting |
| BOLD | `\x1B[1m` | Bold text |
| DIM | `\x1B[2m` | Dim/faded text |

### **2. :: Icon Specifications**

| Type | Icon | Unicode | Bold | Spacing After |
|------|------|---------|------|---------------|
| ERROR | ✗ | U+2717 | No | Space |
| WARNING | ⚠ | U+26A0 | No | Space |
| INFO | ｉ | U+FF49 (full-width) | **Yes** | **No space** |
| SUCCESS | ✔ | U+2714 (heavy) | No | Space |
| ACTION | ⮻ | U+2BBB | No | Space |

### **3. :: Message Format Patterns**

**Error Message**:

```
\x1B[38;5;196m✗\x1B[0m {text}
```

**Warning Message** (requires blank line above):

```
[blank line]
\x1B[38;5;220m⚠ {text}\x1B[0m
```

Note: Warning colors the entire line (icon + text), not just the icon.

**Info Message** (bold icon, no space):

```
\x1B[1m\x1B[38;5;39mｉ\x1B[0m\x1B[38;5;39m{text}\x1B[0m
```

**Success Message**:

```
\x1B[38;5;46m✔\x1B[0m {text}
```

### **4. :: Header Format Patterns**

**Standard Header (write_header)**:

```
\x1B[38;5;27m┌─ {title} ─────────────────────────────────────────────────────────\x1B[0m
```

Width: 75 chars at base, reduces by 10 per nesting level.

**Fat Header (write_header_fat)**:

```
\x1B[1m\x1B[38;5;27m┏━ {title} ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\x1B[0m
```

**Boxed Header (write_boxed_header)**:

```
\x1B[38;5;144m┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓\x1B[0m
\x1B[38;5;144m┃                                  {title}                                  ┃\x1B[0m
\x1B[38;5;144m┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛\x1B[0m
```

---

## **FORMATTING RULES**

### **1. :: Warning Message Requirements**

- **MUST** have blank line above
- **MUST** color entire line (icon + text), not just icon
- Use warning for "partially configured" or "needs attention" states

### **2. :: Info Message Requirements**

- Icon **MUST** be bold
- **NO SPACE** after icon (unlike other message types)
- Rest of text is normal weight

### **3. :: Color Reset Requirements**

- **ALWAYS** reset color after colored text
- Use `\x1B[0m` (or `Palette.RESET` in Python)
- Missing reset will bleed color into subsequent output

### **4. :: Region Indentation**

- 2 spaces per nesting level
- Headers auto-adjust width based on nesting
- Content within regions is automatically indented

---

## **ANTI-PATTERNS**

### **❌ Python Anti-Patterns**

- ❌ **Raw ANSI Codes** - Don't use raw ANSI codes when `outerm` is available
- ❌ **Legacy Constants** - Don't define `COLOR_*` constants, use `outerm`
- ❌ **Manual Formatting** - Don't manually format status messages
- ❌ **Manual Indentation** - Don't manually indent, use regions

### **❌ General Anti-Patterns**

- ❌ **Missing Color Reset** - Always reset after colored text
- ❌ **Wrong Icon** - Use correct Unicode characters (✔ not ✓)
- ❌ **Info Icon Spacing** - Info has NO space after icon
- ❌ **Warning Without Blank Line** - Always add blank line above warnings
- ❌ **Partial Warning Coloring** - Color entire warning line, not just icon
- ❌ **Wrong Color Codes** - Use exact codes from this specification
- ❌ **Mixing Colors and Icons** - Don't use error color with success icon

---

## **QUALITY GATES**

### **Python Implementation**

- [ ] Uses `outerm` package for all terminal output
- [ ] No raw ANSI codes in Python files
- [ ] Headers use context managers for automatic indentation
- [ ] Regions used for nested content indentation

### **Non-Python Implementation**

- [ ] ANSI codes match `outerm` specifications exactly
- [ ] Icons match Unicode characters specified
- [ ] Info icon is bold with no space after
- [ ] Warning has blank line above and full-line coloring
- [ ] All colored text is reset properly

### **General**

- [ ] Color-coded status messages distinguish success/warning/error/info
- [ ] Consistent icon spacing (space after, except info)
- [ ] Header widths follow nesting rules
- [ ] Professional, readable terminal output

---

## **SUCCESS METRICS**

After implementing proper terminal output formatting:

- ✅ **Consistent Cross-Platform** - Python and non-Python output looks identical
- ✅ **Clear Status Indication** - Success/warning/error/info clearly distinguished
- ✅ **Professional Appearance** - Clean, readable terminal output
- ✅ **Correct Icon Formatting** - All icons use correct spacing and styling
- ✅ **Proper Indentation** - Regions and headers create clear visual hierarchy
- ✅ **No Color Bleeding** - All colors properly reset
