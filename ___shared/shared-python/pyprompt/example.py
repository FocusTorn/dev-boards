"""
Example usage of pyprompt package.
"""

from pyprompt import text, select, confirm, HAS_PROMPT_TOOLKIT

def main():
    if not HAS_PROMPT_TOOLKIT:
        print("Error: prompt_toolkit is not installed")
        print("Install it with: pip install prompt-toolkit")
        return
    
    print("=== pyprompt Examples ===\n")
    
    # Text prompt
    name = text("What is your name?", default="John")
    print(f"Name: {name}\n")
    
    # Select prompt
    choice = select("Choose an option:", ["Option 1", "Option 2", "Option 3"])
    print(f"Selected: {choice}\n")
    
    # Confirm prompt
    proceed = confirm("Proceed with initialization?", default=True)
    print(f"Proceed: {proceed}\n")
    
    print("All prompts completed!")

if __name__ == "__main__":
    main()

