$tokens=$null
$errors=$null
[System.Management.Automation.Language.Parser]::ParseFile('D:/_dev/_projects/dev-boards/reference/bootstrap-github.ps1',[ref]$tokens,[ref]$errors) | Out-Null
if ($errors) {
    $errors | Format-List *
} else {
    'No errors'
}
