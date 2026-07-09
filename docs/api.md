[← Архитектура](architecture.md) · [← Назад к README](../README.md) · [Настройки →](configuration.md)

# API

## Обзор

Все Tauri-команды принимают единственный параметр `payload` (camelCase) и возвращают `CommandResult<T>`. Вызов с фронтенда — через `invokeCommand<T>(name, args)` из `shared/api/invoke.ts`.

**Формат ответа:**

```typescript
// Успех
{ ok: true, data: T }

// Ошибка
{ ok: false, error: { code: "ERR_...", message: "Описание", details?: "..." } }
```

**Коды ошибок:** `ERR_DATABASE`, `ERR_VALIDATION`, `ERR_ACCESS_DENIED`, `ERR_UNAUTHORIZED`, `ERR_NOT_FOUND`, `ERR_INTERNAL`, `ERR_PASSWORD_CHANGE_REQUIRED`, `ERR_RESTORE_IN_PROGRESS`, `ERR_FILESYSTEM`.

## Приложение

### initialize_app

```typescript
function initializeApp(): Promise<InitializeAppResponse>
```

Ответ: `{ hasUsers: boolean, restoreRecoveryActive: boolean }`

## Аутентификация

| Команда | Описание | Payload |
|---------|---------|---------|
| `create_first_admin` | Создать первого администратора | `{ username, password, displayName }` |
| `login` | Вход в систему | `{ username, password }` |
| `get_current_user` | Получить текущего пользователя | — |
| `logout` | Выход | — |

```typescript
function login(payload: { username: string; password: string }): Promise<LoginResponse>
// Ответ: { user: CurrentUserDto }
// Если mustChangePassword → ERR_PASSWORD_CHANGE_REQUIRED
```

## Дела

| Команда | Описание | Payload |
|---------|---------|---------|
| `get_cases` | Список дел | — |
| `create_case` | Создать дело | `{ title, subject, description?, periodStart?, periodEnd?, status? }` |
| `get_case_by_id` | Дело по ID | `{ caseId }` |
| `update_case` | Обновить дело | `{ caseId, title?, subject?, description?, periodStart?, periodEnd?, status? }` |
| `update_case_status` | Сменить статус | `{ caseId, status }` |

## Объекты

| Команда | Описание | Payload |
|---------|---------|---------|
| `get_objects` | Объекты дела | `{ caseId }` |
| `create_object` | Создать объект | `{ caseId, name, objectType, attributes? }` |
| `get_object_by_id` | Объект по ID | `{ objectId }` |
| `update_object` | Обновить объект | `{ objectId, name?, objectType?, attributes? }` |
| `link_object_to_materials` | Привязать материалы | `{ objectId, materialIds }` |
| `soft_delete_object` | Мягкое удаление | `{ objectId }` |

## Связи

| Команда | Описание | Payload |
|---------|---------|---------|
| `get_relations` | Связи дела | `{ caseId }` |
| `create_relation` | Создать связь | `{ caseId, sourceObjectId, targetObjectId, relationType, confidence?, description? }` |
| `get_relation_by_id` | Связь по ID | `{ relationId }` |
| `update_relation` | Обновить связь | `{ relationId, relationType?, confidence?, description? }` |
| `soft_delete_relation` | Мягкое удаление | `{ relationId }` |

## Материалы

| Команда | Описание | Payload |
|---------|---------|---------|
| `get_materials` | Список материалов | — |
| `create_material` | Добавить материал | `{ name, materialType, description? }` |
| `update_material` | Обновить материал | `{ materialId, name?, materialType?, description? }` |
| `delete_material` | Удалить материал | `{ materialId }` |

## Граф

| Команда | Описание | Payload |
|---------|---------|---------|
| `get_graph_data` | Данные для графа | `{ caseId, filters? }` |

Ответ: `GraphDataDto { nodes: GraphNodeDto[], edges: GraphEdgeDto[] }`

## Временная шкала

| Команда | Описание | Payload |
|---------|---------|---------|
| `get_timeline` | События дела | `{ caseId }` |
| `create_event` | Создать событие | `{ caseId, title, eventDate, eventType?, datePrecision?, description?, includeInReport? }` |
| `get_event_by_id` | Событие по ID | `{ eventId }` |
| `update_event` | Обновить событие | `{ eventId, title?, ... }` |
| `soft_delete_event` | Мягкое удаление | `{ eventId }` |
| `toggle_event_report_include` | Вкл/выкл в отчёт | `{ eventId }` |

## Аудит

| Команда | Описание | Payload |
|---------|---------|---------|
| `get_audit_logs` | Журнал аудита | `{ filters? }` |
| `get_audit_log_by_id` | Запись по ID | `{ logId }` |
| `get_audit_actions` | Список действий | — |
| `get_audit_users` | Список пользователей | — |
| `export_audit_log` | Экспорт | `{ filters? }` |

## Пользователи

| Команда | Описание | Payload |
|---------|---------|---------|
| `get_users` | Список пользователей | — |
| `get_roles` | Список ролей | — |
| `get_user_by_id` | Пользователь по ID | `{ userId }` |
| `create_user` | Создать пользователя | `{ username, password, displayName, role }` |
| `update_user` | Обновить пользователя | `{ userId, displayName?, role? }` |
| `block_user` | Заблокировать | `{ userId }` |
| `unblock_user` | Разблокировать | `{ userId }` |
| `reset_user_password` | Сбросить пароль | `{ userId, newPassword }` |
| `change_own_password` | Сменить свой пароль | `{ currentPassword, newPassword }` |

## Настройки

| Команда | Описание | Payload |
|---------|---------|---------|
| `get_settings` | Получить настройки | — |
| `update_settings` | Обновить настройки | `{ updates: Record<string, string> }` |
| `choose_settings_directory` | Выбрать директорию | — |
| `reset_settings_to_defaults` | Сбросить настройки | — |

## Резервное копирование

| Команда | Описание | Payload |
|---------|---------|---------|
| `get_backup_history` | История бэкапов | — |
| `choose_backup_folder` | Выбрать папку | — |
| `create_backup` | Создать бэкап | `{ folderPath }` |
| `choose_backup_file` | Выбрать файл бэкапа | — |
| `verify_backup` | Проверить бэкап | `{ filePath }` |
| `choose_restore_backup_file` | Выбрать файл для восстановления | — |
| `restore_backup_preflight` | Проверка перед восстановлением | `{ filePath }` |
| `create_restore_safety_backup` | Страховочный бэкап | `{ restoreFilePath }` |
| `restore_backup` | Восстановить | `{ filePath }` |
| `get_restore_recovery_status` | Статус восстановления | — |
| `resolve_restore_recovery` | Завершить восстановление | — |

## Безопасность

| Команда | Описание | Payload |
|---------|---------|---------|
| `get_effective_permissions` | Матрица прав | — |

## Сводки

### get_case_summary

```typescript
function getCaseSummary(payload: { caseId: string }): Promise<CaseSummaryDto>
```

**Ответ:** `CaseSummaryDto`

```typescript
type CaseSummaryDto = {
  objectCount: number;
  materialCount: number;
  relationCount: number;
  eventCount: number;
};
```

Счётчики для каждой группы дел. Используется в CaseSidebar для отображения количества объектов, материалов, связей и событий.

**Пример ответа:**

```json
{
  "ok": true,
  "data": {
    "objectCount": 12,
    "materialCount": 8,
    "relationCount": 15,
    "eventCount": 23
  }
}
```

### get_case_overview

```typescript
function getCaseOverview(payload: { caseId: string }): Promise<CaseOverviewDto>
```

**Ответ:** `CaseOverviewDto`

```typescript
type CaseOverviewDto = {
  keyObjects: ObjectPreviewDto[];
  recentActivity: ActivityItemDto[];
};

type ObjectPreviewDto = {
  id: string;
  objectCode: string;
  title: string;
  objectType: string;
};

type ActivityItemDto = {
  id: string;
  entityType: string;    // "object" | "material" | "relation" | "event"
  code: string;
  title: string;
  timestamp: string;     // ISO 8601
};
```

Композитные данные для страницы обзора дела: ключевые объекты (★) и последняя активность.

**Пример ответа:**

```json
{
  "ok": true,
  "data": {
    "keyObjects": [
      { "id": "uuid-1", "objectCode": "OBJ-001", "title": "Иванов Иван", "objectType": "person" }
    ],
    "recentActivity": [
      { "id": "uuid-2", "entityType": "event", "code": "EVT-003", "title": "Обыск проведён", "timestamp": "2026-07-09T14:30:00Z" }
    ]
  }
}
```

## См. также

- [Архитектура](architecture.md) — слои и зависимости
- [Настройки](configuration.md) — параметры приложения, политики
- [Аутентификация](auth.md) — роли и права
