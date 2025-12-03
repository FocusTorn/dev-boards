# pyprompt

Beautiful interactive prompts using `prompt_toolkit` - extracted from `git-py` for standalone use.

## Features

- **Text prompts** - Get text input from users with optional defaults
- **Select prompts** - Interactive menu selection with arrow keys
- **Confirm prompts** - Yes/No confirmation with immediate keypress response
- **Beautiful styling** - Pink qmarks, white questions, blue answers
- **Optional region indentation** - Integrate with terminal output systems

## Installation

### Standard Installation (pip)

The package uses standard Python packaging tools and can be installed with `pip`:

```bash
# Install from local directory (development/editable mode)
pip install -e .

# Or install as a regular package
pip install .
```

### Using in a Virtual Environment (Recommended)

```bash
# Create virtual environment
python -m venv venv

# Activate virtual environment
# On Windows:
venv\Scripts\activate
# On Unix/Mac:
source venv/bin/activate

# Install the package
pip install -e .
```

### Installing Dependencies Only

If you only need to install dependencies (without installing the package itself):

```bash
# Using requirements.txt
pip install -r requirements.txt

# Or directly
pip install prompt-toolkit>=3.0.0
```

### Build System

The package uses `hatchling` as the build backend (standard Python packaging). No special tools required - just `pip` and Python 3.8+.

## Quick Start

```python
from pyprompt import text, select, confirm

# Text input
name = text("What is your name?", default="John")
# Output: ? What is your name?: John

# Selection menu
choice = select("Choose an option:", ["Option 1", "Option 2", "Option 3"])
# Output: ? Choose an option: Option 1
#          » Option 1
#            Option 2
#            Option 3

# Confirmation
proceed = confirm("Proceed with initialization?", default=True)
# Output: ? Proceed with initialization? [Y/n]? y
```

## API Reference

### `text(question, default="", indent="", style=None)`

Text input prompt.

**Parameters:**
- `question` (str): Question text
- `default` (str): Default value (optional)
- `indent` (str): Additional indentation (optional)
- `style`: Not used (kept for compatibility)

**Returns:** `str` or `None` if cancelled

**Example:**
```python
name = text("What is your name?", default="John")
email = text("Email address:")
```

### `select(question, choices, indent="", pointer=" »", style=None, warning_mode=False)`

Selection menu with arrow key navigation.

**Parameters:**
- `question` (str): Question text
- `choices` (List[str]): List of choice strings
- `indent` (str): Additional indentation (optional)
- `pointer` (str): Pointer character (default: " »")
- `style`: Not used (kept for compatibility)
- `warning_mode` (bool): If True, hide question line and show only choices

**Returns:** Selected choice `str` or `None` if cancelled

**Example:**
```python
option = select("Choose an option:", ["Option 1", "Option 2", "Option 3"])

# Warning mode (no question line)
action = select(
    "Key already exists: github_windows",
    choices=["Overwrite", "Use Existing"],
    warning_mode=True
)
```

### `confirm(question, default=True, indent="", style=None)`

Yes/No confirmation prompt. Submits immediately on y/n keypress.

**Parameters:**
- `question` (str): Question text
- `default` (bool): Default value (True for Yes, False for No)
- `indent` (str): Additional indentation (optional)
- `style` (dict): Optional dictionary of style overrides for this specific prompt

**Returns:** `bool` or `None` if cancelled

**Example:**
```python
proceed = confirm("Proceed with initialization?", default=True)
# Output: ? Proceed with initialization? [Y/n]? y

replace = confirm("Replace existing files?", default=False)
# Output: ? Replace existing files? [y/N]? n
```

### `set_region_indent_func(func)`

Set a function that returns the current region indentation. Useful when integrating with terminal output systems that use region-based indentation.

**Parameters:**
- `func` (Callable[[], str]): Function that returns indentation string

**Note:** If the `indeneder` package is installed, `pyprompt` will automatically detect and use it. You only need to call this function if you want to override with a custom function.

**Example:**
```python
from pyprompt import set_region_indent_func

# Manual integration with git-py's terminal module
from git_py.core.terminal import get_region_indent
set_region_indent_func(get_region_indent)

# Or use indeneder (auto-detected if installed)
# from indeneder import get_region_indent
# set_region_indent_func(get_region_indent)
```

### `set_global_style(overrides)`

Set global style overrides that apply to all prompts.

**Parameters:**
- `overrides` (dict): Dictionary of style class names to style strings

**Example:**
```python
from pyprompt import set_global_style, text, select

# Set global style - all prompts will use green qmark instead of pink
set_global_style({'qmark': 'fg:#00ff00 bold'})

# All subsequent prompts use the new style
name = text("What is your name?")
choice = select("Choose:", ["A", "B"])
```

### `register_style(name, style_dict)`

Register a named style dictionary that can be applied at the prompt level by name.

**Parameters:**
- `name` (str): Name identifier for the style (e.g., "green_theme", "minimal")
- `style_dict` (dict): Dictionary of style class names to style strings

**Example:**
```python
from pyprompt import register_style, text, select

# Register a named style
register_style("green_theme", {
    'qmark': 'fg:#00ff00 bold',
    'pointer': 'fg:#00ff00 >',
    'highlighted': 'fg:#00ff00 bold'
})

# Use it in prompts by passing the name as a string
name = text("What is your name?", style="green_theme")
choice = select("Choose:", ["A", "B"], style="green_theme")
```

### `get_named_style(name)`

Get a registered named style dictionary.

**Parameters:**
- `name` (str): Name of the registered style

**Returns:** Style dictionary or `None` if not found

**Example:**
```python
from pyprompt import register_style, get_named_style

register_style("blue_theme", {'qmark': 'fg:#0088ff bold'})
style = get_named_style("blue_theme")  # Returns the style dict
```

### `HAS_PROMPT_TOOLKIT`

Boolean flag indicating if `prompt_toolkit` is available.

## Integration with Terminal Output Systems

### Automatic Detection

`pyprompt` automatically detects and integrates with the `indeneder` package if it's installed:

```python
from pyprompt import text, select, confirm
from indeneder import write_header

# No setup needed! pyprompt auto-detects indeneder
with write_header("Setup"):
    name = text("What is your name?")  # Automatically indented
    choice = select("Choose:", ["A", "B"])  # Automatically indented
    proceed = confirm("Proceed?")  # Automatically indented
```

### Manual Integration

If you're using a different terminal output system (like `git-py`'s terminal module), you can manually set the region indent function:

```python
from pyprompt import text, select, confirm, set_region_indent_func
from git_py.core.terminal import get_region_indent, write_header

# Set up region indentation integration
set_region_indent_func(get_region_indent)

# Now prompts will respect region indentation
with write_header("Setup"):
    name = text("What is your name?")  # Automatically indented
    choice = select("Choose:", ["A", "B"])  # Automatically indented
    proceed = confirm("Proceed?")  # Automatically indented
```

### Priority Order

The region indentation is determined in this priority order:
1. **User-set function** (via `set_region_indent_func()`)
2. **Auto-detected indeneder** (if installed)
3. **No indentation** (default)

## Styling

### Default Style

Prompts use a consistent color scheme:
- **Qmark** (`?`): Pink bold (`fg:#ff5faf bold`)
- **Question text**: Bold white (`fg:#ffffff bold`)
- **Answer/Input**: Blue bold (`fg:#33658A bold`)
- **Selected choice**: Pink bold (`fg:#ff5faf bold`)
- **Unselected choices**: Grey (`fg:#666666`)
- **Pointer** (`»`): Pink bold (`fg:#ff5faf bold`)

### Customizing Styles

You can customize styles in two ways:

#### Global Style Override

Set styles that apply to all prompts:

```python
from pyprompt import set_global_style, text, select, confirm

# Change all qmarks to green
set_global_style({'qmark': 'fg:#00ff00 bold'})

# All prompts now use green qmark
name = text("What is your name?")
choice = select("Choose:", ["A", "B"])
```

#### Per-Prompt Style Override

Override styles for individual prompts using either a dictionary or a registered style name:

```python
from pyprompt import text, select, confirm, register_style

# Using a dictionary directly
name = text("What is your name?", style={'qmark': 'fg:#00ff00 bold'})

# Using a registered named style
register_style("green_theme", {'qmark': 'fg:#00ff00 bold', 'pointer': 'fg:#00ff00 >'})
choice = select("Choose:", ["A", "B"], style="green_theme")
```

#### Style Priority

Style overrides are merged in this order (later overrides earlier):
1. Default styles
2. Global style overrides (from `set_global_style()`)
3. Per-prompt style overrides (from `style` parameter - can be dict or registered style name)

#### Available Style Classes

- `qmark` - The `?` question mark
- `question` - The question text
- `answer` - The answer/input text
- `pointer` - The selection pointer (`»`)
- `highlighted` - The selected choice text
- `text` - Unselected choice text
- `dim` - Dim text (used for `[Y/n]` indicator in confirm prompts)
- `input` - Input text (used in text prompts)

## Requirements

- Python >= 3.8
- prompt-toolkit >= 3.0.0

## License

Same license as the parent project.

