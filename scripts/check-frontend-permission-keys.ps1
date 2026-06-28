$ErrorActionPreference = "Stop"

Write-Host "=== Frontend permission keys check ===" -ForegroundColor Cyan

$anyFailed = $false

# ---------------------------------------------------------------------------
# 1. No direct role string comparisons outside auth permission model
# ---------------------------------------------------------------------------
$roleChecks = Select-String -Path "src/**/*.tsx" -Pattern "(role\s*===\s*['\""](administrator|analyst|viewer)['\""]|role\s*!==\s*['\""](administrator|analyst|viewer)['\""])" -SimpleMatch |
    Where-Object { $_.Path -notmatch '\\features\\auth\\model\\' -and $_.Path -notmatch '\\.test\.tsx$' -and $_.Path -notmatch '\\.test\.ts$' }

if ($roleChecks) {
    Write-Host ""
    Write-Host "[FAIL] Direct frontend role checks found outside auth permission model." -ForegroundColor Red
    Write-Host "       Use can(permissions, protectedOperations.*)." -ForegroundColor Yellow
    $roleChecks | ForEach-Object {
        Write-Host "  $($_.Path):$($_.LineNumber): $($_.Line.Trim())"
    }
    $anyFailed = $true
}

# ---------------------------------------------------------------------------
# 2. No operation string literals in component files
# ---------------------------------------------------------------------------
$operationStrings = Select-String -Path "src/**/*.tsx" -Pattern '"(case|material|object|relation|timeline|report|docx|backup|settings|user|audit|integrity)\.[a-z]+"' |
    Where-Object { $_.Path -notmatch '\\shared\\security\\protectedOperations\.ts' -and $_.Path -notmatch '\\.test\.tsx$' -and $_.Path -notmatch '\\.test\.ts$' }

if ($operationStrings) {
    Write-Host ""
    Write-Host "[FAIL] Protected operation string literals found outside protectedOperations.ts." -ForegroundColor Red
    $operationStrings | ForEach-Object {
        Write-Host "  $($_.Path):$($_.LineNumber): $($_.Line.Trim())"
    }
    $anyFailed = $true
}

# ---------------------------------------------------------------------------
# Summary
# ---------------------------------------------------------------------------
Write-Host ""
if ($anyFailed) {
    Write-Host "=== FAILED: frontend permission key violations ===" -ForegroundColor Red
    exit 1
} else {
    Write-Host "=== PASSED: frontend permission keys OK ===" -ForegroundColor Green
}
