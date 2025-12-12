"""Configuration dataclass with validation."""

from dataclasses import dataclass
from pathlib import Path
from typing import Optional
from ..exceptions import PmakeConfigError
from .paths import find_project_root


@dataclass
class PmakeConfig:
    """Configuration for py-makefile build system."""
    
    project_root: Path
    sketch_dir: Path
    arduino_cli_path: Path
    fqbn: str
    sketch_name: str
    port: str = "COM9"
    baudrate: int = 115200
    verbose: bool = False
    create_log: bool = False
    
    # Derived paths (can be overridden but usually computed)
    _build_path: Optional[Path] = None
    _library_path: Optional[Path] = None
    _cache_path: Optional[Path] = None
    
    def __post_init__(self) -> None:
        """Validate configuration after initialization."""
        self.project_root = Path(self.project_root).resolve()
        self.sketch_dir = Path(self.sketch_dir).resolve()
        self.arduino_cli_path = Path(self.arduino_cli_path).resolve()
        
        # Validate required paths
        if not self.sketch_dir.exists():
            raise PmakeConfigError(
                f"Sketch directory does not exist: {self.sketch_dir}",
                field="sketch_dir"
            )
        
        if not self.arduino_cli_path.exists():
            raise PmakeConfigError(
                f"Arduino CLI not found at: {self.arduino_cli_path}",
                field="arduino_cli_path"
            )
        
        # Validate sketch file
        sketch_file = self.sketch_dir / self.sketch_name
        if not sketch_file.exists():
            raise PmakeConfigError(
                f"Sketch file not found: {sketch_file}",
                field="sketch_name"
            )
    
    @property
    def build_path(self) -> Path:
        """Get build output directory."""
        if self._build_path:
            return Path(self._build_path).resolve()
        return self.sketch_dir / "build"
    
    @build_path.setter
    def build_path(self, value: Path) -> None:
        """Set build output directory."""
        self._build_path = Path(value)
    
    @property
    def library_path(self) -> Path:
        """Get library directory."""
        if self._library_path:
            return Path(self._library_path).resolve()
        # Default library path relative to project root
        return self.project_root / "lib" / "esp32-s3"
    
    @library_path.setter
    def library_path(self, value: Path) -> None:
        """Set library directory."""
        self._library_path = Path(value)
    
    @property
    def cache_path(self) -> Path:
        """Get cache directory for build artifacts."""
        if self._cache_path:
            return Path(self._cache_path).resolve()
        return self.project_root / ".pmake2_cache"
    
    @cache_path.setter
    def cache_path(self, value: Path) -> None:
        """Set cache directory."""
        self._cache_path = Path(value)
    
    @classmethod
    def from_script_path(
        cls,
        script_path: Path,
        arduino_cli_path: Path,
        fqbn: str,
        sketch_name: str,
        port: str = "COM9",
        baudrate: int = 115200,
        **kwargs
    ) -> "PmakeConfig":
        """
        Create config from script path with automatic project root detection.
        
        Args:
            script_path: Path to the project-specific wrapper script
            arduino_cli_path: Path to arduino-cli executable
            fqbn: Fully Qualified Board Name
            sketch_name: Name of the sketch file
            port: Serial port
            baudrate: Serial baudrate
            **kwargs: Additional configuration options
            
        Returns:
            Configured PmakeConfig instance
        """
        script_path = Path(script_path).resolve()
        sketch_dir = script_path.parent
        project_root = find_project_root(sketch_dir)
        
        return cls(
            project_root=project_root,
            sketch_dir=sketch_dir,
            arduino_cli_path=arduino_cli_path,
            fqbn=fqbn,
            sketch_name=sketch_name,
            port=port,
            baudrate=baudrate,
            **kwargs
        )

