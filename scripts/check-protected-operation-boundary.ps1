# check-protected-operation-boundary.ps1
# Regression grep-check: business services must not bypass ProtectedServiceContext.
#
# Run:  pwsh -File scripts\check-protected-operation-boundary.ps1

$ErrorActionPreference = "Stop"

Write-Host "=== Protected operation boundary check ===" -ForegroundColor Cyan

$anyFailed = $false

# ---------------------------------------------------------------------------
# 1. No direct PolicyAwarePermissionGuard::require calls outside security/
# ---------------------------------------------------------------------------
$directPolicyGuard = Select-String -Path "src-tauri/src/**/*.rs" -Pattern "PolicyAwarePermissionGuard::require" -SimpleMatch |
    Where-Object { $_.Path -notmatch '\\security\\' -and $_.Path -notmatch '_tests\.rs$' -and $_.Path -notmatch '\\tests\\.*\.rs$' }

if ($directPolicyGuard) {
    Write-Host ""
    Write-Host "[FAIL] Business services must not call PolicyAwarePermissionGuard directly." -ForegroundColor Red
    Write-Host "       Use ProtectedServiceContext::require_operation(...) instead." -ForegroundColor Yellow
    $directPolicyGuard | ForEach-Object {
        Write-Host "  $($_.Path):$($_.LineNumber): $($_.Line.Trim())"
    }
    $anyFailed = $true
}

# ---------------------------------------------------------------------------
# 2. No legacy protected context helpers outside context file and tests
# ---------------------------------------------------------------------------
$legacyProtectedContext = Select-String -Path "src-tauri/src/**/*.rs" -Pattern "require_protected_(user|administrator|analyst_or_admin)\(" -SimpleMatch |
    Where-Object { $_.Path -notmatch '\\security\\protected_service_context\.rs' -and $_.Path -notmatch '_tests\.rs$' -and $_.Path -notmatch '\\tests\\.*\.rs$' }

if ($legacyProtectedContext) {
    Write-Host ""
    Write-Host "[FAIL] Legacy protected context helpers still used outside security/tests." -ForegroundColor Red
    $legacyProtectedContext | ForEach-Object {
        Write-Host "  $($_.Path):$($_.LineNumber): $($_.Line.Trim())"
    }
    $anyFailed = $true
}

# ---------------------------------------------------------------------------
# 3. No string action names outside ProtectedOperation definition and tests
# ---------------------------------------------------------------------------
$stringActionNames = Select-String -Path "src-tauri/src/**/*.rs" -Pattern '"(case|material|object|relation|timeline|report|docx|backup|settings|user|audit|integrity)\.[a-z]+"' |
    Where-Object { $_.Path -notmatch '\\protected_operation\.rs' -and $_.Path -notmatch '_tests\.rs$' -and $_.Path -notmatch '\\tests\\.*\.rs$' }

if ($stringActionNames) {
    Write-Host ""
    Write-Host "[FAIL] String operation action names found outside ProtectedOperation." -ForegroundColor Red
    Write-Host "       Use ProtectedOperation::...action_name() instead." -ForegroundColor Yellow
    $stringActionNames | ForEach-Object {
        Write-Host "  $($_.Path):$($_.LineNumber): $($_.Line.Trim())"
    }
    $anyFailed = $true
}

# ---------------------------------------------------------------------------
# Summary
# ---------------------------------------------------------------------------
Write-Host ""
if ($anyFailed) {
    Write-Host "=== FAILED: protected operation boundary violations ===" -ForegroundColor Red
    exit 1
} else {
    Write-Host "=== PASSED: protected operation boundary OK ===" -ForegroundColor Green
}
