"""
Py-Makefile - Optimized Python-based build system for Arduino/ESP32

BREAKING CHANGES (v2.0.0):
- Complete rewrite with optimized architecture
- Domain-based module structure (config, build, upload, core, cache, ui)
- ProgressMonitor split into separate parser, state, and progress modules
- Build artifact caching added
- Compiled regex patterns for better performance
- Improved path handling without fragile parent chains

MIGRATION GUIDE:
- Import from py_makefile instead of pmake
- Configuration API remains compatible
- Progress monitoring uses new abstraction layer
"""

from .config import PmakeConfig
from .cli import run
from .exceptions import (
    PmakeError,
    PmakeConfigError,
    PmakeBuildError,
    PmakeUploadError,
    PmakeCacheError,
)

__all__ = [
    'PmakeConfig',
    'run',
    'PmakeError',
    'PmakeConfigError',
    'PmakeBuildError',
    'PmakeUploadError',
    'PmakeCacheError',
]
