"""
pyprompt - Beautiful interactive prompts using prompt_toolkit.

A standalone package for creating beautiful, user-friendly command-line prompts.
"""

from .prompts import (
    text,
    select,
    confirm,
    HAS_PROMPT_TOOLKIT,
    set_region_indent_func,
    set_global_style,
    register_style,
    get_named_style,
)

__all__ = [
    'text',
    'select',
    'confirm',
    'HAS_PROMPT_TOOLKIT',
    'set_region_indent_func',
    'set_global_style',
    'register_style',
    'get_named_style',
]

__version__ = '1.0.0'

