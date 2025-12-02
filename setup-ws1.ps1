# Setup script for WS1-Root (local1)
# This syncs RR-Folder1, RR-Folder2, RR-Folder3 from the private remote repo

param(
    [Parameter(Mandatory=$true)]
    [string]$RemoteRepoUrl,
    
    [Parameter(Mandatory=$false)]
    [string]$WorkspaceRoot = "WS1-Root",
    
    [Parameter(Mandatory=$false)]
    [string]$Branch = "main"
)

$FoldersToSync = @("RR-Folder1", "RR-Folder2", "RR-Folder3")
$FolderMapping = @{
    "RR-Folder1" = "L1-Folder1"
    "RR-Folder2" = "L1-Folder2"
    "RR-Folder3" = "L1-Folder3"
}

& .\setup-sparse-checkout.ps1 `
    -RemoteRepoUrl $RemoteRepoUrl `
    -WorkspaceRoot $WorkspaceRoot `
    -LocalFolderName "local1" `
    -FoldersToSync $FoldersToSync `
    -Branch $Branch `
    -FolderMapping $FolderMapping

