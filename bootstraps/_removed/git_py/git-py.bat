@echo off
REM Cross-platform launcher for git_py (Windows batch file)
REM This works alongside git-py (Python script) for Windows convenience

set SCRIPT_DIR=%~dp0
python "%SCRIPT_DIR%main.py" %*

