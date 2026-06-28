$ErrorActionPreference = "Stop"

$matches = Select-String -Path "src-tauri/src/**/*.rs" -Pattern "require_protected_(user|administrator|analyst_or_admin)\(" -SimpleMatch |
    Where-Object { $_.Path -notmatch '_tests\.rs$' }

if ($matches) {
    Write-Host "[FAIL] Legacy protected wrappers remain in production code:" -ForegroundColor Red
    $matches | ForEach-Object {
        Write-Host "  $($_.Path):$($_.LineNumber): $($_.Line.Trim())"
    }
    exit 1
}

Write-Host "No legacy protected wrappers found" -ForegroundColor Green
