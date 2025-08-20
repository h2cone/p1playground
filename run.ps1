param(
    [Parameter(Mandatory = $true, Position = 0)]
    [ValidateSet("start", "stop", "restart")]
    [string]$Command
)

$GodotExecutable = "godot"
$ProjectPath = "./godot"
$Arguments = @(
    "--path", $ProjectPath
)

$ProcessInfoFile = "godot_process.txt"

# Alternative cargo build function that uses direct command execution
function Invoke-CargoBuild {
    param(
        [string]$ManifestPath = "./rust/Cargo.toml"
    )
    
    try {
        Write-Host "Running cargo build using direct command execution..."
        $currentDir = Get-Location
        
        # Change to the directory containing the manifest
        $rustDir = Split-Path -Path $ManifestPath -Parent
        if ($rustDir -and (Test-Path $rustDir)) {
            Set-Location $rustDir
            $result = Invoke-Expression "cargo build" 2>&1
            Set-Location $currentDir
        }
        else {
            $result = Invoke-Expression "cargo build --manifest-path `"$ManifestPath`"" 2>&1
        }
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "Cargo build completed successfully using direct execution."
            return $true
        }
        else {
            Write-Host "Cargo build failed with exit code: $LASTEXITCODE"
            Write-Host "Output: $result"
            return $false
        }
    }
    catch {
        Write-Host "Error during cargo build: $_"
        Set-Location $currentDir
        return $false
    }
}

function Start-Godot {
    Write-Host "Attempting to start Godot project: $ProjectPath"
    Write-Host "Using Godot executable: $GodotExecutable"
    Write-Host "Arguments: $($Arguments -join ' ')"
    
    try {
        $godotProcess = Start-Process -FilePath $GodotExecutable -ArgumentList $Arguments -PassThru -NoNewWindow
        
        if ($godotProcess) {
            $processId = $godotProcess.Id
            Write-Host "Godot process started successfully."
            Write-Host "Process Name: $($godotProcess.ProcessName)"
            Write-Host "Process ID (PID): $processId"
            Write-Host "Command Line: $($godotProcess.CommandLine)"
            
            @{
                ProcessId = $processId
                StartTime = Get-Date
            } | ConvertTo-Json | Out-File -FilePath $ProcessInfoFile
            
            return $godotProcess
        }
        else {
            Write-Host "Failed to start the Godot process using Start-Process."
            Write-Host "Please check the Godot executable path and the project path."
            return $null
        }
    }
    catch {
        Write-Host "Error starting Godot: $_"
        return $null
    }
}

function Stop-Godot {
    if (Test-Path $ProcessInfoFile) {
        $processInfo = Get-Content $ProcessInfoFile | ConvertFrom-Json
        $process = Get-Process -Id $processInfo.ProcessId -ErrorAction SilentlyContinue
        
        if ($process -and -not $process.HasExited) {
            Write-Host "Stopping Godot process (PID: $($process.Id))..."
            $process.Kill()
            $process.WaitForExit()
            Write-Host "Godot process stopped."
            Remove-Item $ProcessInfoFile -ErrorAction SilentlyContinue
        }
        else {
            Write-Host "No running Godot process found."
            Remove-Item $ProcessInfoFile -ErrorAction SilentlyContinue
        }
    }
    else {
        Write-Host "No Godot process information found."
    }
}

function Restart-Godot {
    Stop-Godot
    
    Write-Host "Building Rust project..."
    $buildSuccess = $false
    
    try {
        # First attempt: Use Start-Process with timeout monitoring
        Write-Host "Attempting primary build method: Start-Process with monitoring..."
        Write-Host "Running: cargo build --manifest-path ./rust/Cargo.toml"
        
        # Start the process without -Wait first
        $buildProcess = Start-Process -FilePath "cargo" -ArgumentList "build", "--manifest-path", "./rust/Cargo.toml" -NoNewWindow -PassThru
        
        Write-Host "Waiting for cargo build to complete..."
        
        while (-not $buildProcess.HasExited) {
            Start-Sleep -Milliseconds 500
        }
        
        if ($buildProcess.HasExited) {
            # Ensure the process has fully exited
            $buildProcess.WaitForExit(5000)
            
            Write-Host "Cargo build process completed with exit code: $($buildProcess.ExitCode)"
            
            if ($buildProcess.ExitCode -eq 0) {
                $buildSuccess = $true
                Write-Host "Primary build method succeeded."
            }
            else {
                Write-Host "Primary build method failed with exit code: $($buildProcess.ExitCode)"
            }
        }
    }
    catch {
        Write-Host "Error with primary build method: $_"
    }
    
    # Fallback to alternative method if primary failed
    if (-not $buildSuccess) {
        Write-Host "Trying alternative build method..."
        $buildSuccess = Invoke-CargoBuild -ManifestPath "./rust/Cargo.toml"
    }
    
    if (-not $buildSuccess) {
        Write-Host "All build methods failed. Cannot proceed with restart."
        return $null
    }
    
    Write-Host "Rust build completed successfully. Starting Godot..."
    Start-Sleep -Milliseconds 500  # Brief pause to ensure all file operations are complete
    
    try {
        $godotProcess = Start-Godot
        if ($godotProcess) {
            Write-Host "Godot started successfully after restart"
        }
        else {
            Write-Host "Failed to start Godot after restart"
        }
        return $godotProcess
    }
    catch {
        Write-Host "Error starting Godot after build: $_"
        return $null
    }
}

switch ($Command) {
    "start" {
        $godotProcess = Start-Godot
    }
    "stop" {
        Stop-Godot
    }
    "restart" {
        $godotProcess = Restart-Godot
    }
}