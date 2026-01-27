$tracksDir = 'conductor/tracks'
$commandsDir = '.gemini/commands/conductor/resume'
if (-not (Test-Path $commandsDir)) { New-Item -ItemType Directory -Path $commandsDir -Force }
$tracks = Get-ChildItem $tracksDir -Directory
foreach ($track in $tracks) {
    $trackName = $track.Name
    $filePath = Join-Path $commandsDir "$trackName.toml"
    $content = "description = 'Resume work on the $trackName track'`nprompt = 'Resume work on the Conductor track: $trackName. Please read conductor/tracks/$trackName/plan.md and check the current status to determine the next step.'"
    Set-Content -Path $filePath -Value $content -Encoding utf8
}
Write-Host "Synced $($tracks.Count) tracks."
