@echo off
REM TaskNag Development Environment Manager
REM Usage: dev.bat [start|stop|restart|status]

if "%1"=="" (
    echo Usage: dev.bat [start^|stop^|restart^|status]
    echo.
    echo Commands:
    echo   start   - Start the development environment
    echo   stop    - Stop the development environment and cleanup processes
    echo   restart - Restart the development environment
    echo   status  - Show current development environment status
    exit /b 1
)

REM Get the directory where this script is located
set SCRIPT_DIR=%~dp0

REM Execute the PowerShell script
powershell -ExecutionPolicy Bypass -File "%SCRIPT_DIR%dev-process-manager.ps1" -Action %1

exit /b %ERRORLEVEL%