$ErrorActionPreference = "Stop"

$Repo = "yofabr/pkll"
$Bin  = "pkll"

# Detect arch
$Arch = if ([Environment]::Is64BitOperatingSystem) {
  if ($env:PROCESSOR_ARCHITECTURE -eq "ARM64") { "aarch64" } else { "x86_64" }
} else {
  Write-Host "32-bit Windows is not supported."; exit 1
}

$Target   = "$Arch-pc-windows-msvc"
$Filename = "$Bin-$Target.zip"

# Get latest version
Write-Host ""
Write-Host "  pkll installer"
$Release  = Invoke-RestMethod "https://api.github.com/repos/$Repo/releases/latest"
$Version  = $Release.tag_name
Write-Host "  Version : $Version"
Write-Host "  Target  : $Target"
Write-Host ""

$Url    = "https://github.com/$Repo/releases/download/$Version/$Filename"
$TmpDir = Join-Path $env:TEMP "pkll-install"
New-Item -ItemType Directory -Force -Path $TmpDir | Out-Null

Write-Host "Downloading $Filename..."
Invoke-WebRequest -Uri $Url -OutFile "$TmpDir\$Filename" -UseBasicParsing
Expand-Archive -Path "$TmpDir\$Filename" -DestinationPath $TmpDir -Force

# Install to ~/.local/bin (no admin needed)
$InstallDir = "$env:USERPROFILE\.local\bin"
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Move-Item -Force "$TmpDir\$Bin.exe" "$InstallDir\$Bin.exe"

# Add to user PATH if missing
$CurrentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($CurrentPath -notlike "*$InstallDir*") {
  [Environment]::SetEnvironmentVariable("PATH", "$CurrentPath;$InstallDir", "User")
  Write-Host "  Added $InstallDir to PATH"
  Write-Host "  Restart your terminal for PATH to take effect."
}

# Cleanup
Remove-Item -Recurse -Force $TmpDir

Write-Host ""
Write-Host "  pkll $Version installed to $InstallDir\$Bin.exe"
Write-Host "  Run: pkll <port>"
Write-Host ""
