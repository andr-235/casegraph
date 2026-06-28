#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionDenyReason {
    RoleDenied,
    PolicyDenied { policy_key: &'static str },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionDecision {
    Allow,
    Deny {
        reason: PermissionDenyReason,
        message: &'static str,
    },
}

impl PermissionDecision {
    pub fn allowed(&self) -> bool {
        matches!(self, Self::Allow)
    }
}
