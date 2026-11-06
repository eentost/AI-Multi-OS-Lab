[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [ValidateSet('Capabilities', 'List', 'Start', 'Stop', 'Get')]
    [string]$Action,
    [Parameter(Mandatory = $false)]
    [ValidateLength(1, 64)]
    [ValidatePattern('^[a-zA-Z0-9][a-zA-Z0-9._-]*$')]
    [string]$Name
)
$ErrorActionPreference = 'Stop'
try {
    switch ($Action) {
        'Capabilities' {
            [ordered]@{ worker='windows'; hypervisor='hyper-v'; operations=@('Capabilities','List','Start','Stop','Get'); arbitrary_command_execution=$false } | ConvertTo-Json -Compress
        }
        'List' {
            @(Get-VM | Select-Object Name, State, Generation, CPUUsage, MemoryAssigned, Uptime) | ConvertTo-Json -Depth 5 -Compress
        }
        'Get' {
            if (-not $Name) { throw 'Name is required.' }
            Get-VM -Name $Name -ErrorAction Stop | Select-Object Name, State, Generation, CPUUsage, MemoryAssigned, Uptime | ConvertTo-Json -Compress
        }
        'Start' {
            if (-not $Name) { throw 'Name is required.' }
            Start-VM -Name $Name -ErrorAction Stop | Out-Null
            Get-VM -Name $Name | Select-Object Name, State | ConvertTo-Json -Compress
        }
        'Stop' {
            if (-not $Name) { throw 'Name is required.' }
            Stop-VM -Name $Name -ErrorAction Stop
            Get-VM -Name $Name | Select-Object Name, State | ConvertTo-Json -Compress
        }
    }
} catch {
    [Console]::Error.WriteLine(($_ | Out-String))
    exit 2
}
