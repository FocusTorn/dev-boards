"""
Example demonstrating style customization in pyprompt.
"""

from pyprompt import text, select, confirm, set_global_style, HAS_PROMPT_TOOLKIT

def main():
    if not HAS_PROMPT_TOOLKIT:
        print("Error: prompt_toolkit is not installed")
        print("Install it with: pip install prompt-toolkit")
        return
    
    print("=== Default Style ===\n")
    
    # Default style (pink qmark, white question, blue answer)
    name = text("What is your name?", default="John")
    print(f"Name: {name}\n")
    
    print("=== Global Style Override ===\n")
    
    # Set global style - all prompts will use green qmark
    set_global_style({'qmark': 'fg:#00ff00 bold'})
    
    # All subsequent prompts use green qmark
    choice = select("Choose an option:", ["Option 1", "Option 2", "Option 3"])
    print(f"Selected: {choice}\n")
    
    print("=== Per-Prompt Style Override ===\n")
    
    # This prompt uses yellow qmark (overrides global green)
    proceed = confirm(
        "Proceed with initialization?",
        default=True,
        style={'qmark': 'fg:#ffff00 bold', 'answer': 'fg:#00ffff bold'}
    )
    print(f"Proceed: {proceed}\n")
    
    print("=== Custom Select Style ===\n")
    
    # Custom colors for select prompt
    action = select(
        "Choose action:",
        choices=["Create", "Update", "Delete"],
        style={
            'pointer': 'fg:#00ff00 bold',      # Green pointer
            'highlighted': 'fg:#ffff00 bold',  # Yellow selected text
            'text': 'fg:#888888'               # Darker grey unselected
        }
    )
    print(f"Action: {action}\n")
    
    print("All style examples completed!")

if __name__ == "__main__":
    main()

