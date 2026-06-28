use rusqlite::Connection;

use crate::security::permission_decision::{PermissionDecision, PermissionDenyReason};
use crate::security::protected_operation::ProtectedOperation;
use crate::security::session::CurrentUserDto;
use crate::services::settings_access_policy::SettingsAccessPolicy;

use crate::models::settings_catalog::{
    KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP, KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX,
};

pub struct PolicyAwarePermissionService;

impl PolicyAwarePermissionService {
    pub fn decide(
        conn: &Connection,
        user: &CurrentUserDto,
        operation: ProtectedOperation,
    ) -> PermissionDecision {
        match operation {
            ProtectedOperation::DocxExport => Self::decide_docx_export(conn, user),
            ProtectedOperation::BackupCreate => Self::decide_backup_create(conn, user),
            ProtectedOperation::BackupRestore => Self::administrator_only(user),
            ProtectedOperation::SettingsRead => Self::administrator_only(user),
            ProtectedOperation::SettingsUpdate => Self::administrator_only(user),
            ProtectedOperation::UserManage => Self::administrator_only(user),
            ProtectedOperation::AuditLogRead => Self::audit_log_read(user),

            ProtectedOperation::CaseCreate
            | ProtectedOperation::CaseUpdate
            | ProtectedOperation::MaterialImport
            | ProtectedOperation::MaterialUpdate
            | ProtectedOperation::ObjectCreate
            | ProtectedOperation::ObjectUpdate
            | ProtectedOperation::RelationCreate
            | ProtectedOperation::RelationUpdate
            | ProtectedOperation::TimelineCreate
            | ProtectedOperation::TimelineUpdate
            | ProtectedOperation::ReportDraftGenerate
            | ProtectedOperation::ReportDraftUpdate
            | ProtectedOperation::IntegrityCheckRun => Self::analyst_or_admin(user),
        }
    }

    fn decide_docx_export(conn: &Connection, user: &CurrentUserDto) -> PermissionDecision {
        if user.is_administrator() || user.is_analyst() {
            return PermissionDecision::Allow;
        }

        if user.is_viewer() {
            match SettingsAccessPolicy::from_connection(conn) {
                Ok(policy) if policy.viewer_can_export_docx => {
                    return PermissionDecision::Allow;
                }
                Ok(_) => {
                    return PermissionDecision::Deny {
                        reason: PermissionDenyReason::PolicyDenied {
                            policy_key: KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX,
                        },
                        message: "Экспорт DOCX запрещён текущей политикой доступа.",
                    };
                }
                Err(_) => {
                    return PermissionDecision::Deny {
                        reason: PermissionDenyReason::PolicyDenied {
                            policy_key: KEY_ACCESS_VIEWER_CAN_EXPORT_DOCX,
                        },
                        message: "Не удалось проверить политику доступа к DOCX export.",
                    };
                }
            }
        }

        Self::role_denied("Недостаточно прав для экспорта DOCX.")
    }

    fn decide_backup_create(conn: &Connection, user: &CurrentUserDto) -> PermissionDecision {
        if user.is_administrator() {
            return PermissionDecision::Allow;
        }

        if user.is_analyst() {
            match SettingsAccessPolicy::from_connection(conn) {
                Ok(policy) if policy.analyst_can_create_backup => {
                    return PermissionDecision::Allow;
                }
                Ok(_) => {
                    return PermissionDecision::Deny {
                        reason: PermissionDenyReason::PolicyDenied {
                            policy_key: KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP,
                        },
                        message: "Создание backup запрещено текущей политикой доступа.",
                    };
                }
                Err(_) => {
                    return PermissionDecision::Deny {
                        reason: PermissionDenyReason::PolicyDenied {
                            policy_key: KEY_ACCESS_ANALYST_CAN_CREATE_BACKUP,
                        },
                        message: "Не удалось проверить политику доступа к backup.",
                    };
                }
            }
        }

        Self::role_denied("Недостаточно прав для создания backup.")
    }

    fn administrator_only(user: &CurrentUserDto) -> PermissionDecision {
        if user.is_administrator() {
            PermissionDecision::Allow
        } else {
            Self::role_denied("Действие доступно только администратору.")
        }
    }

    fn analyst_or_admin(user: &CurrentUserDto) -> PermissionDecision {
        if user.is_administrator() || user.is_analyst() {
            PermissionDecision::Allow
        } else {
            Self::role_denied("Недостаточно прав для выполнения действия.")
        }
    }

    fn audit_log_read(user: &CurrentUserDto) -> PermissionDecision {
        if user.is_administrator() || user.is_analyst() {
            PermissionDecision::Allow
        } else {
            Self::role_denied("Недостаточно прав для просмотра журнала.")
        }
    }

    fn role_denied(message: &'static str) -> PermissionDecision {
        PermissionDecision::Deny {
            reason: PermissionDenyReason::RoleDenied,
            message,
        }
    }
}
