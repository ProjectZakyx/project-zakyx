# 🚀 ZAKYX Browser Launcher mit Premium GUI
# Startet den Browser und öffnet die schöne HTML-Oberfläche

Write-Host "===============================================" -ForegroundColor Cyan
Write-Host "ZAKYX BROWSER LAUNCHER" -ForegroundColor Yellow
Write-Host "===============================================" -ForegroundColor Cyan
Write-Host ""

# Prüfe ob Browser-Binary existiert
$BrowserPath = ".\target\release\zakyx-browser.exe"
$HTMLPath = ".\zakyx_webview2_premium_beautiful.html"

if (-not (Test-Path $BrowserPath)) {
    Write-Host "❌ Browser nicht gefunden! Building..." -ForegroundColor Red
    cargo build --release
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Build fehlgeschlagen!" -ForegroundColor Red
        exit 1
    }
}

# Prüfe ob HTML-Datei existiert
if (-not (Test-Path $HTMLPath)) {
    Write-Host "❌ Premium HTML-Datei nicht gefunden: $HTMLPath" -ForegroundColor Red
    Write-Host "💡 Verwende alternative HTML-Datei..." -ForegroundColor Yellow
    
    # Versuche alternative HTML-Dateien
    $AlternativePaths = @(
        ".\dist\index.html",
        ".\release\zakyx-browser-v1.0.0\gui.html",
        ".\dist\app.html"
    )
    
    $FoundPath = $null
    foreach ($path in $AlternativePaths) {
        if (Test-Path $path) {
            $FoundPath = $path
            break
        }
    }
    
    if ($FoundPath) {
        $HTMLPath = $FoundPath
    Write-Host "Alternative HTML-Datei gefunden: $HTMLPath" -ForegroundColor Green
    } else {
        Write-Host "❌ Keine HTML-Datei gefunden!" -ForegroundColor Red
        Write-Host "💡 Bitte führe zuerst den Browser einmal aus um die Datei zu erstellen." -ForegroundColor Yellow
        exit 1
    }
}

Write-Host "Browser Binary gefunden: $BrowserPath" -ForegroundColor Green
Write-Host "Premium HTML gefunden: $HTMLPath" -ForegroundColor Green
Write-Host ""

# Starte Browser im Hintergrund
Write-Host "Starte ZAKYX Browser..." -ForegroundColor Cyan
$BrowserProcess = Start-Process -FilePath $BrowserPath -PassThru
Write-Host "Browser gestartet (PID: $($BrowserProcess.Id))" -ForegroundColor Green

# Warte kurz damit Browser initialisiert
Write-Host "Warte auf Browser-Initialisierung..." -ForegroundColor Yellow
Start-Sleep -Seconds 2

# Öffne Premium HTML-GUI im Standard-Browser (parallel zum ZAKYX Browser)
Write-Host "Oeffne Premium GUI im Browser..." -ForegroundColor Cyan
$FullPath = (Get-Item $HTMLPath).FullName
$FileURL = "file:///$($FullPath.Replace('\', '/'))"

Write-Host "HTML-URL: $FileURL" -ForegroundColor Magenta

# Öffne im Standard-Browser
Start-Process $FileURL

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "BROWSER ERFOLGREICH GESTARTET!" -ForegroundColor Green  
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "Jetzt laeuft:" -ForegroundColor Yellow
Write-Host "   ZAKYX Browser Backend (PID: $($BrowserProcess.Id))" -ForegroundColor Cyan
Write-Host "   Premium HTML-GUI im Browser" -ForegroundColor Cyan
Write-Host ""
Write-Host "Browser-Kontrolle:" -ForegroundColor Yellow
Write-Host "   Prozess anzeigen: Get-Process zakyx-browser" -ForegroundColor Gray
Write-Host "   Browser beenden: Stop-Process -Name 'zakyx-browser'" -ForegroundColor Gray
Write-Host ""
Write-Host "Viel Spass mit der neuen Oberflaeche!" -ForegroundColor Magenta

# Warte auf Benutzereingabe bevor Script beendet wird
Write-Host ""
Write-Host "Druecke eine Taste, um dieses Fenster zu schliessen..." -ForegroundColor DarkGray
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown") 