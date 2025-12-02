# Setup script for WS2-Root (local2)
# This syncs RR-Folder1, RR-Folder3, RR-Folder4 from the private remote repo

param(
    [Parameter(Mandatory=$true)]
    [string]$RemoteRepoUrl,
    
    [Parameter(Mandatory=$false)]
    [string]$WorkspaceRoot = "WS2-Root",
    
    [Parameter(Mandatory=$false)]
    [string]$Branch = "main"
)

$FoldersToSync = @("RR-Folder1", "RR-Folder3", "RR-Folder4")
$FolderMapping = @{
    "RR-Folder1" = "L2-Folder1"
    "RR-Folder3" = "L2-Folder2"
    "RR-Folder4" = "L2-Folder3"
}

& .\setup-sparse-checkout.ps1 `
    -RemoteRepoUrl $RemoteRepoUrl `
    -WorkspaceRoot $WorkspaceRoot `
    -LocalFolderName "local2" `
    -FoldersToSync $FoldersToSync `
    -Branch $Branch `
    -FolderMapping $FolderMapping

