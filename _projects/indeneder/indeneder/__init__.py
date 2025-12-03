"""
indeneder - Terminal output formatting with automatic indentation.

A standalone package for creating headers and automatically indenting content.
"""

from .terminal import (
    write_header,
    write_header_fat,
    write_boxed_header,
    start_region,
    end_region,
    get_region_indent,
    COLOR_GREEN,
    COLOR_YELLOW,
    COLOR_RED,
    COLOR_RESET,
    COLOR_DIM,
    BOLD_CHECK,
)

__all__ = [
    'write_header',
    'write_header_fat',
    'write_boxed_header',
    'start_region',
    'end_region',
    'get_region_indent',
    'COLOR_GREEN',
    'COLOR_YELLOW',
    'COLOR_RED',
    'COLOR_RESET',
    'COLOR_DIM',
    'BOLD_CHECK',
]

__version__ = '1.0.0'

