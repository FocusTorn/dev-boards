"""
Interactive prompt utilities using prompt_toolkit directly.
"""

import sys
from typing import Optional, List

# Import region indent function
try:
    from .terminal import get_region_indent
except ImportError:
    # Fallback if terminal module not available
    def get_region_indent() -> str:
        return ""

# Try to import prompt_toolkit
try:
    from prompt_toolkit import prompt as pt_prompt, PromptSession
    from prompt_toolkit.shortcuts import prompt as pt_prompt_func
    from prompt_toolkit.styles import Style as PTStyle
    from prompt_toolkit.key_binding import KeyBindings
    from prompt_toolkit.keys import Keys
    from prompt_toolkit.application import Application
    from prompt_toolkit.layout import Layout, HSplit, VSplit
    from prompt_toolkit.widgets import RadioList
    from prompt_toolkit.formatted_text import FormattedText
    HAS_PROMPT_TOOLKIT = True
except ImportError:
    HAS_PROMPT_TOOLKIT = False
    pt_prompt_func = None
    PromptSession = None
    PTStyle = None
    KeyBindings = None
    Application = None
    Layout = None
    RadioList = None

# ANSI color codes
QMARK_COLOR = "\x1B[38;5;205m\x1B[1m"  # Pink bold
QUESTION_COLOR = "\x1B[37m\x1B[1m"  # Bold white
ANSWER_COLOR = "\x1B[38;5;61m\x1B[1m"  # Blue bold
TEXT_COLOR = "\x1B[38;5;102m"  # Grey
DIM_GREY = "\x1B[2m\x1B[38;5;102m"  # Dim grey
POINTER_COLOR = "\x1B[38;5;205m\x1B[1m"  # Pink bold
HIGHLIGHT_COLOR = "\x1B[38;5;205m\x1B[1m"  # Pink bold
RESET_COLOR = "\x1B[0m"



def _format_prompt_text(indent: str, question: str) -> str:
    """Format the prompt text with colors."""
    return f"{indent}{QMARK_COLOR}? {RESET_COLOR}{QUESTION_COLOR}{question}{RESET_COLOR}"


def text(question: str, default: str = "", indent: str = "", style=None) -> Optional[str]:
    """
    Text input prompt.
    
    Args:
        question: Question text
        default: Default value
        indent: Indentation string for qmark
        style: Not used
    
    Returns:
        str or None if cancelled
    """
    if not HAS_PROMPT_TOOLKIT:
        # Fallback - use ANSI codes directly
        prompt_text = _format_prompt_text(indent, question)
        if default:
            prompt_text += f" [{default}]"
        try:
            response = input(f"{prompt_text}: ").strip()
            return response if response else default
        except (EOFError, KeyboardInterrupt):
            return None
    
    # Use prompt_toolkit's FormattedText for proper styling
    try:
        # Build the prompt message with FormattedText (no default in brackets)
        formatted_prompt = FormattedText([
            ('class:qmark', f"{indent}? "),
            ('class:question', question),
            ('', ": "),
        ])
        
        # Create custom style matching questionary's look
        # Blue for input/answer text (fg:#33658A is the blue from questionary)
        custom_style = PTStyle.from_dict({
            'qmark': 'fg:#ff5faf bold',      # Pink bold
            'question': 'fg:#ffffff bold',  # White bold
            'input': 'fg:#33658A bold',     # Blue bold for input text
            '': 'fg:#33658A bold',          # Blue bold for default text
        })
        
        # Use prompt_toolkit's prompt with formatted text
        # The default parameter will pre-fill the input field
        result = pt_prompt_func(
            formatted_prompt,
            default=default,
            style=custom_style,
        )
        return result.strip() if result else default
    except (EOFError, KeyboardInterrupt):
        return None
    except Exception as e:
        # Fallback to simple input if prompt_toolkit fails
        prompt_text = _format_prompt_text(indent, question)
        if default:
            prompt_text += f" [{default}]"
        try:
            response = input(f"{prompt_text}: ").strip()
            return response if response else default
        except (EOFError, KeyboardInterrupt):
            return None


def select(question: str, choices: List[str], indent: str = "", pointer: str = " »", style=None) -> Optional[str]:
    """
    Select prompt with dropdown menu using arrow keys.
    
    Args:
        question: Question text
        choices: List of choice strings
        indent: Indentation string for qmark
        pointer: Pointer character (default: " »")
        style: Not used
    
    Returns:
        Selected choice string or None if cancelled
    """
    prompt_text = _format_prompt_text(indent, question)
    
    if not HAS_PROMPT_TOOLKIT or RadioList is None or Application is None or Layout is None:
        # Fallback: simple numbered menu
        print(prompt_text)
        for i, choice in enumerate(choices, 1):
            print(f"  {i}. {choice}")
        
        while True:
            try:
                response = input(f"Enter choice (1-{len(choices)}): ").strip()
                choice_num = int(response)
                if 1 <= choice_num <= len(choices):
                    return choices[choice_num - 1]
                print(f"Invalid choice. Please enter a number between 1 and {len(choices)}")
            except ValueError:
                print("Invalid input. Please enter a number.")
            except (EOFError, KeyboardInterrupt):
                return None
    
    # Use prompt_toolkit with custom display (matching questionary style)
    try:
        from prompt_toolkit.layout import Window
        from prompt_toolkit.formatted_text import FormattedText
        
        # Calculate dynamic indentation based on prompt message
        # Region system handles base indentation (2 spaces), so we calculate relative to that
        # 
        # Layout:
        #   ? Repository visibility:  (region adds 2 spaces, qmark "? " at columns 3-4)
        #   » Public                    (region adds 2 spaces, pointer " »" at columns 3-4, space at 5, text at 6)
        #     Private                   (region adds 2 spaces, then 2 more = 4 total, text at column 5)
        #
        # Wait, user wants unselected to align with selected text's first character
        # Selected: "  » Public" - text starts at column 6 (after "  » ")
        # Unselected: "    Private" - text starts at column 5 (after "    ")
        # They don't align!
        #
        # User said: "then it would look for the actual option text and its first char is at col4"
        # So selected text's first char should be at column 4
        # And "all the unselected text would use that indent=4" (4 spaces total)
        #
        # If selected text is at column 4, and pointer " »" is 2 chars:
        # - Region adds 2 spaces (columns 1-2)
        # - Pointer " »" at columns 3-4
        # - Space at column 5
        # - Text at column 6
        #
        # That doesn't match column 4. Maybe no space after pointer?
        # - Region adds 2 spaces (columns 1-2)
        # - Pointer " »" at columns 3-4
        # - Text at column 5
        #
        # Still not column 4. Maybe pointer is just "»" (1 char)?
        # - Region adds 2 spaces (columns 1-2)
        # - Pointer "»" at column 3
        # - Space at column 4
        # - Text at column 5
        #
        # Still not column 4. Let me re-read user's message...
        # User shows: "  » Public" and "    Private"
        # If "Public" starts at column 6 and "Private" starts at column 5, they don't align
        #
        # Actually, looking at the user's desired output again:
        #   ? Repository visibility:
        #   » Public
        #     Private
        #
        # Both "Public" and "Private" seem to start at the same column visually
        # Let me count: "  » Public" - if pointer is " »" (2 chars), then "Public" is at column 6
        # "    Private" - 4 spaces, so "Private" is at column 5
        #
        # They're off by 1. Maybe user wants both at column 5?
        # Then selected needs: region(2) + pointer(2) + space(1) = 5, but that's 5 chars, text at column 6
        #
        # Let me just implement: pointer aligns to qmark, add space, align unselected to selected text
        # We'll adjust based on user feedback
        
        # Pointer aligns to qmark position
        # prompt_toolkit Application bypasses region system's stdout wrapper,
        # so we need to manually add the region indent to match the prompt message
        region_base_indent = get_region_indent()
        pointer_indent = region_base_indent
        
        # Selected text's first character position:
        # Region indent (e.g., 2 spaces) + pointer "»" (1 char) + space (1 char)
        # Since prompt_toolkit bypasses region system, we need full indent: region + pointer + space
        pointer_char = pointer.lstrip() if pointer.startswith(' ') else pointer
        selected_text_start_pos = len(region_base_indent) + len(pointer_char) + 1  # pointer + space
        unselected_indent = " " * selected_text_start_pos
        
        current_index = [0]  # Use list to allow modification in nested function
        
        # Create key bindings for arrow keys
        kb = KeyBindings()
        
        @kb.add(Keys.Up)
        def move_up(event):
            if current_index[0] > 0:
                current_index[0] -= 1
                event.app.invalidate()
        
        @kb.add(Keys.Down)
        def move_down(event):
            if current_index[0] < len(choices) - 1:
                current_index[0] += 1
                event.app.invalidate()
        
        @kb.add(Keys.Enter)
        def select_choice(event):
            event.app.exit(result=choices[current_index[0]])
        
        @kb.add('c-c')
        def cancel(event):
            event.app.exit(result=None)
        
        # Create a FormattedTextControl that displays the choices
        from prompt_toolkit.layout.controls import FormattedTextControl
        
        def get_choice_text():
            """Generate formatted text for all choices with dynamic alignment."""
            fragments = []
            # Extract just the » character from pointer (remove leading space if present)
            # We want » to align with ? character
            pointer_char = pointer.lstrip() if pointer.startswith(' ') else pointer
            # pointer_char is now "»" (just the symbol)
            
            for i, choice in enumerate(choices):
                if i == current_index[0]:
                    # Selected: pink pointer + pink bold text
                    # Pointer "»" aligns to qmark "?" position (both have same indent from region)
                    # Add space after pointer before choice text
                    fragments.append(('class:pointer', f"{pointer_indent}{pointer_char} "))
                    fragments.append(('class:highlighted', choice))
                else:
                    # Not selected: grey text aligned to selected text's first character
                    # Unselected indent already calculated to align with selected text
                    fragments.append(('class:text', f"{unselected_indent}{choice}"))
                if i < len(choices) - 1:
                    fragments.append(('', '\n'))
            return FormattedText(fragments)
        
        # Create control and window
        choice_control = FormattedTextControl(get_choice_text)
        choice_window = Window(
            content=choice_control,
            height=len(choices),
        )
        
        layout = Layout(choice_window)
        
        # Create custom style
        custom_style = PTStyle.from_dict({
            'pointer': 'fg:#ff5faf bold',      # Pink bold pointer
            'highlighted': 'fg:#ff5faf bold',  # Pink bold selected text
            'text': 'fg:#666666',              # Grey unselected text
        })
        
        # Create application
        app = Application(
            layout=layout,
            style=custom_style,
            key_bindings=kb,
            full_screen=False,
        )
        
        # Display prompt first - region system handles base indentation automatically
        # Don't add indent to prompt text, region system adds it
        # Qmark is always present in our prompts, so always show it
        print(f"{QMARK_COLOR}? {RESET_COLOR}{QUESTION_COLOR}{question}{RESET_COLOR}")
        
        # Run the application
        result = app.run()
        
        if result is not None:
            return result
        return None
    except Exception as e:
        # Fallback to simple menu if RadioList fails
        prompt_text = _format_prompt_text(indent, question)
        print(prompt_text)
        for i, choice in enumerate(choices, 1):
            print(f"  {i}. {choice}")
        
        while True:
            try:
                response = input(f"Enter choice (1-{len(choices)}): ").strip()
                choice_num = int(response)
                if 1 <= choice_num <= len(choices):
                    return choices[choice_num - 1]
                print(f"Invalid choice. Please enter a number between 1 and {len(choices)}")
            except ValueError:
                print("Invalid input. Please enter a number.")
            except (EOFError, KeyboardInterrupt):
                return None


def confirm(question: str, default: bool = True, indent: str = "", style=None) -> Optional[bool]:
    """
    Confirmation prompt (y/n).
    
    Format: {{qmark}} {{message}} [Y/n]? y
    - [Y/n] is in dim grey, default letter capitalized
    - Default answer (y or n) is shown in blue after ?
    - y submits True, n submits False, Enter submits default
    
    Args:
        question: Question text (without [Y/n] - it's added automatically)
        default: Default value (True for Yes, False for No)
        indent: Indentation string for qmark
        style: Not used
    
    Returns:
        bool or None if cancelled
    """
    default_choice = "y" if default else "n"
    default_text = "[Y/n]" if default else "[y/N]"
    
    # Get region indent for final display (when using \r to overwrite)
    # Initial print uses indent only - IndentedOutput wrapper adds region indent automatically
    # But for \r overwrite, we need to manually include region indent since we're at column 0
    region_base_indent = get_region_indent()
    
    # Format: {{qmark}} {{message}} [Y/n]? {{default_answer}}
    # [Y/n] is dim grey, default answer is blue
    # Use only indent parameter - IndentedOutput wrapper adds region indent automatically
    prompt_text = (
        f"{indent}{QMARK_COLOR}? {RESET_COLOR}"
        f"{QUESTION_COLOR}{question} {RESET_COLOR}"
        f"{DIM_GREY}{default_text}{RESET_COLOR}"
        f"? {ANSWER_COLOR}{default_choice}{RESET_COLOR}"
    )
    print(prompt_text, end='', flush=True)
    
    # Try Unix-style single character input first (works on Linux/Mac)
    try:
        import tty
        import termios
        
        # Save terminal settings
        fd = sys.stdin.fileno()
        old_settings = termios.tcgetattr(fd)
        
        try:
            # Set terminal to raw mode for single character input (no echo)
            tty.setraw(sys.stdin.fileno())
            
            # Read single character (no echo in raw mode)
            char = sys.stdin.read(1)
            
            # Restore terminal settings immediately after reading
            termios.tcsetattr(fd, termios.TCSADRAIN, old_settings)
            
            # Handle the character
            if char.lower() == 'y':
                # Clear the line and show only 'y' in blue (no [Y/n] indicator)
                # For \r overwrite, manually include region indent since we're at column 0
                # Use \033[K to clear from cursor to end of line
                full_indent = region_base_indent + indent
                print(f"\r{full_indent}{QMARK_COLOR}? {RESET_COLOR}{QUESTION_COLOR}{question} {RESET_COLOR}? {ANSWER_COLOR}y{RESET_COLOR}\033[K", flush=True)
                print()  # New line after prompt
                return True
            elif char.lower() == 'n':
                # Clear the line and show only 'n' in blue (no [Y/n] indicator)
                # For \r overwrite, manually include region indent since we're at column 0
                # Use \033[K to clear from cursor to end of line
                full_indent = region_base_indent + indent
                print(f"\r{full_indent}{QMARK_COLOR}? {RESET_COLOR}{QUESTION_COLOR}{question} {RESET_COLOR}? {ANSWER_COLOR}n{RESET_COLOR}\033[K", flush=True)
                print()  # New line after prompt
                return False
            elif char == '\r' or char == '\n':  # Enter key
                # Keep default answer (already shown in blue), then submit
                print()  # New line after prompt
                return default
            elif char == '\x03':  # Ctrl+C
                print('', flush=True)
                return None
            else:
                # Invalid input, use default (already shown)
                print()  # New line after prompt
                return default
        finally:
            # Restore terminal settings (fallback in case of exception)
            try:
                termios.tcsetattr(fd, termios.TCSADRAIN, old_settings)
            except:
                pass
    except (ImportError, AttributeError, OSError):
        # Fallback for Windows or if termios not available
        # On Windows, we need to use msvcrt.getch() for single character input
        try:
            import msvcrt
            # Read a single character (no echo, immediate return)
            char = msvcrt.getch()
            if isinstance(char, bytes):
                char = char.decode('utf-8', errors='ignore')
            else:
                char = str(char)
            
            # Handle the character
            char_lower = char.lower()
            if char_lower == 'y':
                # Clear the line and show only 'y' in blue (no [Y/n] indicator)
                # For \r overwrite, manually include region indent since we're at column 0
                # Use \033[K to clear from cursor to end of line
                full_indent = region_base_indent + indent
                print(f"\r{full_indent}{QMARK_COLOR}? {RESET_COLOR}{QUESTION_COLOR}{question} {RESET_COLOR}? {ANSWER_COLOR}y{RESET_COLOR}\033[K", flush=True)
                print()  # New line after prompt
                return True
            elif char_lower == 'n':
                # Clear the line and show only 'n' in blue (no [Y/n] indicator)
                # For \r overwrite, manually include region indent since we're at column 0
                # Use \033[K to clear from cursor to end of line
                full_indent = region_base_indent + indent
                print(f"\r{full_indent}{QMARK_COLOR}? {RESET_COLOR}{QUESTION_COLOR}{question} {RESET_COLOR}? {ANSWER_COLOR}n{RESET_COLOR}\033[K", flush=True)
                print()  # New line after prompt
                return False
            elif char == '\r' or char == '\n' or ord(char) == 13:  # Enter key (Windows uses \r)
                # Default already shown in blue, just submit
                print()  # New line after prompt
                return default
            elif ord(char) == 3:  # Ctrl+C
                print('', flush=True)
                return None
            else:
                # Invalid input, use default (already shown)
                print()  # New line after prompt
                return default
        except (ImportError, AttributeError, OSError, ValueError):
            # Final fallback: use input() - this requires Enter to be pressed
            # This is not ideal but works as a last resort
            try:
                response = input().strip().lower()
            except (EOFError, KeyboardInterrupt):
                print('', flush=True)
                return None
            
            if not response:
                return default
            if response in ['y', 'yes']:
                return True
            elif response in ['n', 'no']:
                return False
            return default


