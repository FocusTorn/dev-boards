"""
Interactive prompt utilities using prompt_toolkit directly.
"""

import sys
from typing import Optional, List, Callable, Union

# Optional region indent function (can be set by user)
_region_indent_func: Optional[Callable[[], str]] = None

# Try to auto-detect indeneder package
try:
    from indeneder import get_region_indent as _indeneder_get_region_indent
    _HAS_INDENEDER = True
except ImportError:
    _HAS_INDENEDER = False
    _indeneder_get_region_indent = None


def set_region_indent_func(func: Callable[[], str]) -> None:
    """
    Set a function that returns the current region indentation.
    
    This is useful when integrating with terminal output systems that use
    region-based indentation (like git-py's terminal module or indeneder).
    
    Args:
        func: A callable that returns a string representing the current indentation
    
    Note:
        If indeneder package is installed, it will be used automatically.
        This function allows overriding with a custom function.
    """
    global _region_indent_func
    _region_indent_func = func


def _get_region_indent() -> str:
    """
    Get the current indentation string based on active regions.
    
    Priority order:
    1. User-set function (via set_region_indent_func)
    2. Auto-detected indeneder package
    3. Empty string (no indentation)
    """
    # First check if user has set a custom function
    if _region_indent_func is not None:
        return _region_indent_func()
    
    # Then check if indeneder is available
    if _HAS_INDENEDER and _indeneder_get_region_indent is not None:
        try:
            return _indeneder_get_region_indent()
        except:
            # If indeneder fails, fall back to no indentation
            pass
    
    # Default: no indentation
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

# Default prompt_toolkit style dictionary
_DEFAULT_PROMPT_STYLE = {
    'qmark': 'fg:#ff5faf bold',      # Pink bold
    'question': 'fg:#ffffff bold',   # Bold white
    'answer': 'fg:#33658A bold',     # Blue (matches ANSWER_COLOR)
    'pointer': 'fg:#ff5faf bold',    # Pink bold pointer
    'highlighted': 'fg:#ff5faf bold', # Pink bold selected text
    'text': 'fg:#666666',            # Grey unselected text
}

# Global style overrides (can be set via set_global_style())
_global_style_overrides: dict = {}

# Named style dictionaries (can be registered and applied at prompt level)
_named_styles: dict = {}


def register_style(name: str, style_dict: dict) -> None:
    """
    Register a named style dictionary that can be applied at the prompt level.
    
    Args:
        name: Name identifier for the style (e.g., "green_theme", "minimal")
        style_dict: Dictionary of style class names to style strings
    
    Example:
        >>> register_style("green_theme", {'qmark': 'fg:#00ff00 bold', 'pointer': 'fg:#00ff00 >'})
        >>> # Later use it in prompts
        >>> choice = select("Choose:", ["A", "B"], style="green_theme")
    """
    global _named_styles
    _named_styles[name] = style_dict.copy()


def get_named_style(name: str) -> Optional[dict]:
    """
    Get a registered named style dictionary.
    
    Args:
        name: Name of the registered style
    
    Returns:
        Style dictionary or None if not found
    """
    return _named_styles.get(name)


def set_global_style(overrides: dict) -> None:
    """
    Set global style overrides that apply to all prompts.
    
    Args:
        overrides: Dictionary of style class names to style strings.
                  Example: {'qmark': 'fg:#00ff00 bold', 'question': 'fg:#ffffff'}
    
    Example:
        >>> set_global_style({'qmark': 'fg:#00ff00 bold', 'question': 'fg:#ffffff'})
        >>> # All subsequent prompts will use green qmark instead of pink
    """
    global _global_style_overrides
    _global_style_overrides = overrides.copy()


def get_prompt_style(additional_overrides: Optional[Union[dict, str]] = None) -> dict:
    """
    Get the prompt style dictionary, merging defaults, global overrides, and per-prompt overrides.
    
    Args:
        additional_overrides: Optional dictionary or string name of registered style, or dict of additional style overrides for this specific prompt
    
    Returns:
        Merged style dictionary
    """
    if PTStyle is None:
        return {}
    
    # Start with defaults
    style = _DEFAULT_PROMPT_STYLE.copy()
    
    # Apply global overrides
    style.update(_global_style_overrides)
    
    # Apply per-prompt overrides (highest priority)
    if additional_overrides:
        # If it's a string, look up the named style
        if isinstance(additional_overrides, str):
            named_style = get_named_style(additional_overrides)
            if named_style:
                style.update(named_style)
            # If named style not found, treat as empty (could raise error, but silently ignore for now)
        elif isinstance(additional_overrides, dict):
            # It's a dictionary, apply it directly
            style.update(additional_overrides)
    
    return style


def text(question: str, default: str = "", indent: str = "", style: Optional[Union[dict, str]] = None) -> Optional[str]:
    """
    Text input prompt using prompt_toolkit.
    
    Args:
        question: Question text
        default: Default value
        indent: Additional indentation string for qmark (region indent is added automatically)
        style: Optional dictionary of style overrides or string name of registered style for this specific prompt.
               Example: {'qmark': 'fg:#00ff00 bold', 'question': 'fg:#ffffff'}
               Or: "green_theme" (if registered via register_style())
    
    Returns:
        str or None if cancelled
    
    Example:
        >>> name = text("What is your name?", default="John")
        ? What is your name?: John
        
        >>> name = text("What is your name?", style={'qmark': 'fg:#00ff00 bold'})
        # Uses green qmark instead of default pink
        
        >>> register_style("green", {'qmark': 'fg:#00ff00 bold'})
        >>> name = text("What is your name?", style="green")
        # Uses registered "green" style
    """
    if not HAS_PROMPT_TOOLKIT or PromptSession is None:
        raise ImportError("prompt_toolkit is required for text prompt")
    
    # Calculate region indent for the prompt
    region_base_indent = _get_region_indent()
    
    # Build the prompt message with FormattedText
    # Remove trailing colon from question if present (we add it ourselves)
    question_clean = question.rstrip(': ')
    formatted_prompt = FormattedText([
        ('class:qmark', f"{region_base_indent}{indent}? "),
        ('class:question', f"{question_clean}: "),
    ])
    
    # Get merged style (defaults + global overrides + per-prompt overrides)
    style_dict = get_prompt_style(style)
    
    # Add input-specific styles
    # Input text should use the same color as answer
    answer_color = style_dict.get('answer', 'fg:#33658A bold')
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


def select(question: str, choices: List[str], indent: str = "", pointer: str = " »", style: Optional[Union[dict, str]] = None, warning_mode: bool = False) -> Optional[str]:
    """
    Select prompt with dropdown menu using arrow keys.
    
    Args:
        question: Question text
        choices: List of choice strings
        indent: Additional indentation string for qmark (region indent is added automatically)
        pointer: Pointer character (default: " »")
        style: Optional dictionary of style overrides or string name of registered style for this specific prompt.
               Example: {'pointer': 'fg:#00ff00 bold', 'highlighted': 'fg:#ffff00'}
               Or: "green_theme" (if registered via register_style())
        warning_mode: If True, don't show question line, just choices
    
    Returns:
        Selected choice string or None if cancelled
    
    Example:
        >>> choice = select("Choose an option:", ["Option 1", "Option 2", "Option 3"])
        ? Choose an option: Option 1
         » Option 1
           Option 2
           Option 3
        
        >>> choice = select("Choose:", ["A", "B"], style={'pointer': 'fg:#00ff00 bold'})
        # Uses green pointer instead of default pink
        
        >>> register_style("green", {'pointer': 'fg:#00ff00 bold'})
        >>> choice = select("Choose:", ["A", "B"], style="green")
        # Uses registered "green" style
    """
    if not HAS_PROMPT_TOOLKIT or RadioList is None or Application is None or Layout is None:
        raise ImportError("prompt_toolkit is required for select prompt")
    
    if ConditionalContainer is None or IsDone is None or PromptSession is None:
        raise ImportError("Required prompt_toolkit components not available")
    
    # Calculate region indent for the prompt
    region_base_indent = _get_region_indent()
    
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
            tokens.append(('class:qmark', f"{region_base_indent}{indent}? "))
            if question:
                tokens.append(('class:question', f"{question}"))
        
        if is_answered[0] and selected_choice[0] is not None:
            # Show answer inline when answered
            tokens.append(('class:answer', f" {selected_choice[0]}"))
        
        # Return FormattedText instead of list for proper formatting
        return FormattedText(tokens)
    
    # Function to get choice tokens (menu display)
    def get_choice_tokens():
        """Generate formatted text for all choices with dynamic alignment."""
        fragments = []
        
        # Calculate indentation for unselected choices
        # Selected: region_indent + indent + pointer + space
        selected_text_start_pos = len(region_base_indent) + len(indent) + len(pointer_char) + 1
        unselected_indent = " " * selected_text_start_pos
        
        for i, choice in enumerate(choices):
            if i == current_index[0]:
                # Selected: pink pointer + pink bold text
                fragments.append(('class:pointer', f"{region_base_indent}{indent}{pointer_char} "))
                fragments.append(('class:highlighted', choice))
            else:
                # Not selected: grey text aligned to selected text's first character
                fragments.append(('class:text', f"{unselected_indent}{choice}"))
            if i < len(choices) - 1:
                fragments.append(('', '\n'))
        return FormattedText(fragments)
    
    # Create FormattedTextControl for prompt line (question)
    # Set show_cursor=False to hide cursor (read-only display)
    prompt_control = FormattedTextControl(get_prompt_tokens, show_cursor=False)
    
    # Create FormattedTextControl for choices (like questionary's InquirerControl)
    # Set show_cursor=False to hide cursor during selection
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
        # Normal mode: question line + choices (no spacing between them)
        layout = Layout(HSplit([
            Window(prompt_control, height=1),  # Single line for prompt
            ConditionalContainer(
                Window(choice_control, height=len(choices)),
                filter=~IsDone()  # Hide menu when answered
            )
        ]))
    
    # Get merged style (defaults + global overrides + per-prompt overrides)
    style_dict = get_prompt_style(style)
    custom_style = PTStyle.from_dict(style_dict)
    
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
        # Invalidate to update prompt line with answer
        event.app.invalidate()
        # Small delay to ensure the answer is displayed before exiting
        event.app.exit(result=selected_choice[0])
    
    @kb.add('c-c')
    def cancel(event):
        event.app.exit(result=None)
    
    # Create application
    from prompt_toolkit.output.defaults import create_output
    
    output = create_output()
    
    # Hide cursor using ANSI escape codes
    # We need to hide it before creating the app and keep it hidden during the app
    HIDE_CURSOR = "\033[?25l"
    SHOW_CURSOR = "\033[?25h"
    
    # Hide cursor before app starts
    try:
        # Write directly to terminal (bypassing prompt_toolkit's output)
        # This ensures the cursor is hidden before prompt_toolkit takes control
        if hasattr(output, 'write_raw'):
            output.write_raw(HIDE_CURSOR)
            output.flush()
        else:
            # Fallback: write directly to stdout
            sys.stdout.write(HIDE_CURSOR)
            sys.stdout.flush()
    except:
        pass  # Ignore if cursor hiding fails
    
    app = Application(
        layout=layout,
        style=custom_style,
        key_bindings=kb,
        full_screen=False,
        output=output,  # Use our output object
        erase_when_done=False,  # Don't erase - we want to show the answer
    )
    
    try:
        # Run the application - prompt_toolkit handles clearing automatically via IsDone filter!
        # prompt_toolkit automatically adds a newline when the application exits
        result = app.run()
        
        # After the app exits, display the answer on the prompt line
        # The choices are hidden by IsDone filter, but we need to show the answer inline
        if result is not None and not warning_mode:
            # Calculate position and print the answer inline with the question
            answer_indent = region_base_indent + indent
            # Get style colors for formatting
            qmark_style = style_dict.get('qmark', '')
            question_style = style_dict.get('question', '')
            answer_style = style_dict.get('answer', '')
            # Use output object to write the answer
            try:
                if hasattr(output, 'write_raw'):
                    # Write the answer with proper formatting
                    output.write_raw(f"\r{answer_indent}? {question} {result}\033[K\n")
                    output.flush()
                else:
                    # Fallback: use print
                    print(f"\r{answer_indent}? {question} {result}\033[K", end='', flush=True)
                    print()  # New line after showing answer
            except:
                # Final fallback
                print(f"\r{answer_indent}? {question} {result}\033[K", end='', flush=True)
                print()  # New line after showing answer
        
        if result is not None:
            return result
        return None
    finally:
        # Always restore cursor, regardless of how the prompt exits
        # (selection, cancellation, error, etc.)
        try:
            if hasattr(output, 'write_raw'):
                output.write_raw(SHOW_CURSOR)
                output.flush()
            else:
                # Fallback: write directly to stdout
                sys.stdout.write(SHOW_CURSOR)
                sys.stdout.flush()
        except:
            pass  # Ignore if cursor showing fails


def confirm(question: str, default: bool = True, indent: str = "", style: Optional[Union[dict, str]] = None) -> Optional[bool]:
    """
    Confirmation prompt (y/n) using prompt_toolkit.
    
    Format: {{qmark}} {{message}} [Y/n]? y
    - [Y/n] is in dim grey, default letter capitalized
    - Default answer (y or n) is shown in answer color after ?
    - y submits True, n submits False, Enter submits default
    
    Args:
        question: Question text (without [Y/n] - it's added automatically)
        default: Default value (True for Yes, False for No)
        indent: Additional indentation string for qmark (region indent is added automatically)
        style: Optional dictionary of style overrides or string name of registered style for this specific prompt.
               Example: {'qmark': 'fg:#00ff00 bold', 'answer': 'fg:#ffff00 bold'}
               Or: "green_theme" (if registered via register_style())
    
    Returns:
        bool or None if cancelled
    
    Example:
        >>> proceed = confirm("Proceed with initialization?", default=True)
        ? Proceed with initialization? [Y/n]? y
        
        >>> proceed = confirm("Continue?", style={'qmark': 'fg:#00ff00 bold'})
        # Uses green qmark instead of default pink
        
        >>> register_style("green", {'qmark': 'fg:#00ff00 bold'})
        >>> proceed = confirm("Continue?", style="green")
        # Uses registered "green" style
    """
    if not HAS_PROMPT_TOOLKIT or PromptSession is None or Application is None:
        raise ImportError("prompt_toolkit is required for confirm prompt")
    
    default_choice = "y" if default else "n"
    default_text = "[Y/n]" if default else "[y/N]"
    
    # Calculate region indent for the prompt
    region_base_indent = _get_region_indent()
    
    # Track selected answer and answered state
    selected_answer = [None]
    is_answered = [False]
    
    # Function to get prompt tokens (question + answer when answered)
    def get_prompt_tokens():
        tokens = []
        # Qmark and question
        tokens.append(('class:qmark', f"{region_base_indent}{indent}? "))
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
    
    # Get merged style (defaults + global overrides + per-prompt overrides)
    # Add dim style for [Y/n] indicator
    style_dict = get_prompt_style(style)
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

