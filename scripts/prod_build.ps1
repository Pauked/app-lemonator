$buildCmd = "cargo build --release"

# Build the Rust project
Invoke-Expression $buildCmd

# Extract version number and name from Cargo.toml
$cargoTomlContent = Get-Content -Path "Cargo.toml"
$versionLine = $cargoTomlContent | Where-Object { $_ -like "version =*" }
$nameLine = $cargoTomlContent | Where-Object { $_ -like "name =*" }

$version = ($versionLine -split '=')[1].Trim().Trim('"')
$appName = ($nameLine -split '=')[1].Trim().Trim('"')

# Output file name
$compressedFileName = "$appName-v$version"

# Where the build will be
$releaseDir = "target/release"

# Determine OS and compress accordingly
if ($env:IsWindows) {
    # Check if the zip file exists, and if so, delete it
    $zipPath = "$releaseDir/$compressedFileName.zip"
    if (Test-Path $zipPath) {
        Remove-Item $zipPath
    }

    # Use Windows compression for .zip
    Compress-Archive -Path "$releaseDir/$appName.exe" -DestinationPath $zipPath
} elseif ($env:IsMacOS) {
    # Check if the dmg file exists, and if so, delete it
    $dmgPath = "$releaseDir/$compressedFileName.dmg"
    if (Test-Path $dmgPath) {
        Remove-Item $dmgPath
    }

    # Build a .dmg for MacOS
    $dmgCmd = "hdiutil create $dmgPath -volname $appName -srcfolder $releaseDir/$appName"
    Invoke-Expression $dmgCmd
} else {
    Write-Output "Unsupported OS"
}
