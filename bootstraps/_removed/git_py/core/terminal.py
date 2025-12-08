"""
Terminal output formatting utilities.
"""

import sys
from typing import Optional
from contextlib import contextmanager

# ANSI color codes for status symbols
COLOR_GREEN = "\x1B[32m"      # Green for checkmarks
COLOR_YELLOW = "\x1B[33m"     # Yellow for warnings
COLOR_BRIGHT_YELLOW_BOLD = "\x1B[33m\x1B[1m"  # Darker yellow bold for warning symbol (⚠)
COLOR_RED = "\x1B[31m"        # Red for errors
COLOR_RESET = "\x1B[0m"       # Reset color
COLOR_DIM = "\x1B[2m"         # Dim text
BOLD_CHECK = "✔"              # Unicode heavy/bold checkmark (U+2714)

# Standard prompt text colors (matching PROMPT_STYLE dictionary)
# These match the colors from prompts.PROMPT_STYLE for consistency
COLOR_QUESTION_TEXT = "\x1B[37m\x1B[1m"  # Bold white (matches 'question' style: fg:#ffffff bold)

# Region system for automatic indentation
_active_regions = []
_region_indent = "  "  # 2 spaces


class IndentedOutput:
    """Wrapper for stdout/stderr that automatically indents output in regions."""
    def __init__(self, original_stream):
        self.original_stream = original_stream
    
    def write(self, text):
        if _active_regions:
            indent = _region_indent * len(_active_regions)
            # Split by newlines and indent each line
            lines = text.split('\n')
            indented_lines = []
            for i, line in enumerate(lines):
                if i == len(lines) - 1 and not text.endswith('\n'):
                    # Last line without trailing newline - don't add newline
                    if line.strip():
                        indented_lines.append(indent + line)
                    else:
                        indented_lines.append(line)
                else:
                    if line.strip():
                        indented_lines.append(indent + line)
                    else:
                        indented_lines.append(line)
            text = '\n'.join(indented_lines)
        self.original_stream.write(text)
    
    def flush(self):
        self.original_stream.flush()
    
    def __getattr__(self, name):
        return getattr(self.original_stream, name)


class start_region_context:
    """Context manager returned by write_header for automatic indentation."""
    def __init__(self, name: str):
        self.name = name
        self.old_stdout = None
        self.old_stderr = None
    
    def __enter__(self):
        _active_regions.append(self.name)
        # Replace stdout/stderr with indented versions
        self.old_stdout = sys.stdout
        self.old_stderr = sys.stderr
        sys.stdout = IndentedOutput(self.old_stdout)
        sys.stderr = IndentedOutput(self.old_stderr)
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        # Restore original streams
        if self.old_stdout:
            sys.stdout = self.old_stdout
        if self.old_stderr:
            sys.stderr = self.old_stderr
        if _active_regions and _active_regions[-1] == self.name:
            _active_regions.pop()
        return False


@contextmanager
def start_region(name: str = ""):
    """
    Context manager for a region that automatically indents all output.
    
    Usage:
        with start_region("Step 1"):
            print("This will be indented by 2 spaces")
            print("So will this")
        print("This will not be indented")
    """
    _active_regions.append(name)
    # Replace stdout/stderr with indented versions
    old_stdout = sys.stdout
    old_stderr = sys.stderr
    sys.stdout = IndentedOutput(old_stdout)
    sys.stderr = IndentedOutput(old_stderr)
    try:
        yield
    finally:
        # Restore original streams
        sys.stdout = old_stdout
        sys.stderr = old_stderr
        if _active_regions and _active_regions[-1] == name:
            _active_regions.pop()


def end_region(name: Optional[str] = None):
    """
    Explicitly end a region (usually not needed if using context manager).
    If name is provided, ends that specific region; otherwise ends the last one.
    """
    if name:
        if name in _active_regions:
            while _active_regions and _active_regions[-1] != name:
                _active_regions.pop()
            if _active_regions:
                _active_regions.pop()
    else:
        if _active_regions:
            _active_regions.pop()


def write_boxed_header(title: str, width: int = 80) -> None:
    """
    Write a boxed header with centered title.
    
    Args:
        title: The title text to display
        width: The total width of the box (default: 80)
    """
    # Ensure title has even length for proper centering
    display_title = title if len(title) % 2 == 0 else f"{title} "
    
    # Calculate padding for centering
    padding = max(0, ((width - len(display_title)) / 2) - 1)
    
    left_pad = " " * int(padding)
    right_pad = " " * int(padding)
    top_bottom = "━" * (width - 2)
    color_cyan = "\x1B[38;5;51m"
    color_reset = "\x1B[0m"
    
    print(f"{color_cyan}┏{top_bottom}┓{color_reset}")
    print(f"{color_cyan}┃{left_pad}{display_title}{right_pad}┃{color_reset}")
    print(f"{color_cyan}┗{top_bottom}┛{color_reset}")


def write_header(title: str, width: int = 65, start_region: bool = True):
    """
    Write a simple header with underline.
    
    Args:
        title: The header title
        width: The total width of the header (default: 65)
        start_region: If True, returns a context manager that automatically indents output
    
    Returns:
        If start_region is True, returns a context manager. Otherwise returns None.
    
    Usage:
        with write_header("Step 1"):
            print("This will be indented")
            print("So will this")
    """
    tail_lines = max(0, width - (len(title) + 4))
    tail = "─" * tail_lines
    color_blue = "\x1B[38;5;33m"
    color_reset = "\x1B[0m"
    
    print()  # Blank line before header
    print(f"{color_blue}┌─ {title} {tail}{color_reset}")
    
    if start_region:
        return start_region_context(title)
    return None


def write_header_fat(title: str, width: int = 65) -> None:
    """Write a fat header with thick underline."""
    tail_lines = max(0, width - (len(title) + 4))
    tail = "━" * tail_lines
    color_blue = "\x1B[38;5;33m"
    color_reset = "\x1B[0m"
    
    print(f"{color_blue}┏━ {title} {tail}{color_reset}")


def get_region_indent() -> str:
    """Get the current indentation string based on active regions."""
    return _region_indent * len(_active_regions)

