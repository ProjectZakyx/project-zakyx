# GitHub Default Branch Setzen
# Setzt 'project-zakyx-main' als Standard-Branch im GitHub Repository

param(
    [string]$GitHubToken = "",
    [string]$Owner = "ProjectZakyx",
    [string]$Repo = "project-zakyx",
    [string]$NewDefaultBranch = "project-zakyx-main"
)

Write-Host "GitHub Default Branch aendern" -ForegroundColor Cyan
Write-Host "=================================" -ForegroundColor Cyan
Write-Host ""

# Pruefe ob Token vorhanden ist
if ([string]::IsNullOrEmpty($GitHubToken)) {
    Write-Host "WARNUNG: GitHub Personal Access Token nicht gefunden!" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Option 1: GitHub Web-Oberflaeche verwenden" -ForegroundColor Green
    Write-Host "   1. Gehe zu: https://github.com/$Owner/$Repo/settings/branches" -ForegroundColor White
    Write-Host "   2. Scrolle zu 'Default branch'" -ForegroundColor White
    Write-Host "   3. Klicke auf 'Switch to another branch'" -ForegroundColor White
    Write-Host "   4. Waehle '$NewDefaultBranch'" -ForegroundColor White
    Write-Host "   5. Klicke 'Update' und bestaetige" -ForegroundColor White
    Write-Host ""
    Write-Host "Option 2: Mit Token ueber API (empfohlen)" -ForegroundColor Green
    Write-Host "   Fuehre aus: .\set_default_branch.ps1 -GitHubToken 'DEIN_TOKEN'" -ForegroundColor White
    Write-Host ""
    Write-Host "Token erstellen:" -ForegroundColor Cyan
    Write-Host "   https://github.com/settings/tokens" -ForegroundColor White
    Write-Host "   Benoetigte Berechtigung: 'repo' (Full control of private repositories)" -ForegroundColor White
    Write-Host ""
    
    # Oeffne Browser mit Settings-Seite
    $settingsUrl = "https://github.com/$Owner/$Repo/settings/branches"
    Write-Host "Oeffne GitHub Settings-Seite..." -ForegroundColor Cyan
    Start-Process $settingsUrl
    
    exit 0
}

# API Request
$apiUrl = "https://api.github.com/repos/$Owner/$Repo"
$headers = @{
    "Authorization" = "token $GitHubToken"
    "Accept" = "application/vnd.github.v3+json"
}

Write-Host "Verbinde mit GitHub API..." -ForegroundColor Cyan

try {
    # Hole aktuelle Repository-Informationen
    $repoInfo = Invoke-RestMethod -Uri $apiUrl -Headers $headers -Method Get
    $currentDefault = $repoInfo.default_branch
    
    Write-Host "Aktueller Standard-Branch: $currentDefault" -ForegroundColor Green
    
    if ($currentDefault -eq $NewDefaultBranch) {
        Write-Host "'$NewDefaultBranch' ist bereits der Standard-Branch!" -ForegroundColor Green
        exit 0
    }
    
    # Setze neuen Default Branch
    Write-Host "Setze '$NewDefaultBranch' als Standard-Branch..." -ForegroundColor Yellow
    
    $body = @{
        default_branch = $NewDefaultBranch
    } | ConvertTo-Json
    
    $response = Invoke-RestMethod -Uri $apiUrl -Headers $headers -Method Patch -Body $body -ContentType "application/json"
    
    Write-Host ""
    Write-Host "Erfolgreich!" -ForegroundColor Green
    Write-Host "   Neuer Standard-Branch: $($response.default_branch)" -ForegroundColor White
    Write-Host ""
    Write-Host "Repository: https://github.com/$Owner/$Repo" -ForegroundColor Cyan
    
} catch {
    Write-Host ""
    Write-Host "Fehler beim Setzen des Default Branches!" -ForegroundColor Red
    Write-Host "   Fehler: $($_.Exception.Message)" -ForegroundColor Red
    
    if ($_.Exception.Response.StatusCode -eq 401) {
        Write-Host ""
        Write-Host "Authentifizierung fehlgeschlagen!" -ForegroundColor Yellow
        Write-Host "   Bitte ueberpruefe dein GitHub Token." -ForegroundColor Yellow
    } elseif ($_.Exception.Response.StatusCode -eq 403) {
        Write-Host ""
        Write-Host "Keine Berechtigung!" -ForegroundColor Yellow
        Write-Host "   Du benoetigst 'repo' Berechtigung fuer dieses Repository." -ForegroundColor Yellow
    } elseif ($_.Exception.Response.StatusCode -eq 404) {
        Write-Host ""
        Write-Host "Repository nicht gefunden!" -ForegroundColor Yellow
        Write-Host "   Bitte ueberpruefe Owner und Repository-Name." -ForegroundColor Yellow
    }
    
    exit 1
}

