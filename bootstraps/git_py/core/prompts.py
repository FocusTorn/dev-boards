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
    from prompt_toolkit.layout import Layout, HSplit, VSplit, Window, ConditionalContainer
    from prompt_toolkit.layout.margins import Margin
    from prompt_toolkit.layout.controls import FormattedTextControl
    from prompt_toolkit.filters import IsDone
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
    ConditionalContainer = None
    IsDone = None
    RadioList = None

# Helper function to convert hex color to ANSI 256-color code
def _hex_to_ansi256(hex_color: str) -> str:
    """Convert hex color (#RRGGBB) to ANSI 256-color escape code."""
    # Remove # if present
    hex_color = hex_color.lstrip('#')
    
    # Parse RGB
    r = int(hex_color[0:2], 16)
    g = int(hex_color[2:4], 16)
    b = int(hex_color[4:6], 16)
    
    # Convert RGB to ANSI 256-color (6x6x6 cube + 16 base colors)
    # Formula: 16 + 36 * r + 6 * g + b, where r, g, b are in [0,5]
    # Map [0,255] to [0,5]
    r_idx = int((r / 255.0) * 5)
    g_idx = int((g / 255.0) * 5)
    b_idx = int((b / 255.0) * 5)
    
    ansi_code = 16 + 36 * r_idx + 6 * g_idx + b_idx
    return f"\x1B[38;5;{ansi_code}m"

# prompt_toolkit style dictionary - shared by all prompts
def _get_prompt_style() -> dict:
    """Get the shared prompt_toolkit style dictionary for all prompts."""
    if PTStyle is None:
        return {}
    return {
        'qmark': 'fg:#ff5faf bold',      # Pink bold (matches QMARK_COLOR)
        'question': 'fg:#ffffff bold',   # Bold white (matches QUESTION_COLOR)
        
        # 'answer': 'fg:#33658A bold',     # Blue (matches ANSWER_COLOR)
        'answer': 'fg:#FF0000 bold',     # Blue (matches ANSWER_COLOR)
        
        'pointer': 'fg:#ff5faf bold',    # Pink bold pointer
        'highlighted': 'fg:#ff5faf bold', # Pink bold selected text
        'text': 'fg:#666666',            # Grey unselected text
    }

PROMPT_STYLE = _get_prompt_style()


def _format_prompt_text(indent: str, question: str) -> str:
    """Format the prompt text with colors (fallback when prompt_toolkit not available)."""
    # Fallback ANSI codes (only used if prompt_toolkit unavailable)
    qmark_color = "\x1B[38;5;205m\x1B[1m"  # Pink bold
    question_color = "\x1B[37m\x1B[1m"  # Bold white
    return f"{indent}{qmark_color}? {RESET_COLOR}{question_color}{question}{RESET_COLOR}"


def text(question: str, default: str = "", indent: str = "", style=None) -> Optional[str]:
    """
    Text input prompt using prompt_toolkit.
    
    Args:
        question: Question text
        default: Default value
        indent: Indentation string for qmark
        style: Not used
    
    Returns:
        str or None if cancelled
    """
    if not HAS_PROMPT_TOOLKIT:
        raise ImportError("prompt_toolkit is required for text prompt")
    
    # Calculate region indent for the prompt
    region_base_indent = get_region_indent()
    
    # Build the prompt message with FormattedText
    formatted_prompt = FormattedText([
        ('class:qmark', f"{region_base_indent}? "),
        ('class:question', f"{question}: "),
    ])
    
    # Use shared prompt style, add input-specific styles
    # Input text should use the same color as answer (from PROMPT_STYLE)
    answer_color = PROMPT_STYLE.get('answer', 'fg:#33658A bold')
    style_dict = PROMPT_STYLE.copy()
    style_dict.update({
        'input': answer_color,          # Use answer color for input text
        '': answer_color,              # Use answer color for default text
    })
    custom_style = PTStyle.from_dict(style_dict)
    
    # Use prompt_toolkit's prompt with formatted text
    # The default parameter will pre-fill the input field
    try:
        result = pt_prompt_func(
            formatted_prompt,
            default=default,
            style=custom_style,
        )
        return result.strip() if result else default
    except (EOFError, KeyboardInterrupt):
        return None


def select(question: str, choices: List[str], indent: str = "", pointer: str = " »", style=None, warning_mode: bool = False) -> Optional[str]:
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
    if not HAS_PROMPT_TOOLKIT or RadioList is None or Application is None or Layout is None:
        raise ImportError("prompt_toolkit is required for select prompt")
    
    if ConditionalContainer is None or IsDone is None:
        raise ImportError("Required prompt_toolkit components not available")
    
    # Calculate region indent for the prompt
    region_base_indent = get_region_indent()
    
    # Track selected choice and answered state
    selected_choice = [None]  # Use list to allow modification in nested functions
    is_answered = [False]
    current_index = [0]
    
    # Extract pointer character (remove leading space if present)
    pointer_char = pointer.lstrip() if pointer.startswith(' ') else pointer
    
    # Function to get prompt tokens (question + answer when answered)
    def get_prompt_tokens():
        tokens = []
        if warning_mode:
            # Warning mode: no question line - choices appear directly below warning
            # Return empty FormattedText so no prompt line is shown
            return FormattedText([])
        else:
            # Normal mode: pink qmark and white question
            tokens.append(('class:qmark', f"{region_base_indent}? "))
            if question:
                tokens.append(('class:question', f"{question}"))
        
        if is_answered[0] and selected_choice[0] is not None:
            # Show answer inline when answered
            tokens.append(('class:answer', f" {selected_choice[0]}"))
        
        return FormattedText(tokens)
    
    # Function to get choice tokens (menu display)
    def get_choice_tokens():
        """Generate formatted text for all choices with dynamic alignment."""
        fragments = []
        
        # Calculate indentation for unselected choices
        # Selected: region_indent + pointer + space
        selected_text_start_pos = len(region_base_indent) + len(pointer_char) + 1
        unselected_indent = " " * selected_text_start_pos
        
        for i, choice in enumerate(choices):
            if i == current_index[0]:
                # Selected: pink pointer + pink bold text
                fragments.append(('class:pointer', f"{region_base_indent}{pointer_char} "))
                fragments.append(('class:highlighted', choice))
            else:
                # Not selected: grey text aligned to selected text's first character
                fragments.append(('class:text', f"{unselected_indent}{choice}"))
            if i < len(choices) - 1:
                fragments.append(('', '\n'))
        return FormattedText(fragments)
    
    # Create FormattedTextControl for question line (read-only, no cursor)
    prompt_control = FormattedTextControl(get_prompt_tokens, show_cursor=False)
    
    # Create FormattedTextControl for choices (like questionary's InquirerControl)
    choice_control = FormattedTextControl(get_choice_tokens, show_cursor=False)
    
    # Create layout: question line + conditional menu (hidden when IsDone)
    # If warning_mode, skip the question line entirely
    if warning_mode:
        # No question line - just show choices directly
        layout = Layout(HSplit([
            ConditionalContainer(
                Window(choice_control, height=len(choices)),
                filter=~IsDone()  # Hide menu when answered
            )
        ]))
    else:
        # Normal mode: question line + choices
        # Use FormattedTextControl for question line (read-only, no cursor)
        layout = Layout(HSplit([
            Window(prompt_control, height=1),  # Question line (read-only)
            ConditionalContainer(
                Window(choice_control, height=len(choices)),
                filter=~IsDone()  # Hide menu when answered
            )
        ]))
    
    # Use shared prompt style (standard styling for all prompts)
    custom_style = PTStyle.from_dict(PROMPT_STYLE)
    
    # Create key bindings
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
        selected_choice[0] = choices[current_index[0]]
        is_answered[0] = True
        event.app.exit(result=selected_choice[0])
    
    @kb.add('c-c')
    def cancel(event):
        event.app.exit(result=None)
    
    # Create application
    app = Application(
        layout=layout,
        style=custom_style,
        key_bindings=kb,
        full_screen=False,
    )
    
    # Run the application - prompt_toolkit handles clearing automatically via IsDone filter!
    # prompt_toolkit automatically adds a newline when the application exits
    result = app.run()
    
    if result is not None:
        # If warning_mode with no question, choices are cleared automatically by IsDone filter
        # The answer will be shown inline if there's a question, otherwise it's just returned
        return result
    return None


def confirm(question: str, default: bool = True, indent: str = "", style=None) -> Optional[bool]:
    """
    Confirmation prompt (y/n) using prompt_toolkit.
    
    Format: {{qmark}} {{message}} [Y/n]? y
    - [Y/n] is in dim grey, default letter capitalized
    - Default answer (y or n) is shown in answer color after ?
    - y submits True, n submits False, Enter submits default
    
    Args:
        question: Question text (without [Y/n] - it's added automatically)
        default: Default value (True for Yes, False for No)
        indent: Indentation string for qmark
        style: Not used
    
    Returns:
        bool or None if cancelled
    """
    if not HAS_PROMPT_TOOLKIT or PromptSession is None or Application is None:
        raise ImportError("prompt_toolkit is required for confirm prompt")
    
    default_choice = "y" if default else "n"
    default_text = "[Y/n]" if default else "[y/N]"
    
    # Calculate region indent for the prompt
    region_base_indent = get_region_indent()
    
    # Track selected answer and answered state
    selected_answer = [None]
    is_answered = [False]
    
    # Function to get prompt tokens (question + answer when answered)
    def get_prompt_tokens():
        tokens = []
        # Qmark and question
        tokens.append(('class:qmark', f"{region_base_indent}? "))
        tokens.append(('class:question', f"{question} "))
        
        if not is_answered[0]:
            # Show [Y/n] indicator and default answer
            tokens.append(('class:dim', default_text))
            tokens.append(('class:question', "? "))
            tokens.append(('class:answer', default_choice))
        else:
            # Show only selected answer (no [Y/n] indicator)
            tokens.append(('class:question', "? "))
            tokens.append(('class:answer', selected_answer[0] if selected_answer[0] else default_choice))
        
        return tokens
    
    # Create PromptSession for the question line
    prompt_session = PromptSession(
        get_prompt_tokens,
        reserve_space_for_menu=0
    )
    
    # Create layout with just the prompt
    layout = Layout(prompt_session.layout.container)
    
    # Use shared prompt style, add dim style for [Y/n] indicator
    style_dict = PROMPT_STYLE.copy()
    style_dict.update({
        'dim': 'fg:#666666',  # Dim grey for [Y/n] indicator
    })
    custom_style = PTStyle.from_dict(style_dict)
    
    # Create key bindings
    kb = KeyBindings()
    
    @kb.add('y')
    @kb.add('Y')
    def select_yes(event):
        selected_answer[0] = 'y'
        is_answered[0] = True
        event.app.exit(result=True)
    
    @kb.add('n')
    @kb.add('N')
    def select_no(event):
        selected_answer[0] = 'n'
        is_answered[0] = True
        event.app.exit(result=False)
    
    @kb.add(Keys.Enter)
    def submit_default(event):
        is_answered[0] = True
        event.app.exit(result=default)
    
    @kb.add('c-c')
    def cancel(event):
        event.app.exit(result=None)
    
    # Create application
    app = Application(
        layout=layout,
        style=custom_style,
        key_bindings=kb,
        full_screen=False,
    )
    
    # Run the application
    # prompt_toolkit automatically adds a newline when the application exits
    result = app.run()
    
    if result is not None:
        return result
    return None
    
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


