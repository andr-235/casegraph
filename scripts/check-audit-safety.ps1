# check-audit-safety.ps1
# Regression grep-check: forbidden JSON field names must not be EMITTED
# (i.e., used as JSON object keys) by audit snapshot / technical-details
# builders.  Test assertions that call .get("...") are explicitly excluded.
#
# Run:  pwsh -File scripts\check-audit-safety.ps1

$ErrorActionPreference = "Stop"

Write-Host "=== Audit safety denylist check ===" -ForegroundColor Cyan

# ---------------------------------------------------------------------------
# Targets — only production builder files, no test files
# ---------------------------------------------------------------------------
$targets = @(
    "src-tauri/src/audit/audit_metadata.rs",
    "src-tauri/src/services/case_service.rs",
    "src-tauri/src/services/material_service.rs",
    "src-tauri/src/services/object_service.rs",
    "src-tauri/src/services/relation_service.rs",
    "src-tauri/src/services/report_draft_service.rs",
    "src-tauri/src/services/settings_service.rs",
    "src-tauri/src/services/timeline_service.rs",
    "src-tauri/src/services/user_management_service.rs",
    "src-tauri/src/services/audit_service.rs"
)

# ---------------------------------------------------------------------------
# Forbidden key patterns — each is the quoted JSON key name as it would
# appear in a json!({}) macro or struct field definition.
# Assertion lines (.get("...") / .is_none() / .is_some() / assert!) are
# excluded because they legitimately name the key they are *rejecting*.
# ---------------------------------------------------------------------------
$forbiddenSnapshotKeys = @(
    '"originalPath"',
    '"storedPath"',
    '"backupPath"',
    '"archivePath"',
    '"outputPath"',
    '"exportPath"',
    '"storageRoot"',
    '"thumbnailPath"',
    '"absolutePath"',
    '"filePath"',
    '"sourcePath"',
    '"targetPath"',
    '"tempDir"',
    '"workingDir"',
    '"fileContent"',
    '"archiveContent"',
    '"zipBytes"',
    '"docxBytes"',
    '"sqliteDump"',
    '"databaseDump"',
    '"base64"',
    '"editorState"',
    '"sections"',
    '"sectionContent"'
)

# Patterns that indicate a line is a test assertion rather than a key emission
$assertionIndicators = @('.get(', 'assert!', 'assert_eq!', 'assert_ne!', '#[should_panic', '// ')

function Is-AssertionLine([string]$line) {
    $trimmed = $line.Trim()
    foreach ($ind in $assertionIndicators) {
        if ($trimmed.Contains($ind)) { return $true }
    }
    return $false
}

$anyFailed = $false

foreach ($pattern in $forbiddenSnapshotKeys) {
    $found = $targets | Where-Object { Test-Path $_ } | ForEach-Object {
        $file = $_
        Select-String -Path $file -Pattern ([regex]::Escape($pattern)) -SimpleMatch |
            Where-Object { -not (Is-AssertionLine $_.Line) }
    }

    if ($found) {
        Write-Host ""
        Write-Host "[FAIL] Forbidden audit field emitted: $pattern" -ForegroundColor Red
        $found | ForEach-Object {
            Write-Host "  $($_.Filename):$($_.LineNumber): $($_.Line.Trim())" -ForegroundColor Yellow
        }
        $anyFailed = $true
    }
}

# ---------------------------------------------------------------------------
# Warn on raw json!() in service files (should use audit_metadata builders)
# ---------------------------------------------------------------------------
$serviceFiles = $targets | Where-Object { $_ -match "services" }

Write-Host ""
Write-Host "--- Checking for raw json!() in service files ---" -ForegroundColor Cyan
$jsonMacros = $serviceFiles | Where-Object { Test-Path $_ } | ForEach-Object {
    Select-String -Path $_ -Pattern 'json!\(' -SimpleMatch
}
if ($jsonMacros) {
    Write-Host "[WARN] Raw json!() macro found in service files:" -ForegroundColor Yellow
    Write-Host "       Prefer audit_metadata builders for audit JSON." -ForegroundColor Yellow
    $jsonMacros | ForEach-Object {
        Write-Host "  $($_.Filename):$($_.LineNumber): $($_.Line.Trim())"
    }
} else {
    Write-Host "No raw json!() macros. Good." -ForegroundColor Green
}

# ---------------------------------------------------------------------------
# Summary
# ---------------------------------------------------------------------------
Write-Host ""
if ($anyFailed) {
    Write-Host "=== FAILED: forbidden patterns found in audit builders ===" -ForegroundColor Red
    exit 1
} else {
    Write-Host "=== PASSED: audit safety denylist check ===" -ForegroundColor Green
}
