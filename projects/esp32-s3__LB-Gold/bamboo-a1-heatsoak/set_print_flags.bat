@echo off
REM Windows batch wrapper for set_print_flags.py
REM This allows OrcaSlicer to execute the Python script properly

REM Get the directory where this batch file is located
set SCRIPT_DIR=%~dp0

REM Execute Python script with all arguments
python "%SCRIPT_DIR%set_print_flags.py" %*

REM Exit with the same code as Python
exit /b %ERRORLEVEL%
