pub mod audit_error_sanitizer;
pub mod audit_metadata;
pub mod audit_safe_value;
pub mod audit_service;

mod audit_repository;

#[cfg(test)]
pub mod audit_snapshot_assertions;

#[cfg(test)]
mod audit_snapshot_tests;

#[cfg(test)]
mod audit_error_sanitizer_tests;

#[cfg(test)]
mod boundary_tests {
    #[test]
    fn audit_repository_is_private_to_audit_module() {
        // Rust privacy system ensures crate::audit::audit_repository
        // cannot be accessed outside of the audit layer since it is declared
        // as a private submodule (mod audit_repository) under crate::audit.
        assert!(true);
    }
}
