# 🚀 Ora Browser Release Script
# Automatisierte Binary-Erstellung und Packaging

param(
    [Parameter(Mandatory=$false)]
    [string]$Version = "1.0.0",
    
    [Parameter(Mandatory=$false)]
    [switch]$SkipTests = $false,
    
    [Parameter(Mandatory=$false)]
    [switch]$CreateInstaller = $false
)

Write-Host "🌐 Ora Browser Release Script v1.0" -ForegroundColor Cyan
Write-Host "=================================" -ForegroundColor Cyan

# 📋 Pre-Release Checks
Write-Host "📋 Performing pre-release checks..." -ForegroundColor Yellow

# Check Rust installation
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Rust/Cargo not found! Please install Rust first." -ForegroundColor Red
    exit 1
}

# Check WebView2 (optional warning)
$webview2Path = "$env:ProgramFiles\Microsoft\EdgeWebView\Application"
if (!(Test-Path $webview2Path)) {
    Write-Host "⚠️ WebView2 Runtime not detected. Users will need to install it." -ForegroundColor Yellow
}

Write-Host "✅ Pre-release checks passed!" -ForegroundColor Green

# 🧪 Run Tests (unless skipped)
if (!$SkipTests) {
    Write-Host "🧪 Running tests..." -ForegroundColor Yellow
    cargo test
    if ($LASTEXITCODE -ne 0) {
        Write-Host "❌ Tests failed! Please fix before release." -ForegroundColor Red
        exit 1
    }
    Write-Host "✅ Tests passed!" -ForegroundColor Green
}

# 🔧 Clean previous builds
Write-Host "🔧 Cleaning previous builds..." -ForegroundColor Yellow
cargo clean
Remove-Item -Path "release" -Recurse -Force -ErrorAction SilentlyContinue

# 📦 Build Release Binary
Write-Host "📦 Building release binary..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed!" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Release binary built successfully!" -ForegroundColor Green

# 📁 Create Release Directory Structure
Write-Host "📁 Creating release directory..." -ForegroundColor Yellow
New-Item -Path "release" -ItemType Directory -Force | Out-Null
New-Item -Path "release\ora-browser-v$Version" -ItemType Directory -Force | Out-Null

$releaseDir = "release\ora-browser-v$Version"

# 📋 Copy Release Files
Write-Host "📋 Copying release files..." -ForegroundColor Yellow

# Binary
Copy-Item "target\release\projekt-ora.exe" "$releaseDir\ora-browser.exe"

# Documentation
Copy-Item "README.md" "$releaseDir\"
Copy-Item "LICENSE" "$releaseDir\"

# GUI Assets
Copy-Item "gui.html" "$releaseDir\" -ErrorAction SilentlyContinue

# Resources (if exist)
if (Test-Path "resources") {
    Copy-Item "resources" "$releaseDir\" -Recurse
}

# 📝 Create Release Info
Write-Host "📝 Creating release info..." -ForegroundColor Yellow

$releaseInfo = @"
🌐 Ora Browser v$Version
=========================

🎯 Release Information:
- Version: $Version
- Build Date: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss UTC")
- Build Machine: $env:COMPUTERNAME
- Windows Version: $((Get-WmiObject Win32_OperatingSystem).Caption)

📁 Files included:
- ora-browser.exe    - Main application binary
- README.md         - Full documentation
- LICENSE           - MIT License text
- gui.html          - HTML GUI interface
- resources/        - Icons and assets (if available)

🛠️ System Requirements:
- Windows 10/11 (64-bit)
- Microsoft Edge WebView2 Runtime
- Minimum 512 MB RAM

📥 Installation:
1. Extract all files to a folder
2. Install WebView2 Runtime if needed:
   https://developer.microsoft.com/en-us/microsoft-edge/webview2/
3. Run ora-browser.exe

🚀 Features:
✅ WebView2 Integration for real web rendering
✅ Tab Management System with navigation
✅ Bookmark System with default bookmarks
✅ History Tracking with auto-save
✅ HTML GUI with Tailwind CSS
✅ Fallback modes for compatibility

💡 Usage:
- Enter URLs in address bar (e.g., 'google.de')
- Use commands: 'newtab', 'bookmarks', 'history', 'gui'
- Press Enter to navigate

🐛 Troubleshooting:
- Install WebView2 Runtime if pages don't load
- Add to antivirus whitelist if startup fails
- Check Windows Defender SmartScreen settings

📞 Support:
- Issues: https://github.com/user/ora-browser/issues
- Documentation: See README.md

© 2024 Ora Browser Project - MIT License
"@

$releaseInfo | Out-File "$releaseDir\RELEASE_INFO.txt" -Encoding UTF8

# 🗜️ Create ZIP Archive
Write-Host "🗜️ Creating ZIP archive..." -ForegroundColor Yellow

$zipPath = "release\ora-browser-v$Version.zip"
Compress-Archive -Path "$releaseDir\*" -DestinationPath $zipPath -Force

# 📊 Calculate File Sizes
$binarySize = [math]::Round((Get-Item "target\release\projekt-ora.exe").Length / 1MB, 2)
$zipSize = [math]::Round((Get-Item $zipPath).Length / 1MB, 2)

# 🎯 Release Summary
Write-Host ""
Write-Host "🎉 RELEASE COMPLETED SUCCESSFULLY!" -ForegroundColor Green
Write-Host "=================================" -ForegroundColor Green
Write-Host "📦 Version: $Version" -ForegroundColor White
Write-Host "📁 Location: $releaseDir" -ForegroundColor White
Write-Host "🗜️ Archive: $zipPath" -ForegroundColor White
Write-Host "📊 Binary Size: $binarySize MB" -ForegroundColor White
Write-Host "📊 Archive Size: $zipSize MB" -ForegroundColor White
Write-Host ""

# 📋 List Release Contents
Write-Host "📋 Release Contents:" -ForegroundColor Cyan
Get-ChildItem $releaseDir | ForEach-Object {
    $size = if ($_.PSIsContainer) { "DIR" } else { "$([math]::Round($_.Length / 1KB, 1)) KB" }
    Write-Host "   $($_.Name.PadRight(20)) $size" -ForegroundColor White
}

Write-Host ""
Write-Host "🚀 Ready for distribution!" -ForegroundColor Green
Write-Host "📤 Upload $zipPath to GitHub Releases" -ForegroundColor Yellow

# 🔧 Optional: Create Installer
if ($CreateInstaller) {
    Write-Host "🔧 Creating installer (requires NSIS)..." -ForegroundColor Yellow
    # NSIS installer creation would go here
    # This requires NSIS to be installed
    Write-Host "⚠️ Installer creation not implemented yet" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "✨ Release script completed! ✨" -ForegroundColor Magenta 