# scripts/install_native_host.ps1
# SynMem Native Host Installer for Windows
#
# This script registers the native messaging host with Chrome/Edge/Firefox
#
# Usage:
#   .\install_native_host.ps1 -ExtensionId "your-extension-id"
#   .\install_native_host.ps1 -ExtensionId "your-extension-id" -Uninstall
#   .\install_native_host.ps1 -ExtensionId "your-extension-id" -Firefox

param(
    [Parameter(Mandatory=$true)]
    [string]$ExtensionId,
    
    [switch]$Uninstall,
    
    [switch]$Firefox,
    
    [string]$HostPath = $null
)

$ErrorActionPreference = "Stop"

# Configuration
$HostName = "com.synmem.nativehost"
$HostDescription = "SynMem Native Messaging Host"

# Determine the host executable path
if (-not $HostPath) {
    # Default to the release build location
    $ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
    $RepoRoot = Split-Path -Parent $ScriptDir
    $HostPath = Join-Path $RepoRoot "target\release\synmem-native-host.exe"
    
    # Fall back to debug build if release doesn't exist
    if (-not (Test-Path $HostPath)) {
        $HostPath = Join-Path $RepoRoot "target\debug\synmem-native-host.exe"
    }
}

# Validate host executable exists (only when installing)
if (-not $Uninstall) {
    if (-not (Test-Path $HostPath)) {
        Write-Error "Native host executable not found at: $HostPath"
        Write-Error "Please build the project first with: cargo build --release -p synmem-native-host"
        exit 1
    }

    # Get absolute path
    $HostPath = (Resolve-Path $HostPath).Path
}

Write-Host "🧠 SynMem Native Host Installer" -ForegroundColor Cyan
Write-Host "================================" -ForegroundColor Cyan
Write-Host ""

if ($Uninstall) {
    Write-Host "Uninstalling native host..." -ForegroundColor Yellow
} else {
    Write-Host "Host Name:      $HostName" -ForegroundColor Green
    Write-Host "Host Path:      $HostPath" -ForegroundColor Green
    Write-Host "Extension ID:   $ExtensionId" -ForegroundColor Green
    Write-Host ""
}

# Function to get manifest directory
function Get-ManifestDir {
    param([string]$Browser)
    
    switch ($Browser) {
        "Chrome" {
            return "$env:LOCALAPPDATA\Google\Chrome\NativeMessagingHosts"
        }
        "Edge" {
            return "$env:LOCALAPPDATA\Microsoft\Edge\NativeMessagingHosts"
        }
        "Firefox" {
            return "$env:APPDATA\Mozilla\NativeMessagingHosts"
        }
    }
}

# Function to create manifest
function New-NativeHostManifest {
    param(
        [string]$Browser
    )
    
    if ($Firefox) {
        # Firefox uses a different format
        $manifest = @{
            name = $HostName
            description = $HostDescription
            path = $HostPath
            type = "stdio"
            allowed_extensions = @("$ExtensionId@synmem.com")
        }
    } else {
        # Chrome/Edge format
        $manifest = @{
            name = $HostName
            description = $HostDescription
            path = $HostPath
            type = "stdio"
            allowed_origins = @("chrome-extension://$ExtensionId/")
        }
    }
    
    return $manifest | ConvertTo-Json -Depth 10
}

# Function to install for a browser
function Install-NativeHost {
    param([string]$Browser)
    
    $ManifestDir = Get-ManifestDir -Browser $Browser
    $ManifestPath = Join-Path $ManifestDir "$HostName.json"
    
    Write-Host "Installing for $Browser..." -ForegroundColor Cyan
    
    if ($Uninstall) {
        if (Test-Path $ManifestPath) {
            Remove-Item $ManifestPath -Force
            Write-Host "  ✓ Removed manifest: $ManifestPath" -ForegroundColor Green
        } else {
            Write-Host "  ~ Manifest not found, skipping" -ForegroundColor Yellow
        }
    } else {
        # Create directory if it doesn't exist
        if (-not (Test-Path $ManifestDir)) {
            New-Item -ItemType Directory -Path $ManifestDir -Force | Out-Null
            Write-Host "  ✓ Created directory: $ManifestDir" -ForegroundColor Green
        }
        
        # Write manifest
        $ManifestContent = New-NativeHostManifest -Browser $Browser
        $ManifestContent | Out-File -FilePath $ManifestPath -Encoding UTF8 -Force
        Write-Host "  ✓ Created manifest: $ManifestPath" -ForegroundColor Green
    }
}

# Install for Chrome and Edge (unless Firefox-only)
if (-not $Firefox) {
    Install-NativeHost -Browser "Chrome"
    Install-NativeHost -Browser "Edge"
}

# Install for Firefox if requested
if ($Firefox) {
    Install-NativeHost -Browser "Firefox"
}

Write-Host ""
if ($Uninstall) {
    Write-Host "✅ Native host uninstalled successfully!" -ForegroundColor Green
} else {
    Write-Host "✅ Native host installed successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Cyan
    Write-Host "  1. Restart your browser"
    Write-Host "  2. The extension should now be able to connect to the native host"
}
