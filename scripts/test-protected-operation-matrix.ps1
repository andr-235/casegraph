$ErrorActionPreference = "Stop"

Push-Location src-tauri

cargo fmt --check
cargo check

cargo test protected_operation_matrix
cargo test protected_service_context
cargo test policy_aware_permission
cargo test docx_policy
cargo test backup_policy

Pop-Location

powershell -ExecutionPolicy Bypass -File scripts/check-protected-operation-boundary.ps1

Write-Host "Protected operation matrix regression suite OK"
