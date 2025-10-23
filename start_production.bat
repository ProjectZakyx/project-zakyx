@echo off
REM 🚀 ZAKYX BROWSER - PRODUCTION STARTUP SCRIPT
REM Version: 1.0.0
REM Datum: 2025-06-21

echo.
echo ========================================
echo 🚀 ZAKYX BROWSER - PRODUCTION START
echo ========================================
echo.

REM Prüfe ob Release-Build existiert
if not exist "target\release\zakyx-browser.exe" (
    echo ❌ FEHLER: Release-Build nicht gefunden!
    echo.
    echo Bitte zuerst erstellen mit:
    echo cargo build --release
    echo.
    pause
    exit /b 1
)

REM Erstelle Konfigurationsverzeichnis
if not exist "%APPDATA%\zakyx-browser" (
    echo 📁 Erstelle Konfigurationsverzeichnis...
    mkdir "%APPDATA%\zakyx-browser"
    mkdir "%APPDATA%\zakyx-browser\logs"
)

REM Setze Produktions-Umgebungsvariablen
set RUST_LOG=zakyx_browser=info
set ZAKYX_BROWSER_ENV=production
set ZAKYX_BROWSER_CONFIG=%APPDATA%\zakyx-browser\config.toml

echo ⚙️ Produktions-Konfiguration:
echo    📂 Config-Verzeichnis: %APPDATA%\zakyx-browser
echo    📄 Log-Level: INFO
echo    🌐 Proxy-Port: 3030
echo.

REM Prüfe ob Port 3030 frei ist
netstat -an | findstr :3030 >nul
if %errorlevel% == 0 (
    echo ⚠️ WARNUNG: Port 3030 ist bereits belegt!
    echo.
    echo Möchten Sie trotzdem fortfahren? (J/N)
    set /p choice=
    if /i not "%choice%"=="J" (
        echo Abbruch.
        pause
        exit /b 1
    )
)

echo ✅ Starte ZAKYX Browser (Produktions-Modus)...
echo.
echo 📝 Logs werden gespeichert in: %APPDATA%\zakyx-browser\logs\
echo 🌐 Proxy-Server läuft auf: http://localhost:3030
echo 🔍 Health-Check: http://localhost:3030/health
echo.
echo Zum Beenden: Strg+C drücken
echo ========================================
echo.

REM Starte ZAKYX Browser
"target\release\zakyx-browser.exe"

REM Nach dem Beenden
echo.
echo ========================================
echo 🛑 ZAKYX BROWSER BEENDET
echo ========================================
echo.
echo 📊 Produktions-Statistiken:
if exist "%APPDATA%\zakyx-browser\logs\zakyx-browser.log" (
    echo    📄 Log-Datei: %APPDATA%\zakyx-browser\logs\zakyx-browser.log
    for %%A in ("%APPDATA%\zakyx-browser\logs\zakyx-browser.log") do echo    📏 Log-Größe: %%~zA Bytes
) else (
    echo    📄 Keine Logs gefunden
)
echo.
pause 