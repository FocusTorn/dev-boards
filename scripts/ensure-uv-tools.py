# #!/usr/bin/env python3
# """
# Script to ensure required UV tools are installed.

# This script reads the [tool.uv.tools] section from pyproject.toml
# and ensures all listed tools are installed via `uv tool install`.

# Usage:
#     python scripts/ensure-uv-tools.py
#     # or from workspace root:
#     uv run python scripts/ensure-uv-tools.py
# """

# import sys
# import os
# import subprocess
# from pathlib import Path

# # Try to import tomllib (Python 3.11+) or tomli (fallback)
# try:
#     import tomllib
# except ImportError:
#     try:
#         import tomli as tomllib
#     except ImportError:
#         print("ERROR: No TOML parser available. Install tomli: uv add --group dev tomli", file=sys.stderr)
#         sys.exit(1)

# # Get workspace root from environment variable or detect from current directory
# workspace_root = os.environ.get('WORKSPACE_ROOT')
# if workspace_root:
#     workspace_root = Path(workspace_root).resolve()
# else:
#     # Try to detect workspace root by looking for pyproject.toml
#     current = Path(__file__).resolve().parent.parent
#     if (current / "pyproject.toml").exists():
#         workspace_root = current
#     else:
#         print("ERROR: WORKSPACE_ROOT environment variable not set and could not detect workspace root.", file=sys.stderr)
#         print()
#         print("  Please add WORKSPACE_ROOT to your workspace settings file:")
#         print("    .vscode/settings.json (VS Code)")
#         print("    .cursor/settings.json (Cursor)")
#         print()
#         print("  Add the following setting:")
#         print('    "WORKSPACE_ROOT": "D:\\\\_dev\\\\_Projects\\\\dev-boards"')
#         print()
#         print("  Or set it as an environment variable in your shell.")
#         sys.exit(1)

# # Import terminal output utilities from outerm
# _script_dir = Path(__file__).resolve().parent.parent
# _shared_python = _script_dir / "_shared-resources" / "shared-python"
# if str(_shared_python) not in sys.path:
#     sys.path.insert(0, str(_shared_python))

# try:
#     from outerm.terminal import Palette, success, error, info
# except ImportError:
#     # Fallback if outerm is not available
#     class Palette:
#         SUCCESS = {'ansi': '\x1B[38;5;46m', 'iChar': '✔'}
#         ERROR = {'ansi': '\x1B[38;5;196m', 'iChar': '✗'}
#         INFO = {'ansi': '\x1B[38;5;39m', 'iChar': 'ｉ'}
#         RESET = '\x1B[0m'
    
#     def success(msg, icon=''):
#         return f"{Palette.SUCCESS['ansi']}{Palette.SUCCESS['iChar']}{Palette.RESET} {msg}"
    
#     def error(msg, icon=''):
#         return f"{Palette.ERROR['ansi']}{Palette.ERROR['iChar']}{Palette.RESET} {msg}"
    
#     def info(msg, icon=''):
#         return f"{Palette.INFO['ansi']}{Palette.INFO['iChar']}{Palette.RESET} {msg}"

# # Get color and icon constants from Palette
# SUCCESS_COLOR = Palette.SUCCESS.get('ansi', '\x1B[38;5;46m')
# ERROR_COLOR = Palette.ERROR.get('ansi', '\x1B[38;5;196m')
# INFO_COLOR = Palette.INFO.get('ansi', '\x1B[38;5;39m')
# RESET = Palette.RESET
# ICON_SUCCESS = Palette.SUCCESS.get('iChar', '✔')
# ICON_ERROR = Palette.ERROR.get('iChar', '✗')
# ICON_INFO = Palette.INFO.get('iChar', 'ｉ')


# def read_required_tools():
#     """Read required tools from pyproject.toml [tool.uv.tools] section."""
#     pyproject_path = workspace_root / "pyproject.toml"
    
#     if not pyproject_path.exists():
#         return []
    
#     try:
#         with open(pyproject_path, 'rb') as f:
#             data = tomllib.load(f)
        
#         # Check for [tool.uv.tools] section
#         tool_uv = data.get("tool", {}).get("uv", {})
#         required_tools = tool_uv.get("tools", [])
        
#         # Support both list format and dict format
#         if isinstance(required_tools, list):
#             return required_tools
#         elif isinstance(required_tools, dict):
#             # If it's a dict, return the keys (tool names)
#             return list(required_tools.keys())
#         else:
#             return []
#     except Exception as e:
#         print(f"{ERROR_COLOR}{ICON_ERROR}{RESET} Failed to read pyproject.toml: {e}", file=sys.stderr)
#         return []


# def is_tool_installed(tool_name):
#     """Check if a tool is installed via `uv tool list`."""
#     try:
#         result = subprocess.run(
#             ["uv", "tool", "list"],
#             capture_output=True,
#             text=True,
#             errors="ignore"
#         )
        
#         if result.returncode == 0:
#             # Check if tool name appears in the output
#             lines = result.stdout.strip().split("\n")
#             for line in lines:
#                 line = line.strip()
#                 if not line or line.startswith("-"):
#                     continue
#                 # Extract tool name (first word before space)
#                 parts = line.split()
#                 if parts and parts[0].lower() == tool_name.lower():
#                     return True
#     except Exception:
#         pass
    
#     return False


# def install_tool(tool_name):
#     """Install a tool via `uv tool install`."""
#     try:
#         print(f"  {INFO_COLOR}{ICON_INFO}{RESET} Installing {tool_name}...")
#         result = subprocess.run(
#             ["uv", "tool", "install", tool_name],
#             capture_output=True,
#             text=True,
#             errors="ignore"
#         )
        
#         if result.returncode == 0:
#             print(f"  {SUCCESS_COLOR}{ICON_SUCCESS}{RESET} {tool_name} installed successfully")
#             return True
#         else:
#             print(f"  {ERROR_COLOR}{ICON_ERROR}{RESET} Failed to install {tool_name}: {result.stderr}", file=sys.stderr)
#             return False
#     except Exception as e:
#         print(f"  {ERROR_COLOR}{ICON_ERROR}{RESET} Failed to install {tool_name}: {e}", file=sys.stderr)
#         return False


# def main():
#     """Main entry point."""
#     required_tools = read_required_tools()
    
#     if not required_tools:
#         print(f"{INFO_COLOR}{ICON_INFO}{RESET} No required tools specified in [tool.uv.tools] section")
#         return 0
    
#     print(f"{INFO_COLOR}{ICON_INFO}{RESET} Checking required UV tools...")
#     print()
    
#     missing_tools = []
#     installed_tools = []
    
#     for tool_name in required_tools:
#         if is_tool_installed(tool_name):
#             print(f"  {SUCCESS_COLOR}{ICON_SUCCESS}{RESET} {tool_name} (already installed)")
#             installed_tools.append(tool_name)
#         else:
#             missing_tools.append(tool_name)
    
#     if not missing_tools:
#         print()
#         print(f"{SUCCESS_COLOR}{ICON_SUCCESS}{RESET} All required tools are installed")
#         return 0
    
#     print()
#     print(f"{INFO_COLOR}{ICON_INFO}{RESET} Installing {len(missing_tools)} missing tool(s)...")
#     print()
    
#     failed = []
#     for tool_name in missing_tools:
#         if not install_tool(tool_name):
#             failed.append(tool_name)
    
#     print()
#     if failed:
#         print(f"{ERROR_COLOR}{ICON_ERROR}{RESET} Failed to install: {', '.join(failed)}")
#         return 1
#     else:
#         print(f"{SUCCESS_COLOR}{ICON_SUCCESS}{RESET} All required tools installed successfully")
#         return 0


# if __name__ == '__main__':
#     sys.exit(main())

