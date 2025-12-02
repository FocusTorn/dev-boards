
function Write-BoxedHeader { #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [string]$Title,
        
        [Parameter(Mandatory = $false)]
        [int]$Width = 80
    )
    
    $displayTitle = if ($Title.Length % 2 -eq 1) { "$Title " } else { $Title }
    
    $padding = [Math]::Max(0, (($Width - $displayTitle.Length) / 2) - 1)
    
    $leftPad = " " * [Math]::Floor($padding)
    $rightPad = " " * [Math]::Floor($padding)
    $topBottom = "━" * ($Width - 2)
    $colorCyan = "`e[38;5;51m"
    $colorReset = "`e[0m"
    
    Write-Host "$colorCyan┏$topBottom┓$colorReset"
    Write-Host "$colorCyan┃$leftPad$displayTitle$rightPad┃$colorReset"
    Write-Host "$colorCyan┗$topBottom┛$colorReset"
    Write-Host ""

} #<

function Write-Header { #>
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [string]$Title,
        
        [Parameter(Mandatory = $false)]
        [int]$Width = 65
    )
    
    $taiLines = [Math]::Max(0, $Width - ($Title.Length + 4))
    $tail = "─" * ($taiLines)
    $colorBlue = "`e[38;5;33m"
    $colorReset = "`e[0m"
    
    
    Write-Host "$colorBlue┌─ $Title $tail$colorReset"
    Write-Host ""
} #<



Write-BoxedHeader -Title "GitHub SSH Status"

Write-Header -Title "Sub Header"
