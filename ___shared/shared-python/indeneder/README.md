# indeneder

Terminal output formatting with automatic indentation - extracted from `git-py` for standalone use.

## Features

- **Automatic indentation** - Content within headers is automatically indented
- **Header types** - Simple headers, fat headers, and boxed headers
- **Context manager support** - Use `with` statement for clean indentation
- **Color support** - ANSI color codes for status symbols

## Installation

### Standard Installation (pip)

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

## Quick Start

```python
from indeneder import write_header, write_header_fat, write_boxed_header

# Simple header with automatic indentation
with write_header("Step 1: Setup"):
    print("This will be indented by 2 spaces")
    print("So will this")
    print("All output is automatically indented")

# Fat header
write_header_fat("Major Section")

# Boxed header
write_boxed_header("Important Notice")
```

## API Reference

### `write_header(title, width=65, start_region=True)`

Write a simple header with underline. Returns a context manager that automatically indents content.

**Parameters:**
- `title` (str): The header title
- `width` (int): The total width of the header (default: 65)
- `start_region` (bool): If True, returns a context manager for indentation (default: True)

**Returns:** Context manager (if `start_region=True`) or `None`

**Example:**
```python
with write_header("Step 1"):
    print("Indented content")
    print("More indented content")
```

### `write_header_fat(title, width=65)`

Write a fat header with thick underline. Does not start a region.

**Parameters:**
- `title` (str): The header title
- `width` (int): The total width of the header (default: 65)

**Example:**
```python
write_header_fat("Major Section")
print("This is not indented")
```

### `write_boxed_header(title, width=80)`

Write a boxed header with centered title. Does not start a region.

**Parameters:**
- `title` (str): The header title
- `width` (int): The total width of the box (default: 80)

**Example:**
```python
write_boxed_header("Important Notice")
print("This is not indented")
```

### `start_region(name="")`

Context manager for manual region indentation (without a header).

**Parameters:**
- `name` (str): Optional name for the region

**Example:**
```python
with start_region("Custom Region"):
    print("This will be indented")
    print("So will this")
```

### `get_region_indent()`

Get the current indentation string based on active regions.

**Returns:** `str` - Current indentation (2 spaces per active region)

**Example:**
```python
with write_header("Step 1"):
    indent = get_region_indent()  # Returns "  " (2 spaces)
    with start_region("Sub-step"):
        indent = get_region_indent()  # Returns "    " (4 spaces)
```

### `end_region(name=None)`

Explicitly end a region (usually not needed if using context manager).

**Parameters:**
- `name` (str, optional): Name of region to end, or None to end the last one

## Color Constants

The package exports color constants for status symbols:

- `COLOR_GREEN` - Green for checkmarks
- `COLOR_YELLOW` - Yellow for warnings
- `COLOR_RED` - Red for errors
- `COLOR_RESET` - Reset color
- `COLOR_DIM` - Dim text
- `BOLD_CHECK` - Unicode heavy/bold checkmark (✔)

**Example:**
```python
from indeneder import write_header, COLOR_GREEN, COLOR_RESET, BOLD_CHECK

with write_header("Status"):
    print(f"{COLOR_GREEN}{BOLD_CHECK}{COLOR_RESET} Operation completed")
```

## Nested Headers

Headers can be nested for hierarchical output:

```python
with write_header("Main Section"):
    print("Level 1 indentation")
    
    with write_header("Sub-section"):
        print("Level 2 indentation")
        
        with write_header("Sub-sub-section"):
            print("Level 3 indentation")
```

## Requirements

- Python >= 3.8
- No external dependencies

## License

Same license as the parent project.

