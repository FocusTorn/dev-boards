#!/usr/bin/env python3
"""
Formatters module for creating styled headers in terminal output.
"""

def write_boxed_header(title: str, width: int = 80) -> None: #>
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
    
    #<

def write_header(title: str, width: int = 65) -> None: #>
    tail_lines = max(0, width - (len(title) + 4))
    tail = "─" * tail_lines
    color_blue = "\x1B[38;5;33m"
    color_reset = "\x1B[0m"
    
    print(f"{color_blue}┌─ {title} {tail}{color_reset}")
    
    #<

def write_header_fat(title: str, width: int = 65) -> None: #>
    tail_lines = max(0, width - (len(title) + 4))
    tail = "━" * tail_lines
    color_blue = "\x1B[38;5;33m"
    color_reset = "\x1B[0m"
    
    print(f"{color_blue}┏━ {title} {tail}{color_reset}")
    
    #<











if __name__ == "__main__":
    # Example usage
    write_boxed_header("GitHub SSH Status")
    write_header("Sub Header")
    write_header_fat("Sub Header")


