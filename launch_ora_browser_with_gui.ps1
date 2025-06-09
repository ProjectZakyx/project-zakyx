# 🚀 Ora Browser Launcher mit Premium GUI
# Startet den Browser und öffnet die schöne HTML-Oberfläche

Write-Host "🚀 ===============================================" -ForegroundColor Cyan
Write-Host "🌟           ORA BROWSER LAUNCHER              🌟" -ForegroundColor Yellow
Write-Host "🚀 ===============================================" -ForegroundColor Cyan
Write-Host ""

# Prüfe ob Browser-Binary existiert
$BrowserPath = ".\target\release\ora-browser.exe"
$HTMLPath = ".\ora_webview2_premium_beautiful.html"

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
    Write-Host "💡 Bitte führe zuerst den Browser einmal aus um die Datei zu erstellen." -ForegroundColor Yellow
    exit 1
}

Write-Host "✅ Browser Binary gefunden: $BrowserPath" -ForegroundColor Green
Write-Host "✅ Premium HTML gefunden: $HTMLPath" -ForegroundColor Green
Write-Host ""

# Starte Browser im Hintergrund
Write-Host "🚀 Starte Ora Browser..." -ForegroundColor Cyan
$BrowserProcess = Start-Process -FilePath $BrowserPath -PassThru
Write-Host "✅ Browser gestartet (PID: $($BrowserProcess.Id))" -ForegroundColor Green

# Warte kurz damit Browser initialisiert
Write-Host "⏳ Warte auf Browser-Initialisierung..." -ForegroundColor Yellow
Start-Sleep -Seconds 2

# Öffne Premium HTML-GUI im Standard-Browser (parallel zum Ora Browser)
Write-Host "🎨 Öffne Premium GUI im Browser..." -ForegroundColor Cyan
$FullPath = (Get-Item $HTMLPath).FullName
$FileURL = "file:///$($FullPath.Replace('\', '/'))"

Write-Host "🌐 HTML-URL: $FileURL" -ForegroundColor Magenta

# Öffne im Standard-Browser
Start-Process $FileURL

Write-Host ""
Write-Host "🎉 =======================================" -ForegroundColor Green
Write-Host "✅        BROWSER ERFOLGREICH GESTARTET!" -ForegroundColor Green  
Write-Host "🎉 =======================================" -ForegroundColor Green
Write-Host ""
Write-Host "🔥 Jetzt läuft:" -ForegroundColor Yellow
Write-Host "   💻 Ora Browser Backend (PID: $($BrowserProcess.Id))" -ForegroundColor Cyan
Write-Host "   🎨 Premium HTML-GUI im Browser" -ForegroundColor Cyan
Write-Host ""
Write-Host "📋 Browser-Kontrolle:" -ForegroundColor Yellow
Write-Host "   📊 Prozess anzeigen: Get-Process ora-browser" -ForegroundColor Gray
Write-Host "   🛑 Browser beenden: Stop-Process -Name 'ora-browser'" -ForegroundColor Gray
Write-Host ""
Write-Host "🌟 Genieße die wunderschöne neue Oberfläche! 🌟" -ForegroundColor Magenta

# Warte auf Benutzereingabe bevor Script beendet wird
Write-Host ""
Write-Host "💡 Drücke eine beliebige Taste um dieses Fenster zu schließen..." -ForegroundColor DarkGray
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown") 