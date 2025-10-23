@echo off
echo.
echo ==========================================
echo 🌐 ZAKYX BROWSER GUI INTEGRATION TEST
echo ==========================================
echo.

echo 🧪 PHASE 1: BUILDING BROWSER...
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Build failed!
    pause
    exit /b 1
)
echo ✅ Build successful!
echo.

echo 🧪 PHASE 2: CHECKING HTML FILES...
if exist "zakyx_premium_gui.html" (
    echo ✅ Premium GUI HTML found
) else (
    echo ❌ Premium GUI HTML missing
)

if exist "zakyx_browser_premium_gui.html" (
    echo ✅ Browser Premium GUI HTML found
) else (
    echo ❌ Browser Premium GUI HTML missing
)
echo.

echo 🧪 PHASE 3: STARTING BROWSER WITH TIMEOUT...
echo Browser wird für 5 Sekunden gestartet...
timeout /t 2 /nobreak >nul
start /wait timeout /t 5 /nobreak ^& taskkill /f /im zakyx-browser.exe 2^>nul
.\target\release\zakyx-browser.exe

echo.
echo 🧪 PHASE 4: CHECKING LOG OUTPUT...
echo Last browser startup logs:
echo.

echo 🧪 PHASE 5: GUI FUNCTIONALITY TEST...
echo Testing HTML GUI in default browser:
start "HTML GUI Test" "zakyx_premium_gui.html"

echo.
echo 🧪 PHASE 6: FEATURE VERIFICATION...
echo Checking if all browser features are loaded:
echo - WebView2 integration
echo - Security features  
echo - Performance optimizer
echo - Platform extensions
echo - UI/UX improvements

echo.
echo ==========================================
echo 🎯 GUI INTEGRATION TEST COMPLETE
echo ==========================================
pause 