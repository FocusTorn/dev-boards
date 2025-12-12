"""UI and interaction domain."""

from .interface import (  # type: ignore
    show_interactive_menu,
    print_help,
    action,  # type: ignore[attr-defined]
    success,  # type: ignore[attr-defined]
    error,  # type: ignore[attr-defined]
    warning,  # type: ignore[attr-defined]
    info,  # type: ignore[attr-defined]
    write_header,  # type: ignore[attr-defined]
    write_header_fat,  # type: ignore[attr-defined]
    write_boxed_header,  # type: ignore[attr-defined]
)

__all__ = [
    'show_interactive_menu',
    'print_help',
    'action',
    'success',
    'error',
    'warning',
    'info',
    'write_header',
    'write_header_fat',
    'write_boxed_header',
]

