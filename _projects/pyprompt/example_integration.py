"""
Example showing integration with git-py's terminal system (optional).
"""

# This example shows how to use pyprompt with git-py's terminal output system
# You don't need git-py to use pyprompt - this is just for integration

try:
    # Try to import git-py's terminal module
    import sys
    import os
    
    # Add git-py to path (adjust path as needed)
    git_py_path = os.path.join(os.path.dirname(__file__), '..', '..', 'bootstraps', 'git_py')
    if os.path.exists(git_py_path):
        sys.path.insert(0, git_py_path)
        from git_py.core.terminal import get_region_indent, write_header
        from pyprompt import text, select, confirm, set_region_indent_func
        
        # Set up region indentation integration
        set_region_indent_func(get_region_indent)
        
        print("=== pyprompt with git-py Terminal Integration ===\n")
        
        # Prompts will now respect region indentation
        with write_header("User Setup"):
            name = text("What is your name?", default="John")
            print(f"Name: {name}")
            
            choice = select("Choose an option:", ["Option 1", "Option 2", "Option 3"])
            print(f"Selected: {choice}")
            
            proceed = confirm("Proceed?", default=True)
            print(f"Proceed: {proceed}")
        
        print("\nIntegration example completed!")
    else:
        print("git-py not found - skipping integration example")
        print("pyprompt works standalone without git-py")
        
except ImportError as e:
    print(f"git-py not available: {e}")
    print("pyprompt works standalone without git-py")

