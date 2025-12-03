#!/usr/bin/env python3
"""
Simple demo script for indeneder package.
"""

# pip install -e D:\_dev\_Projects\dev-boards\_projects\indeneder


import sys

try:
    from indeneder import write_header
except ImportError:
    print("Error: indeneder package not found.", file=sys.stderr)
    print("Please install indeneder first:", file=sys.stderr)
    print("  pip install -e ../_projects/indeneder", file=sys.stderr)
    sys.exit(1)

try:
    from pyprompt import text, select, confirm, set_global_style, register_style, HAS_PROMPT_TOOLKIT
except ImportError:
    print("Error: pyprompt package not found.", file=sys.stderr)
    print("Please install pyprompt first:", file=sys.stderr)
    print("  pip install -e ../_projects/pyprompt", file=sys.stderr)
    sys.exit(1)

if not HAS_PROMPT_TOOLKIT:
    print("Error: prompt_toolkit is not installed", file=sys.stderr)
    print("Install it with: pip install prompt-toolkit", file=sys.stderr)
    sys.exit(1)



def main():
    """Demonstrate indeneder header functionality."""
    
    # Simple header with automatic indentation
    with write_header("Demo Section"):
        print("this is part of the region")
    
    print("\n---\n")
    
    # Multiple lines in a region
    with write_header("Another Section"):
        print("this is part of the region")
        print("this is also part of the region")
        print("all of these are indented")
    
    print("\n=== Global Style Override ===\n")
    
    # Script-wide (global) style override
    # Set qmark color to green (#00ff00) and pointer style to green
    # Note: pointer character is set via pointer parameter, not in style dict
    set_global_style({
        'qmark': 'fg:#00ff00 bold',      # Green qmark
        'pointer': 'fg:#00ff00 bold',    # Green pointer style (color only)
    })
    
    with write_header("Prompts with Global Style Override"):
        # All prompts now use green qmark
        name = text("What is your name?", default="John")
        print(f"Name: {name}")
        
        # Pointer character ">" is set via pointer parameter, style sets the color
        choice = select("Choose an option:", ["Option 1", "Option 2", "Option 3"], pointer=">")
        print(f"Selected: {choice}")
        
        proceed = confirm("Proceed?", default=True)
        print(f"Proceed: {proceed}")
    
    print("\n=== Per-Prompt Style Override ===\n")
    
    with write_header("Prompt with Per-Prompt Override"):
        # This prompt overrides the pointer to ">>>" (global style still applies to qmark)
        # The pointer parameter sets the character, style sets the color
        choice = select(
            "Choose action:",
            choices=["Create", "Update", "Delete"],
            pointer=">>>",  # Override pointer character to ">>>"
            style={'pointer': 'fg:#00ff00 bold'}  # Style override for pointer (green, bold)
        )
        print(f"Action: {choice}")
    
    print("\n=== Named Style Dictionary ===\n")
    
    # Register a named style that can be reused
    register_style("blue_theme", {
        'qmark': 'fg:#0088ff bold',      # Blue qmark
        'pointer': 'fg:#0088ff bold',    # Blue pointer
        'highlighted': 'fg:#00aaff bold', # Light blue selected text
        'answer': 'fg:#0066cc bold',      # Darker blue answer
    })
    
    with write_header("Prompts with Named Style"):
        # Use the registered named style by passing the name as a string
        name = text("What is your name?", style="blue_theme")
        print(f"Name: {name}")
        
        choice = select("Choose an option:", ["Option 1", "Option 2", "Option 3"], style="blue_theme")
        print(f"Selected: {choice}")
        
        # You can still override specific properties even when using a named style
        proceed = confirm("Proceed?", default=True, style="blue_theme")
        print(f"Proceed: {proceed}")
    
    return 0


if __name__ == "__main__":
    sys.exit(main())

