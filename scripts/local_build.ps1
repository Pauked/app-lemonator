# Run a Release build
& cargo build --release

if ($LASTEXITCODE -eq 0) {
    # Init folders from environment variables
    $sourceApp = Join-Path -Path $env:LocalDev "app-lemonator/target/release"
    $sourceApp = Join-Path -Path $sourceApp -ChildPath $env:ExeAppLemonator
    $targetPath = $env:LocalAppLemonator

    # Copy to local folder
    Copy-Item $sourceApp -Destination $targetPath

    Write-Host "Build and Deploy Successful!"
}
else {
    Write-Host "Build failed! Could not Deploy."
}
