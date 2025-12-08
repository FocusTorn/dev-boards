"""
SSH key selection logic for authentication command.
"""

from pathlib import Path

from ..core.terminal import write_header, get_region_indent
from ..core.prompts import HAS_PROMPT_TOOLKIT, text as prompt_text, select as prompt_select
from ..operations.ssh import discover_ssh_keys


def select_ssh_key() -> tuple:
    """
    Prompt user to select an SSH key or generate a new one.
    
    Returns:
        tuple: (selected_ssh_key_path, new_key_name, new_key_overwrite)
        - selected_ssh_key_path: Path to selected key, "GENERATE_NEW", or None
        - new_key_name: Name for new key if generating, None otherwise
        - new_key_overwrite: Whether to overwrite existing key if generating
    """
    existing_keys = discover_ssh_keys()
    
    with write_header("SSH Key Selection"):
        if existing_keys:
            menu_options = []
            for key in existing_keys:
                menu_options.append(key['name'])
            menu_options.append("Generate new SSH key")
            menu_options.append("Skip SSH key setup")
        else:
            menu_options = ["Generate new SSH key", "Skip SSH key setup"]
        
        if HAS_PROMPT_TOOLKIT:
            # Calculate indentation based on active regions (2 spaces per region)
            indent = get_region_indent()
            selected_option = prompt_select(
                "SSH key to use:",
                choices=menu_options,
                indent=indent,
                pointer=" »"
            )
            
            if selected_option is None:
                print("Cancelled.")
                return (None, None, False)
            
            menu_entry_index = menu_options.index(selected_option)
            
            if menu_entry_index == len(menu_options) - 1:
                # "Skip SSH key setup"
                return (None, None, False)
            elif menu_entry_index == len(menu_options) - 2:
                # "Generate new SSH key" - prompt for key name now
                indent = get_region_indent()
                new_key_name = prompt_text(
                    "SSH key name (without .pub extension):",
                    default="github_auto",
                    indent=indent,
                )
                if new_key_name is None:
                    print("Cancelled.")
                    return (None, None, False)
                new_key_name = new_key_name.strip()
                if not new_key_name:
                    new_key_name = "github_auto"
                
                # Check if key already exists and prompt to overwrite
                ssh_dir = Path.home() / ".ssh"
                key_path_private = ssh_dir / new_key_name
                key_path_public = ssh_dir / f"{new_key_name}.pub"
                overwrite_key = False
                
                if key_path_private.exists() or key_path_public.exists():
                    selected_action = prompt_select(
                        f"SSH Key already exists: {new_key_name}",
                        choices=["Overwrite", "Use Existing"],
                        indent=indent,
                        pointer=" »"
                    )
                    if selected_action is None:
                        print("Cancelled.")
                        return (None, None, False)
                    overwrite_key = (selected_action == "Overwrite")
                
                return ("GENERATE_NEW", new_key_name, overwrite_key)
            else:
                # Selected an existing key
                selected_key = existing_keys[menu_entry_index]
                return (selected_key["public_path"], None, False)
        else:
            # Fallback: simple numbered menu
            print("SSH key to use:")
            for i, option in enumerate(menu_options, 1):
                print(f"  {i}. {option}")
            
            while True:
                try:
                    choice = input(f"Enter choice (1-{len(menu_options)}): ").strip()
                    choice_num = int(choice)
                    if 1 <= choice_num <= len(menu_options):
                        menu_entry_index = choice_num - 1
                        break
                    else:
                        print(f"Invalid choice. Please enter a number between 1 and {len(menu_options)}")
                except ValueError:
                    print("Invalid input. Please enter a number.")
                except KeyboardInterrupt:
                    print("\nCancelled.")
                    return (None, None, False)
            
            if menu_entry_index == len(menu_options) - 1:
                # "Skip SSH key setup"
                return (None, None, False)
            elif menu_entry_index == len(menu_options) - 2:
                # "Generate new SSH key" - prompt for key name now
                new_key_name = input("SSH key name (default: github_auto): ").strip()
                if not new_key_name:
                    new_key_name = "github_auto"
                
                # Check if key already exists and prompt to overwrite
                ssh_dir = Path.home() / ".ssh"
                key_path_private = ssh_dir / new_key_name
                key_path_public = ssh_dir / f"{new_key_name}.pub"
                
                if key_path_private.exists() or key_path_public.exists():
                    print(f"SSH Key already exists: {new_key_name}")
                    print("  1. Overwrite")
                    print("  2. Use Existing")
                    while True:
                        try:
                            choice = input("Enter choice (1-2): ").strip()
                            if choice == "1":
                                overwrite_key = True
                                break
                            elif choice == "2":
                                overwrite_key = False
                                break
                            else:
                                print("Invalid choice. Please enter 1 or 2.")
                        except KeyboardInterrupt:
                            print("\nCancelled.")
                            return (None, None, False)
                else:
                    overwrite_key = False
                
                return ("GENERATE_NEW", new_key_name, overwrite_key)
            else:
                # Selected an existing key
                if existing_keys:
                    selected_key = existing_keys[menu_entry_index]
                    return (selected_key["public_path"], None, False)
                else:
                    # Shouldn't happen, but handle gracefully
                    return ("GENERATE_NEW", "github_auto", False)

