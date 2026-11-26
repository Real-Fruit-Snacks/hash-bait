# install.ps1 - installs hash-bait for the current Windows account. No admin needed.
#
#   irm https://raw.githubusercontent.com/Real-Fruit-Snacks/hash-bait/main/install.ps1 | iex
#
# Safe to re-run: it just downloads the latest release over the old one.
# ASCII only and no 'exit': under 'irm | iex' an exit would close your window.
$ErrorActionPreference = 'Stop'

$repo  = 'Real-Fruit-Snacks/hash-bait'
$asset = 'hash-bait-x86_64-pc-windows-msvc.zip'
$dir   = Join-Path $env:LOCALAPPDATA 'Programs\hash-bait'

New-Item -ItemType Directory -Force $dir | Out-Null

$zip = Join-Path $env:TEMP 'hash-bait.zip'
Write-Host "Downloading latest hash-bait release..."
Invoke-WebRequest "https://github.com/$repo/releases/latest/download/$asset" -OutFile $zip
Expand-Archive $zip -DestinationPath $dir -Force
Remove-Item $zip

$userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
if (($userPath -split ';') -notcontains $dir) {
    [Environment]::SetEnvironmentVariable('Path', "$userPath;$dir", 'User')
    Write-Host "Added $dir to your PATH - open a new terminal to pick it up."
}

& (Join-Path $dir 'hash-bait.exe') --version
Write-Host "hash-bait installed to $dir"
