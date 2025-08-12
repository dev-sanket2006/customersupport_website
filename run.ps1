# Kill the running major1.exe if it's already running
$process = Get-Process "major1" -ErrorAction SilentlyContinue
if ($process) {
    Write-Host "Stopping running server..."
    Stop-Process -Name "major1" -Force
}

# Now run the server
Write-Host "Starting the server..."
cargo run
