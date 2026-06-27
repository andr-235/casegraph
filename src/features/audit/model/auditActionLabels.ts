const auditActionLabels: Record<string, string> = {
  AUTH_LOGIN_SUCCEEDED: "Вход выполнен",
  AUTH_LOGIN_FAILED: "Ошибка входа",
  AUTH_LOGOUT: "Выход",

  CASE_CREATED: "Дело создано",
  CASE_UPDATED: "Дело изменено",
  CASE_STATUS_CHANGED: "Статус дела изменён",

  TIMELINE_EVENT_CREATED: "Событие создано",
  TIMELINE_EVENT_UPDATED: "Событие изменено",
  TIMELINE_EVENT_DELETED: "Событие удалено",
  TIMELINE_EVENT_REPORT_INCLUDE_CHANGED: "Включение события в DOCX изменено",

  USER_CREATED: "Пользователь создан",
  USER_UPDATED: "Пользователь изменён",
  USER_BLOCKED: "Пользователь заблокирован",
  USER_UNBLOCKED: "Пользователь разблокирован",
  USER_PASSWORD_RESET: "Пароль пользователя сброшен",
  USER_PASSWORD_CHANGED: "Пользователь сменил пароль",

  ACCESS_DENIED: "Отказ в доступе",
  AUDIT_LOG_EXPORTED: "Журнал экспортирован",

  // legacy aliases
  EVENT_CREATED: "Событие создано",
  EVENT_UPDATED: "Событие изменено",
  EVENT_DELETED: "Событие удалено",
  EVENT_REPORT_FLAG_CHANGED: "Включение события в DOCX изменено",
  EVENT_REPORT_INCLUDE_CHANGED: "Включение события в DOCX изменено",
  RESET_USER_PASSWORD: "Пароль пользователя сброшен",
  CHANGE_OWN_PASSWORD: "Пользователь сменил пароль",
};

export function formatAuditAction(action: string): string {
  return auditActionLabels[action] ?? action;
}
