[← API](api.md) · [← Назад к README](../README.md) · [Аутентификация →](auth.md)

# Настройки

## Обзор

Настройки приложения хранятся в SQLite (таблица `app_settings`). 11 настроек задаются при миграции и могут быть изменены администратором через UI или API.

## Список настроек

| Ключ | По умолчанию | Описание |
|------|-------------|---------|
| `backup_folder` | — | Папка для резервных копий |
| `backup_retention_count` | `30` | Количество хранимых бэкапов |
| `auto_backup_enabled` | `false` | Автоматическое резервное копирование |
| `auto_backup_interval_hours` | `24` | Интервал авто-бэкапа (часы) |
| `viewer_can_export_docx` | `false` | Может ли наблюдатель экспортировать отчёты |
| `analyst_can_create_backup` | `true` | Может ли аналитик создавать бэкапы |
| `session_timeout_minutes` | `0` | Таймаут сессии (0 — без таймаута) |
| `audit_retention_days` | `365` | Срок хранения аудит-логов |
| `material_storage_folder` | — | Папка для файлов материалов |
| `default_case_status` | `draft` | Статус по умолчанию для новых дел |
| `report_template` | — | Шаблон отчёта |

## Политики доступа

Две настройки управляют дополнительными правами:

| Настройка | Роль | Право |
|----------|------|------|
| `viewer_can_export_docx` | `viewer` | Экспорт отчётов в DOCX |
| `analyst_can_create_backup` | `analyst` | Создание резервных копий |

По умолчанию эти права есть только у `administrator`. Политики позволяют расширить доступ без изменения роли.

## Матрица прав

Права вычисляются динамически через `get_effective_permissions`. 27+ операций сгруппированы по модулям:

| Модуль | Операции |
|--------|---------|
| `case.*` | `create`, `read`, `update`, `delete`, `updateStatus` |
| `object.*` | `create`, `read`, `update`, `softDelete` |
| `relation.*` | `create`, `read`, `update`, `softDelete` |
| `material.*` | `create`, `read`, `update`, `delete` |
| `timeline.*` | `create`, `read`, `update`, `softDelete`, `toggleReportInclude` |
| `audit.*` | `read`, `export` |
| `user.*` | `create`, `read`, `update`, `block`, `unblock`, `resetPassword` |
| `settings.*` | `read`, `update`, `resetToDefaults` |
| `backup.*` | `create`, `verify`, `restore`, `readHistory` |
| `graph.*` | `read` |

## Проверка прав на фронтенде

```typescript
import { can } from "@/shared/lib/permissions";
import { protectedOperations } from "@/shared/security/protectedOperations";

if (can(permissions, protectedOperations.caseCreate)) {
  // Кнопка «Создать дело» активна
}
```

## Сброс настроек

`reset_settings_to_defaults` сбрасывает все настройки к заводским значениям. Только для `administrator`.

## См. также

- [API](api.md) — полный список команд
- [Аутентификация](auth.md) — роли и права доступа
- [Быстрый старт](getting-started.md) — установка и первый запуск
