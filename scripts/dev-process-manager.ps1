# Development Process Manager for TaskNag
# Handles proper process cleanup for npm run tauri dev

param(
    [Parameter(Mandatory=$true)]
    [ValidateSet("start", "stop", "restart", "status")]
    [string]$Action
)

$ProjectRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$ProcessTrackingFile = Join-Path $ProjectRoot "scripts\dev-processes.json"

# Function to log with timestamp
function Write-Log {
    param([string]$Message, [string]$Level = "INFO")
    $Timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    Write-Host "[$Timestamp] [$Level] $Message"
}

# Function to get all processes in a tree
function Get-ProcessTree {
    param([int]$ParentPID)
    
    $children = Get-WmiObject Win32_Process | Where-Object { $_.ParentProcessId -eq $ParentPID }
    $result = @()
    
    foreach ($child in $children) {
        $result += $child
        $result += Get-ProcessTree -ParentPID $child.ProcessId
    }
    
    return $result
}

# Function to kill process tree
function Stop-ProcessTree {
    param([int]$ProcessId)
    
    try {
        $process = Get-Process -Id $ProcessId -ErrorAction SilentlyContinue
        if ($null -eq $process) {
            Write-Log "Process $ProcessId not found" "WARN"
            return
        }

        Write-Log "Stopping process tree for PID: $ProcessId ($($process.ProcessName))"
        
        # Get all child processes
        $children = Get-ProcessTree -ParentPID $ProcessId
        
        # Kill children first
        foreach ($child in $children) {
            try {
                Write-Log "Killing child process: $($child.ProcessId) ($($child.Name))"
                Stop-Process -Id $child.ProcessId -Force -ErrorAction SilentlyContinue
            }
            catch {
                Write-Log "Failed to kill child process $($child.ProcessId): $($PSItem.Exception.Message)" "WARN"
            }
        }
        
        # Kill parent process
        try {
            Write-Log "Killing main process: $ProcessId"
            Stop-Process -Id $ProcessId -Force
        }
        catch {
            Write-Log "Failed to kill main process ${ProcessId}: $($PSItem.Exception.Message)" "ERROR"
        }
        
        # Wait a bit and verify
        Start-Sleep -Seconds 2
        $stillRunning = Get-Process -Id $ProcessId -ErrorAction SilentlyContinue
        if ($stillRunning) {
            Write-Log "Process $ProcessId still running after kill attempt" "WARN"
        } else {
            Write-Log "Process tree successfully terminated"
        }
    }
    catch {
        Write-Log "Error stopping process tree: $($PSItem.Exception.Message)" "ERROR"
    }
}

# Function to check port availability
function Test-Port {
    param([int]$Port)
    
    try {
        $connections = Get-NetTCPConnection -LocalPort $Port -ErrorAction SilentlyContinue
        return $connections.Count -eq 0
    }
    catch {
        return $true  # Assume available if we can't check
    }
}

# Function to find processes using specific ports
function Get-ProcessByPort {
    param([int]$Port)
    
    try {
        $netstatOutput = netstat -ano | Where-Object { $_ -match ":$Port\s" }
        $pids = @()
        
        foreach ($line in $netstatOutput) {
            if ($line -match "LISTENING\s+(\d+)") {
                $pids += [int]$matches[1]
            }
        }
        
        return $pids | Select-Object -Unique
    }
    catch {
        Write-Log "Error finding processes by port: $($PSItem.Exception.Message)" "ERROR"
        return @()
    }
}

# Function to save running process info
function Save-ProcessInfo {
    param([int]$ProcessId, [string]$ProcessName, [string]$Command)
    
    $processInfo = @{
        PID = $ProcessId
        ProcessName = $ProcessName
        Command = $Command
        StartTime = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
        Ports = @(5173, 1420)  # Common development ports
    }
    
    try {
        $processInfo | ConvertTo-Json | Set-Content -Path $ProcessTrackingFile
        Write-Log "Process info saved: PID $ProcessId"
    }
    catch {
        Write-Log "Failed to save process info: $($PSItem.Exception.Message)" "WARN"
    }
}

# Function to load process info
function Get-SavedProcessInfo {
    try {
        if (Test-Path $ProcessTrackingFile) {
            $content = Get-Content -Path $ProcessTrackingFile -Raw
            return $content | ConvertFrom-Json
        }
    }
    catch {
        Write-Log "Failed to load process info: $($PSItem.Exception.Message)" "WARN"
    }
    return $null
}

# Function to cleanup orphaned processes
function Clear-OrphanedProcesses {
    Write-Log "Checking for orphaned development processes..."
    
    # Check for orphaned npm/node processes
    $nodeProcesses = Get-Process -Name node -ErrorAction SilentlyContinue | Where-Object {
        $_.Id -ne $PID  # Don't kill this script's process
    }
    
    foreach ($proc in $nodeProcesses) {
        try {
            $commandLine = (Get-WmiObject Win32_Process -Filter "ProcessId = $($proc.Id)").CommandLine
            
            # Check if it's a development-related process
            if ($commandLine -match "(vite|npm.*dev|task-manager)" -and $commandLine -notmatch "claude") {
                Write-Log "Found orphaned development process: PID $($proc.Id) - $commandLine"
                
                # Check if it's using our development ports
                $usingDevPorts = $false
                foreach ($port in @(5173, 1420, 3000, 8080)) {
                    $portPids = Get-ProcessByPort -Port $port
                    if ($portPids -contains $proc.Id) {
                        Write-Log "Process $($proc.Id) is using port $port"
                        $usingDevPorts = $true
                        break
                    }
                }
                
                if ($usingDevPorts) {
                    Write-Log "Terminating orphaned process: PID $($proc.Id)"
                    Stop-ProcessTree -ProcessId $proc.Id
                }
            }
        }
        catch {
            Write-Log "Error checking process $($proc.Id): $($PSItem.Exception.Message)" "WARN"
        }
    }
}

# Main action handlers
switch ($Action) {
    "start" {
        Write-Log "Starting development environment..."
        
        # First, clean up any orphaned processes
        Clear-OrphanedProcesses
        
        # Ensure port 5173 is available
        if (-not (Test-Port -Port 5173)) {
            Write-Log "Port 5173 is occupied. Attempting to free it..."
            $portPids = Get-ProcessByPort -Port 5173
            foreach ($pid in $portPids) {
                Stop-ProcessTree -ProcessId $pid
            }
            
            Start-Sleep -Seconds 3
            if (-not (Test-Port -Port 5173)) {
                Write-Log "Failed to free port 5173" "ERROR"
                exit 1
            }
        }
        
        Write-Log "Port 5173 is available"
        
        # Change to project directory
        Set-Location $ProjectRoot
        
        # Start the development server
        Write-Log "Starting npm run tauri dev..."
        $devProcess = Start-Process -FilePath "npm" -ArgumentList "run", "tauri", "dev" -PassThru -WindowStyle Normal
        
        if ($devProcess) {
            Save-ProcessInfo -ProcessId $devProcess.Id -ProcessName "npm" -Command "npm run tauri dev"
            Write-Log "Development server started with PID: $($devProcess.Id)"
        } else {
            Write-Log "Failed to start development server" "ERROR"
            exit 1
        }
    }
    
    "stop" {
        Write-Log "Stopping development environment..."
        
        # Load saved process info
        $savedInfo = Get-SavedProcessInfo
        if ($savedInfo) {
            Write-Log "Found saved process info for PID: $($savedInfo.PID)"
            Stop-ProcessTree -ProcessId $savedInfo.PID
        }
        
        # Also clean up any orphaned processes
        Clear-OrphanedProcesses
        
        # Remove tracking file
        if (Test-Path $ProcessTrackingFile) {
            Remove-Item $ProcessTrackingFile -Force
            Write-Log "Process tracking file removed"
        }
        
        Write-Log "Development environment stopped"
    }
    
    "restart" {
        Write-Log "Restarting development environment..."
        & $PSCommandPath -Action stop
        Start-Sleep -Seconds 5
        & $PSCommandPath -Action start
    }
    
    "status" {
        Write-Log "Development environment status:"
        
        $savedInfo = Get-SavedProcessInfo
        if ($savedInfo) {
            $process = Get-Process -Id $savedInfo.PID -ErrorAction SilentlyContinue
            if ($process) {
                Write-Log "Main process: PID $($savedInfo.PID) ($($process.ProcessName)) - RUNNING"
                Write-Log "Start time: $($savedInfo.StartTime)"
            } else {
                Write-Log "Main process: PID $($savedInfo.PID) - NOT RUNNING (orphaned tracking file)"
            }
        } else {
            Write-Log "No saved process information found"
        }
        
        # Check port status
        foreach ($port in @(5173, 1420)) {
            $isAvailable = Test-Port -Port $port
            $status = if ($isAvailable) { "AVAILABLE" } else { "OCCUPIED" }
            Write-Log "Port $port : $status"
            
            if (-not $isAvailable) {
                $portPids = Get-ProcessByPort -Port $port
                foreach ($pid in $portPids) {
                    $process = Get-Process -Id $pid -ErrorAction SilentlyContinue
                    if ($process) {
                        Write-Log "  └─ Occupied by PID $pid ($($process.ProcessName))"
                    }
                }
            }
        }
    }
}

Write-Log "Action '$Action' completed"