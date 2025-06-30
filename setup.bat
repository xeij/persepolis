@echo off
echo =================================
echo   Void Setup Script
echo =================================
echo.

echo Checking if Rust is installed...
where cargo >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    echo ✓ Rust/Cargo is already installed
    goto :build
) else (
    echo ✗ Rust/Cargo not found in current PATH
    echo.
    echo Trying to load Rust environment...
    if exist "%USERPROFILE%\.cargo\env.bat" (
        call "%USERPROFILE%\.cargo\env.bat"
        where cargo >nul 2>nul
        if %ERRORLEVEL% EQU 0 (
            echo ✓ Rust environment loaded successfully
            goto :build
        )
    )
    echo.
    echo Would you like to install Rust? (Y/N)
    set /p install_rust=
    if /i "%install_rust%" == "Y" (
        goto :install_rust
    ) else (
        echo Please install Rust manually from https://rustup.rs/
        pause
        exit /b 1
    )
)

:install_rust
echo.
echo Installing Rust...
echo This will download and run the rustup installer.
echo Press any key to continue...
pause >nul

:: Download and run rustup
powershell -Command "Invoke-WebRequest -Uri 'https://win.rustup.rs' -OutFile 'rustup-init.exe'"
rustup-init.exe
del rustup-init.exe

echo.
echo ======================================================================
echo  IMPORTANT: Rust installation complete!
echo ======================================================================
echo  
echo  You need to RESTART your command prompt/PowerShell window for Rust
echo  to be available in your PATH.
echo  
echo  Steps to continue:
echo  1. Close this window
echo  2. Open a new Command Prompt or PowerShell window
echo  3. Navigate back to this folder
echo  4. Run setup.bat again
echo  
echo  OR alternatively, add Rust to your current session by running:
echo     call "%%USERPROFILE%%\.cargo\env.bat"
echo  
echo ======================================================================
pause
exit /b 0

:build
echo.
echo Building Void...
cargo build
if %ERRORLEVEL% NEQ 0 (
    echo ⚠ Debug build failed, trying release mode...
    echo.
    cargo build --release
    if %ERRORLEVEL% NEQ 0 (
        echo ✗ Build failed in both debug and release modes. Please fix the errors above.
        pause
        exit /b 1
    ) else (
        echo ✓ Release build succeeded!
        goto :release_success
    )
) else (
    echo ✓ Debug build succeeded!
)

echo.
goto :menu

:release_success
echo.
echo Note: Only release mode is available (debug mode failed to link)
echo.
echo Choose an option:
echo 1. Run release mode
echo 2. Exit
echo.
set /p choice=Enter your choice (1-2): 

if "%choice%" == "1" goto :release_run_built
if "%choice%" == "2" goto :end
goto :end

:menu
echo Choose an option:
echo 1. Run in development mode (with inspector)
echo 2. Build and run release mode
echo 3. Just build (don't run)
echo 4. Exit
echo.
set /p choice=Enter your choice (1-4): 

if "%choice%" == "1" goto :dev_run
if "%choice%" == "2" goto :release_run
if "%choice%" == "3" goto :build_only
if "%choice%" == "4" goto :end

:dev_run
echo.
echo Starting in development mode...
echo Press F12 in-game to toggle inspector
echo Controls: WASD to move, Mouse to look, Left Click to shoot
cargo run
goto :end

:release_run
echo.
echo Building release version...
cargo build --release
echo.
echo Running release version...
target\release\void.exe
goto :end

:release_run_built
echo.
echo Running release version...
target\release\void.exe
goto :end

:build_only
echo.
echo Building only...
cargo build
echo ✓ Build complete!
goto :end

:end
echo.
echo Setup complete! Thanks for playing Void!
pause 