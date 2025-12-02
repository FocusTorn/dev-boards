#!/usr/bin/env python3
"""
GitHub Setup Wizard
Unified wizard collects all information for SSH, local repo, and remote setup
"""

import os
import sys
import json
import subprocess
from pathlib import Path
from typing import Optional, List, Dict, Any
import argparse


try:
    import questionary
    from questionary import Style, Choice
    from prompt_toolkit.styles import merge_styles, Style as PTStyle
    from prompt_toolkit.formatted_text import FormattedText
    from prompt_toolkit.shortcuts import prompt
    from prompt_toolkit.key_binding import KeyBindings
    from prompt_toolkit.keys import Keys
    from prompt_toolkit.application import Application
    from prompt_toolkit.layout import Layout, HSplit, VSplit
    from prompt_toolkit.widgets import RadioList
    from prompt_toolkit.formatted_text import HTML, ANSI
except ImportError:
    print("❌ Error: questionary library is required")
    print("   Install it with: pip install questionary")
    sys.exit(1)


def create_no_bg_style(base_style: Style) -> Style:
    """
    Create a style that removes background colors from selected/highlighted items.
    Extends the base style by explicitly setting empty styles for selected/highlighted
    to override questionary's default background colors.
    """
    # Get the base style rules as a list of tuples
    style_rules = list(base_style.style_rules) if hasattr(base_style, 'style_rules') else []
    
    # Remove any existing 'selected' or 'highlighted' rules
    style_rules = [(token, style) for token, style in style_rules 
                   if token not in ('selected', 'highlighted')]
    
    # Add empty styles for selected/highlighted to remove background
    # Empty string means no styling, which should remove the default background
    style_rules.append(('selected', ''))
    style_rules.append(('highlighted', ''))
    
    return Style(style_rules)


def select_no_bg(question: str, choices: List[str], default: Optional[str] = None, **kwargs) -> Optional[str]:
    """
    Custom select prompt using prompt_toolkit directly.
    - Default choice: normal text (no background, no special formatting)
    - When cursor is on a line: pink bold text
    - No background highlighting on any line
    
    Usage:
        result = select_no_bg("Choose:", ["opt1", "opt2"], default="opt1")
    """
    if not choices:
        return None
    
    # Find default index
    default_index = 0
    if default:
        try:
            default_index = choices.index(default)
        except ValueError:
            default_index = 0
    
    # Use list to make it mutable and reactive
    selected_index = [default_index]
    result_value = [None]
    
    # Extract colors from WIZARD_STYLE if provided
    answer_color = 'fg:#33658A bold'  # Default from WIZARD_STYLE
    text_color = 'fg:#666666'  # Default from WIZARD_STYLE
    pointer_color = 'fg:#ff5faf bold'  # Default from WIZARD_STYLE
    highlighted_color = 'fg:#ff5faf bold'  # Default from WIZARD_STYLE
    qmark_color = 'fg:#ff5faf bold'  # Default from WIZARD_STYLE
    
    base_style = kwargs.pop('style', None)
    if base_style:
        for token, style in base_style.style_rules:
            if token == 'answer':
                answer_color = style
            elif token == 'text':
                text_color = style
            elif token == 'pointer':
                pointer_color = style
            elif token == 'highlighted':
                highlighted_color = style
            elif token == 'qmark':
                qmark_color = style
    
    # Create custom style for the prompt - matching WIZARD_STYLE colors
    custom_style = PTStyle.from_dict({
        'qmark': qmark_color,  # Pink bold for question mark (❓)
        'question': 'bold fg:#ffffff',  # Bold white for question
        'pointer': pointer_color,
        'highlighted': highlighted_color,  # Pink when cursor is on line
        'answer': answer_color,  # Use answer color from WIZARD_STYLE (fg:#33658A bold)
        'text': text_color,  # Text color for non-highlighted choices
        '': text_color,  # Default text color (gray) for non-highlighted choices
    })
    
    # Create key bindings
    kb = KeyBindings()
    
    @kb.add('up')
    @kb.add('k')
    def move_up(event):
        if selected_index[0] > 0:
            selected_index[0] -= 1
    
    @kb.add('down')
    @kb.add('j')
    def move_down(event):
        if selected_index[0] < len(choices) - 1:
            selected_index[0] += 1
    
    @kb.add('enter')
    def select_choice(event):
        result_value[0] = choices[selected_index[0]]
        # Invalidate to trigger redraw with answer shown
        event.app.invalidate()
        # Small delay to show the answer, then exit
        import time
        time.sleep(0.1)
        # Exit without printing newline
        event.app.exit(result=result_value[0], style='class:')
    
    @kb.add('c-c')
    def cancel(event):
        event.app.exit(result=None)
    
    # Create formatted text for choices - this function will be called to update the display
    def get_formatted_choices():
        result = []
        for i, choice in enumerate(choices):
            is_selected = (i == selected_index[0])
            pointer = '> ' if is_selected else '  '
            
            if is_selected:
                # Pink bold when cursor is on this line
                result.append(('class:pointer', pointer))
                result.append(('class:highlighted', choice))
            else:
                # Use text color from WIZARD_STYLE for non-highlighted choices
                result.append(('class:pointer', pointer))
                result.append(('class:text', choice))
            
            if i < len(choices) - 1:
                result.append(('', '\n'))
        
        return result
    
    # Create layout
    from prompt_toolkit.layout.containers import Window
    from prompt_toolkit.layout.controls import FormattedTextControl
    
    # Track if this is the first render to add blank line only once
    first_render = [True]
    
    # Format question text with bold white styling
    # Show answer after question if one has been selected
    def get_question_text():
        if result_value[0]:
            # Show question with answer (like questionary does) - use answer color from WIZARD_STYLE
            # Add space between question and answer to match questionary format
            # No newline at the end to avoid blank lines between prompts
            return [('class:qmark', '❓ '), ('class:question', question), ('', ' '), ('class:answer', result_value[0])]
        else:
            # Add blank line at start only on first render
            if first_render[0]:
                first_render[0] = False
                return [('', '\n'), ('class:qmark', '❓ '), ('class:question', question)]
            else:
                return [('class:qmark', '❓ '), ('class:question', question)]
    
    # Hide choices when answer is selected
    def get_choices_display():
        if result_value[0]:
            # Hide choices when answer is selected
            return []
        return get_formatted_choices()
    
    question_control = FormattedTextControl(get_question_text)
    choices_control = FormattedTextControl(get_choices_display)
    
    # Create conditional container that hides choices window when answer is selected
    from prompt_toolkit.layout.containers import ConditionalContainer
    from prompt_toolkit.filters import Condition
    
    def should_show_choices():
        return result_value[0] is None
    
    layout = Layout(
        HSplit([
            Window(question_control, height=1, dont_extend_height=True, always_hide_cursor=True),
            ConditionalContainer(
                Window(choices_control, always_hide_cursor=True),
                filter=Condition(should_show_choices)
            )
        ])
    )
    
    # Create application with refresh to update display on key press
    app = Application(
        layout=layout,
        key_bindings=kb,
        style=custom_style,
        full_screen=False,
        refresh_interval=0.05,  # Refresh to update display when selection changes
        erase_when_done=False  # Don't erase the prompt when done (keeps it visible)
    )
    
    # Run the application
    try:
        result = app.run()
        return result
    except KeyboardInterrupt:
        return None


def get_script_dir() -> Path:
    """Get the directory where this script is located"""
    if getattr(sys, 'frozen', False):
        return Path(sys.executable).parent
    return Path(__file__).parent


def write_boxed_header(title: str, width: int = 80) -> None:
    """Print a boxed header"""
    display_title = title if len(title) % 2 == 0 else f"{title} "
    padding = max(0, (width - len(display_title)) // 2 - 1)
    left_pad = " " * padding
    right_pad = " " * padding
    top_bottom = "━" * (width - 2)
    
    print(f"┏{top_bottom}┓")
    print(f"┃{left_pad}{display_title}{right_pad}┃")
    print(f"┗{top_bottom}┛")
    print()


def write_header(title: str, width: int = 65) -> None:
    """Print a section header"""
    tail_lines = max(0, width - (len(title) + 4))
    tail = "─" * tail_lines
    print(f"┌─ {title} {tail}")


def test_ssh_connection() -> bool:
    """Test SSH connection to GitHub"""
    known_hosts_path = Path.home() / ".ssh" / "known_hosts"
    has_known_host = False
    
    if known_hosts_path.exists():
        try:
            content = known_hosts_path.read_text()
            if "github.com" in content:
                has_known_host = True
        except Exception:
            pass
    
    if not has_known_host:
        return False
    
    try:
        result = subprocess.run(
            ["ssh", "-o", "BatchMode=yes", "-o", "ConnectTimeout=5", "-T", "git@github.com"],
            capture_output=True,
            timeout=10
        )
        return result.returncode == 1
    except Exception:
        return False


def test_local_repo() -> bool:
    """Check if current directory is a git repository"""
    try:
        result = subprocess.run(
            ["git", "rev-parse", "--git-dir"],
            capture_output=True,
            timeout=5
        )
        return result.returncode == 0
    except Exception:
        return False


def get_git_config(key: str) -> Optional[str]:
    """Get a git config value"""
    try:
        result = subprocess.run(
            ["git", "config", "--global", key],
            capture_output=True,
            text=True,
            timeout=5
        )
        if result.returncode == 0:
            return result.stdout.strip()
    except Exception:
        pass
    return None


def discover_ssh_keys() -> List[Dict[str, str]]:
    """Discover existing SSH keys"""
    ssh_dir = Path.home() / ".ssh"
    existing_keys = []
    
    if not ssh_dir.exists():
        return existing_keys
    
    for key_file in ssh_dir.glob("*"):
        if (key_file.suffix == ".pub" or 
            key_file.name in ["known_hosts", "config", "authorized_keys"] or
            key_file.name.endswith(".bak")):
            continue
        
        pub_key_path = key_file.with_suffix(".pub")
        if not pub_key_path.exists():
            continue
        
        key_info = {
            "name": key_file.stem,
            "private_path": str(key_file),
            "public_path": str(pub_key_path),
            "fingerprint": "",
            "comment": ""
        }
        
        try:
            pub_key_content = pub_key_path.read_text().strip()
            parts = pub_key_content.split()
            if len(parts) >= 3:
                key_info["comment"] = parts[2]
            
            result = subprocess.run(
                ["ssh-keygen", "-lf", str(pub_key_path)],
                capture_output=True,
                text=True,
                timeout=5
            )
            if result.returncode == 0:
                fingerprint_parts = result.stdout.strip().split()
                if len(fingerprint_parts) >= 2:
                    key_info["fingerprint"] = fingerprint_parts[1]
        except Exception:
            pass
        
        existing_keys.append(key_info)
    
    existing_keys.sort(key=lambda x: ("0" if x["name"].startswith("github_") else "1") + x["name"])
    return existing_keys


def has_existing_ssh_config() -> bool:
    """Check if SSH config already has GitHub configuration"""
    ssh_config_path = Path.home() / ".ssh" / "config"
    if not ssh_config_path.exists():
        return False
    
    try:
        content = ssh_config_path.read_text()
        return "Host github.com" in content
    except Exception:
        return False


def has_existing_remotes() -> bool:
    """Check if git repository has existing remotes"""
    try:
        result = subprocess.run(
            ["git", "remote"],
            capture_output=True,
            text=True,
            timeout=5
        )
        if result.returncode == 0:
            remotes = result.stdout.strip().splitlines()
            return len(remotes) > 0
    except Exception:
        pass
    return False


# Custom wizard style matching Go wizard formatting
# WIZARD_STYLE = Style([
#     ('qmark', 'fg:#ff5faf bold'),        # Question mark - magenta (205)
#     ('question', 'fg:#ffffd7 bold'),     # Question text - bright yellow (230)
#     ('answer', 'fg:#ffffd7 bold'),       # Answer text - bright yellow (230)
#     ('pointer', 'fg:#ff5faf bold'),      # Pointer (>) - magenta (205)
#     ('highlighted', 'fg:#ffffd7 bold'),  # Highlighted choice - yellow bold
#     ('selected', 'fg:#ff5faf'),          # Selected choice - magenta
#     ('separator', 'fg:#585858'),         # Separator - gray (240)
#     ('instruction', 'fg:#585858'),       # Instruction text - gray (240)
#     ('text', 'fg:#585858'),              # Plain text - gray (240)
#     ('disabled', 'fg:#585858 italic')    # Disabled choices - gray (240)
# ])

WIZARD_STYLE = Style([
    ('qmark', 'fg:#ff5faf bold'),       # token in front of the question
    ('question', 'bold'),               # question text
    ('answer', 'fg:#33658A bold'),      # submitted answer text behind the question
    ('pointer', 'fg:#ff5faf bold'),     # pointer used in select and checkbox prompts
    ('selected', 'fg:#009900 noreverse bold'), 
    ('highlighted', 'fg:#ff5faf bold'), # pointed-at choice in select and checkbox prompts
    ('separator', 'fg:#cc5454'),        # separator in lists
    ('instruction', 'fg:#666666'),      # user instructions for select, rawselect, checkbox
    ('text', 'fg:#666666'),                       # plain text
    ('disabled', 'fg:#858585 italic')   # disabled choices for select and checkbox prompts
    
    

])


# choices2 = [
#     Choice(title=[("class:text", "order a "), ("class:highlighted", "big pizza")]),
#     Choice(title="Create new key", checked=True),                         
# ]
                         
                         
def run_wizard(
    ssh_configured: bool,
    local_repo_exists: bool,
    has_remotes: bool,
    existing_ssh_keys: List[Dict[str, str]],
    has_ssh_config: bool,
    default_github_user: Optional[str],
    default_repo_name: str,
    default_git_name: Optional[str],
    default_git_email: Optional[str],
    skip_ssh: bool = False,
    skip_local_repo: bool = False,
    skip_remote: bool = False
) -> Optional[Dict[str, Any]]:
    """Run the wizard using questionary's form() for a more natural flow"""
    
    results = {}
    original_working_dir = Path.cwd()
    
    
    # Build form questions dynamically based on current state
    questions = {}
    
    
    
    
    
    
    # Test code removed - select_no_bg cannot be used inside questionary.form()
    # It must be called directly as it returns a string, not a question object
    # SSH Setup Section
    if not ssh_configured and not skip_ssh:
        if existing_ssh_keys:
            # Create choices from existing keys
            key_choices = []
            for key in existing_ssh_keys:
                fingerprint = f"({key['fingerprint']})" if key['fingerprint'] else ""
                comment = f" - {key['comment']}" if key['comment'] else ""
                key_choices.append(f"{key['name']}{comment} {fingerprint}")
            key_choices.append("Create new key")
            
            # Call select_no_bg directly (it returns a string, not a question object)
            # Store the result to add to answers later
            ssh_key_selection = select_no_bg(
                "Select SSH Key",
                choices=key_choices,
                default=key_choices[0] if key_choices else None,
                style=WIZARD_STYLE,
                qmark="❓"
            )
            if ssh_key_selection is None:
                return None
        else:
            questions['ssh_new_key_name'] = questionary.text(
                "SSH key name",
                default="github_pi",
                placeholder="github_pi",
                style=WIZARD_STYLE,
                qmark="❓"
            )
        
        questions['ssh_add_to_github'] = questionary.confirm(
            "Add SSH key to GitHub?",
            default=True,
            style=WIZARD_STYLE,
            qmark="❓"
        )
        
        if has_ssh_config:
            questions['ssh_update_config'] = questionary.confirm(
                "Update SSH config?",
                default=True,
                style=WIZARD_STYLE,
                qmark="❓"
            )
        else:
            questions['ssh_configure_config'] = questionary.confirm(
                "Configure SSH config?",
                default=True,
                style=WIZARD_STYLE,
                qmark="❓"
            )
    
    # Local Repository Setup Section
    if not skip_local_repo:
        if local_repo_exists:
            questions['local_recreate_repo'] = questionary.confirm(
                f"Remove existing repository and recreate? (Repository exists at: {original_working_dir})",
                default=True,
                style=WIZARD_STYLE,
                qmark="❓"
            )
        
        if not default_git_name:
            questions['local_git_name'] = questionary.text(
                "Git user name",
                placeholder="Your Name",
                style=WIZARD_STYLE,
                qmark="❓"
            )
        
        if not default_git_email:
            questions['local_git_email'] = questionary.text(
                "Git user email",
                placeholder="your.email@example.com",
                style=WIZARD_STYLE,
                qmark="❓"
            )
    
    # Remote Repository Setup Section
    if not skip_remote:
        if not default_github_user:
            questions['remote_github_user'] = questionary.text(
                "GitHub username",
                placeholder="username",
                style=WIZARD_STYLE,
                qmark="❓"
            )
        else:
            questions['remote_github_user'] = questionary.text(
                "GitHub username (Detected from Git config)",
                default=default_github_user,
                placeholder=default_github_user,
                style=WIZARD_STYLE,
                qmark="❓"
            )
        
        questions['remote_repo_name'] = questionary.text(
            "Repository name (Default: current directory name)",
            default=default_repo_name,
            placeholder=default_repo_name,
            style=WIZARD_STYLE,
            qmark="❓"
        )
        
        questions['remote_repo_private'] = questionary.confirm(
            "Make repository private? (Private repositories are only visible to you and collaborators)",
            default=False,
            style=WIZARD_STYLE,
            qmark="❓"
        )
        
        questions['remote_create_repo'] = questionary.confirm(
            "Create repository on GitHub? (Automatically create the repository on GitHub if it doesn't exist)",
            default=True,
            style=WIZARD_STYLE,
            qmark="❓"
        )
        
        if has_remotes:
            questions['remote_remove_remotes'] = questionary.confirm(
                "Remove existing remotes? (Remove existing remotes and set up new one)",
                default=True,
                style=WIZARD_STYLE,
                qmark="❓"
            )
    
    # Run the form with all questions (if any)
    answers = {}
    if questions:
        form_answers = questionary.form(**questions).ask()
        if form_answers is None:
            return None
        answers.update(form_answers)
    
    # Add any direct prompt results (like select_no_bg) to answers
    # Check if ssh_key_selection was set in the SSH section
    try:
        if ssh_key_selection:
            answers['ssh_key_selection'] = ssh_key_selection
    except NameError:
        pass  # ssh_key_selection wasn't set (no SSH keys or not in SSH section)
    
    # Convert boolean answers to "yes"/"no" strings to match PowerShell version
    for key, value in answers.items():
        if isinstance(value, bool):
            results[key] = "yes" if value else "no"
        else:
            results[key] = value
    
    # Handle SSH key selection special case
    if 'ssh_key_selection' in results:
        if results['ssh_key_selection'] == "Create new key":
            # If user selected "Create new key", we need the key name
            if 'ssh_new_key_name' not in questions:
                key_name = questionary.text(
                    "SSH key name",
                    default="github_pi",
                    placeholder="github_pi",
                    style=WIZARD_STYLE,
                    qmark="❓"
                ).ask()
                if key_name:
                    results['ssh_new_key_name'] = key_name
    
    return results


def main():
    """Main entry point"""
    parser = argparse.ArgumentParser(description="GitHub Setup Wizard")
    parser.add_argument("--skip-ssh", action="store_true", help="Skip SSH setup")
    parser.add_argument("--skip-local-repo", action="store_true", help="Skip local repository setup")
    parser.add_argument("--skip-remote", action="store_true", help="Skip remote repository setup")
    parser.add_argument("--output", type=str, help="Output file for results (JSON)")
    
    args = parser.parse_args()
    
    write_boxed_header("GitHub Setup")
    
    # Discover current state
    ssh_configured = test_ssh_connection()
    local_repo_exists = test_local_repo()
    has_remotes = has_existing_remotes()
    
    # Discover SSH keys
    existing_ssh_keys = []
    has_ssh_config = False
    
    if not ssh_configured and not args.skip_ssh:
        existing_ssh_keys = discover_ssh_keys()
        has_ssh_config = has_existing_ssh_config()
    
    # Get defaults
    default_github_user = get_git_config("user.name")
    default_repo_name = Path.cwd().name
    default_git_name = get_git_config("user.name")
    default_git_email = get_git_config("user.email")
    
    # Run wizard
    results = run_wizard(
        ssh_configured=ssh_configured,
        local_repo_exists=local_repo_exists,
        has_remotes=has_remotes,
        existing_ssh_keys=existing_ssh_keys,
        has_ssh_config=has_ssh_config,
        default_github_user=default_github_user,
        default_repo_name=default_repo_name,
        default_git_name=default_git_name,
        default_git_email=default_git_email,
        skip_ssh=args.skip_ssh,
        skip_local_repo=args.skip_local_repo,
        skip_remote=args.skip_remote
    )
    
    if results is None:
        print("\n⚠️  Wizard was cancelled.")
        return 1
    
    if not results:
        print("✅ No configuration needed. Everything is already set up!")
        return 0
    
    # Output results
    if args.output:
        output_path = Path(args.output)
        output_path.write_text(json.dumps(results, indent=2), encoding="utf-8")
        print(f"\n✅ Results saved to: {output_path}")
    else:
        print("\n✅ Wizard completed!")
        print("\nResults:")
        print(json.dumps(results, indent=2))
    
    return 0


if __name__ == "__main__":
    sys.exit(main())
