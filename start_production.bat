@echo off
REM 🚀 ORA BROWSER - PRODUCTION STARTUP SCRIPT
REM Version: 1.0.0
REM Datum: 2025-06-21

echo.
echo ========================================
echo 🚀 ORA BROWSER - PRODUCTION START
echo ========================================
echo.

REM Prüfe ob Release-Build existiert
if not exist "target\release\ora-browser.exe" (
    echo ❌ FEHLER: Release-Build nicht gefunden!
    echo.
    echo Bitte zuerst erstellen mit:
    echo cargo build --release
    echo.
    pause
    exit /b 1
)

REM Erstelle Konfigurationsverzeichnis
if not exist "%APPDATA%\ora-browser" (
    echo 📁 Erstelle Konfigurationsverzeichnis...
    mkdir "%APPDATA%\ora-browser"
    mkdir "%APPDATA%\ora-browser\logs"
)

REM Setze Produktions-Umgebungsvariablen
set RUST_LOG=ora_browser=info
set ORA_BROWSER_ENV=production
set ORA_BROWSER_CONFIG=%APPDATA%\ora-browser\config.toml

echo ⚙️ Produktions-Konfiguration:
echo    📂 Config-Verzeichnis: %APPDATA%\ora-browser
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

echo ✅ Starte Ora Browser (Produktions-Modus)...
echo.
echo 📝 Logs werden gespeichert in: %APPDATA%\ora-browser\logs\
echo 🌐 Proxy-Server läuft auf: http://localhost:3030
echo 🔍 Health-Check: http://localhost:3030/health
echo.
echo Zum Beenden: Strg+C drücken
echo ========================================
echo.

REM Starte Ora Browser
"target\release\ora-browser.exe"

REM Nach dem Beenden
echo.
echo ========================================
echo 🛑 ORA BROWSER BEENDET
echo ========================================
echo.
echo 📊 Produktions-Statistiken:
if exist "%APPDATA%\ora-browser\logs\ora-browser.log" (
    echo    📄 Log-Datei: %APPDATA%\ora-browser\logs\ora-browser.log
    for %%A in ("%APPDATA%\ora-browser\logs\ora-browser.log") do echo    📏 Log-Größe: %%~zA Bytes
) else (
    echo    📄 Keine Logs gefunden
)
echo.
pause 