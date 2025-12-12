"""Custom exception classes for py-makefile."""

from typing import Optional


class PmakeError(Exception):
    """Base exception for all py-makefile errors."""
    pass


class PmakeConfigError(PmakeError):
    """Configuration-related errors."""
    
    def __init__(self, message: str, field: Optional[str] = None):
        super().__init__(message)
        self.field = field


class PmakeBuildError(PmakeError):
    """Build/compilation-related errors."""
    
    def __init__(self, message: str, returncode: Optional[int] = None):
        super().__init__(message)
        self.returncode = returncode


class PmakeUploadError(PmakeError):
    """Upload-related errors."""
    
    def __init__(self, message: str, returncode: Optional[int] = None):
        super().__init__(message)
        self.returncode = returncode


class PmakeCacheError(PmakeError):
    """Cache-related errors."""
    pass

