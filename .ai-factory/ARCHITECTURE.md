# Архитектура: Structured Modules (Technical Layers)

## Обзор

CaseGraph использует архитектуру **Structured Modules** — модульную архитектуру с разделением по техническим слоям. Каждый модуль инкапсулирует свою функциональную область, а зависимости направлены строго вниз: от интерфейсного слоя к слою данных.

Для десктопного приложения на Tauri роль контроллеров выполняют Tauri-команды, роль сервисов — сервисный слой Rust, роль репозиториев — слой доступа к SQLite. На фронтенде фичи организованы по тому же принципу: `api/` (вызовы Tauri) → `model/` (типы) → `ui/` (компоненты).

## Обоснование решения

- **Тип проекта:** десктопное приложение для управления делами
- **Технологический стек:** Tauri v2 + React 19 + TypeScript + Rust + SQLite
- **Ключевой фактор:** средняя сложность домена, малая команда, офлайн-режим
- **Почему Structured Modules:** обеспечивает чёткие границы модулей без избыточного формализма Explicit Architecture, упрощает миграцию в будущем при росте сложности

## Структура папок

```
casegraph/
├── src/                              # Фронтенд (React + TypeScript)
│   ├── main.tsx                      # Точка входа
│   ├── App.tsx                       # Корневой компонент, управление состоянием
│   ├── app/                          # Ядро приложения
│   │   ├── api/appApi.ts             # Инициализация
│   │   ├── App.tsx                   # Состояние загрузки
│   │   ├── router.tsx                # Роутинг (enum BootstrapState)
│   │   └── styles/                   # Глобальные стили
│   ├── features/                     # Фиче-модули
│   │   ├── auth/                     # Аутентификация
│   │   │   ├── api/                  # Вызовы Tauri
│   │   │   └── model/                # Типы
│   │   ├── cases/                    # Управление делами
│   │   │   ├── api/
│   │   │   └── model/
│   │   ├── objects/                  # Объекты дела
│   │   │   ├── api/
│   │   │   ├── model/
│   │   │   └── ui/                   # UI-компоненты фичи
│   │   ├── relations/               # Связи между объектами
│   │   ├── materials/               # Материалы
│   │   ├── graph/                   # Графовая визуализация
│   │   ├── timeline/                # Временная шкала
│   │   ├── audit/                   # Журнал аудита
│   │   ├── backup/                  # Резервное копирование
│   │   ├── users/                   # Управление пользователями
│   │   └── settings/                # Настройки
│   ├── pages/                        # Страницы (сборка UI из фич)
│   │   ├── cases/                    # CasesPage, CreateCaseModal
│   │   ├── case-workspace/           # CaseWorkspacePage
│   │   ├── login/                    # LoginPage
│   │   ├── first-admin/              # FirstAdminSetupPage
│   │   ├── change-password/          # ChangePasswordPage
│   │   ├── audit-log/                # AuditLogPage
│   │   ├── backup/                   # BackupPage
│   │   ├── materials/                # MaterialsPage
│   │   └── settings/                 # SettingsPage
│   └── shared/                       # Общий код (кросс-модульный)
│       ├── api/                      # invoke.ts — обёртка Tauri invoke
│       ├── lib/                      # can(), formatError()
│       ├── security/                 # protectedOperations
│       ├── types/                    # Общие типы
│       └── ui/                       # Общие UI-компоненты (ConfirmModal)
│
├── src-tauri/                        # Бэкенд (Rust + Tauri)
│   └── src/
│       ├── main.rs                   # fn main() → lib::run()
│       ├── lib.rs                    # Регистрация всех команд, SessionState
│       ├── commands/                 # Tauri-команды (слой представления)
│       │   ├── app_commands.rs
│       │   ├── auth_commands.rs
│       │   ├── case_commands.rs
│       │   ├── material_commands.rs
│       │   ├── object_commands.rs
│       │   ├── relation_commands.rs
│       │   ├── graph_commands.rs
│       │   ├── timeline_commands.rs
│       │   ├── audit_commands.rs
│       │   ├── backup_commands.rs
│       │   ├── user_management_commands.rs
│       │   ├── settings_commands.rs
│       │   ├── restore_recovery_commands.rs
│       │   └── security_commands.rs
│       ├── services/                 # Бизнес-логика (слой приложения)
│       │   ├── auth_service.rs
│       │   ├── case_service.rs
│       │   ├── material_service.rs
│       │   ├── object_service.rs
│       │   ├── relation_service.rs
│       │   ├── graph_service.rs
│       │   ├── timeline_service.rs
│       │   ├── backup_service.rs
│       │   ├── user_management_service.rs
│       │   ├── settings_service.rs
│       │   ├── report_draft_service.rs
│       │   ├── protected_service_guard.rs
│       │   ├── protected_policy_guard.rs
│       │   └── protected_access_audit.rs
│       ├── repositories/             # Доступ к данным (слой данных)
│       │   ├── case_repository.rs
│       │   ├── material_repository.rs
│       │   ├── object_repository.rs
│       │   ├── relation_repository.rs
│       │   ├── graph_repository.rs
│       │   ├── timeline_repository.rs
│       │   ├── user_repository.rs
│       │   ├── user_management_repository.rs
│       │   ├── user_access_repository.rs
│       │   ├── role_repository.rs
│       │   └── settings_repository.rs
│       ├── domain/                   # Доменные модели, DTO, константы
│       ├── db/                       # SQLite: подключение, миграции
│       ├── security/                 # Аутентификация, авторизация
│       ├── audit/                    # Аудит (best-effort)
│       ├── backup/                   # ZIP-архивы
│       ├── models/                   # AppSettingsDto, SettingsCatalog
│       ├── storage/                  # Файловое хранилище материалов
│       └── errors/                   # AppErrorDto, CommandResult<T>
```

## Правила зависимостей

Зависимости направлены строго вниз. Внутренние слои никогда не зависят от внешних.

### Бэкенд (Rust)

- ✅ **commands → services** — команды вызывают сервисы
- ✅ **services → repositories** — сервисы вызывают репозитории
- ✅ **services → domain** — сервисы используют DTO и константы
- ✅ **repositories → db** — репозитории работают через `rusqlite::Connection`
- ✅ **repositories → domain** — репозитории возвращают Row-структуры, маппят в DTO
- ❌ **repositories → services** — репозитории не вызывают сервисы
- ❌ **services → commands** — сервисы не знают о Tauri-командах
- ❌ **domain → services** — DTO не зависят от бизнес-логики

### Фронтенд (TypeScript)

- ✅ **pages → features** — страницы собирают UI из фич
- ✅ **features/api → shared/api** — API-вызовы используют общую обёртку invoke
- ✅ **features/ui → shared/ui** — UI-компоненты могут использовать общие компоненты
- ✅ **features/model → shared/types** — типы могут расширять общие типы
- ✅ **features → shared/lib** — фичи используют общие утилиты
- ❌ **features/A → features/B** — фичи не зависят друг от друга напрямую

## Коммуникация между слоями

### Бэкенд: команда → сервис → репозиторий

```
Tauri Command (commands/)                — принимает payload, вызывает сервис
    ↓
Application Service (services/)          — бизнес-логика, валидация, проверка прав
    ↓
Repository (repositories/)              — SQL-запросы через rusqlite
    ↓
Database (db/)                           — SQLite (connection.rs, migrations.rs)
```

### Фронтенд: страница → API фичи → Tauri

```
Page (pages/)                    — управляет состоянием, собирает UI
    ↓
Feature API (features/*/api/)    — invokeCommand<T>("command_name", { payload })
    ↓
Tauri IPC (shared/api/invoke.ts) — разворачивает CommandResult, выбрасывает AppCommandError
    ↓
Tauri Command (Rust)             — десериализует payload, вызывает сервис
```

### Модульная изоляция на фронтенде

Каждая фича — независимый модуль. Страницы собирают фичи вместе. Фичи не импортируют друг друга напрямую — общая логика выносится в `shared/`.

```
pages/cases/CasesPage.tsx
    ├── features/cases/api/casesApi.ts       # getCases, createCase, ...
    ├── features/cases/model/caseTypes.ts    # CaseDto, CaseStatus, ...
    └── shared/lib/permissions.ts            # can(permissions, operation)

pages/case-workspace/CaseWorkspacePage.tsx
    ├── features/objects/api/objectsApi.ts
    ├── features/relations/api/relationsApi.ts
    ├── features/timeline/api/timelineApi.ts
    ├── features/graph/api/graphApi.ts
    └── shared/ui/ConfirmModal.tsx
```

## Ключевые принципы

1. **Границы модулей:** каждая фича и каждый бэкенд-модуль инкапсулируют свою область. Наружу экспортируется только публичное API.
2. **Однонаправленный поток данных:** от UI → команды Tauri → сервисы → репозитории → БД. Ответ возвращается обратно тем же путём.
3. **Единый контракт API:** `CommandResult<T> = { ok: true, data: T } | { ok: false, error: AppErrorDto }` для всех команд.
4. **Domain Awareness на бэкенде:** доменная логика и константы (статусы, типы, права) живут в `domain/`, не размазаны по сервисам.
5. **Минимальный shared:** папка `shared/` содержит только по-настоящему общий код. Логика, специфичная для фичи, остаётся внутри фичи.

## Примеры кода

### Бэкенд: цепочка команда → сервис → репозиторий

```rust
// commands/case_commands.rs — Слой команд (Presentation)
#[tauri::command]
pub fn get_cases(app: AppHandle, session: State<'_, SessionState>) -> CommandResult<Vec<CaseDto>> {
    match CaseService::get_cases(&app, &session) {
        Ok(response) => CommandResult::ok(response),
        Err(error) => CommandResult::err(error),
    }
}

// services/case_service.rs — Слой сервисов (Application)
impl CaseService {
    pub fn get_cases(app: &AppHandle, session: &SessionState) -> Result<Vec<CaseDto>, AppErrorDto> {
        // Проверка прав доступа
        let ctx = ProtectedServiceContext::require_operation(app, ProtectedOperation::CaseRead)?;
        // Вызов репозитория
        CaseRepository::get_all(&ctx.conn)
    }
}

// repositories/case_repository.rs — Слой репозиториев (Data)
impl CaseRepository {
    pub fn get_all(conn: &Connection) -> Result<Vec<CaseDto>, AppErrorDto> {
        let mut stmt = conn.prepare("SELECT id, case_code, title, ... FROM cases ORDER BY created_at DESC")
            .map_err(|e| AppErrorDto::database(e.to_string()))?;
        let rows = stmt.query_map([], |row| {
            Ok(CaseDto {
                id: row.get("id")?,
                case_code: row.get("case_code")?,
                title: row.get("title")?,
                // ...
            })
        }).map_err(|e| AppErrorDto::database(e.to_string()))?;
        // ...
    }
}
```

### Фронтенд: страница → API фичи → Tauri

```typescript
// features/cases/api/casesApi.ts — API фичи
import { invokeCommand } from "@/shared/api/invoke";
import type { CaseDto, CreateCasePayload, CreateCaseResponse } from "../model/caseTypes";

export function getCases(): Promise<CaseDto[]> {
  return invokeCommand<CaseDto[]>("get_cases");
}

export function createCase(payload: CreateCasePayload): Promise<CreateCaseResponse> {
  return invokeCommand<CreateCaseResponse>("create_case", { payload });
}

// shared/api/invoke.ts — Обёртка (разворачивает CommandResult)
export async function invokeCommand<T>(
  command: string,
  args?: Record<string, unknown>
): Promise<T> {
  const result = await invoke<CommandResult<T>>(command, args ?? {});
  if (!result.ok) {
    throw new AppCommandError(result.error);
  }
  return result.data;
}

// pages/cases/CasesPage.tsx — Страница (собирает UI)
function CasesPage() {
  const [cases, setCases] = useState<CaseDto[]>([]);
  useEffect(() => {
    getCases().then(setCases).catch(e => setError(formatError(e)));
  }, []);
  // ...
}
```

## Анти-паттерны

- ❌ **Пропуск слоёв:** страница напрямую вызывает `invoke` без API-функции фичи, команда напрямую обращается к БД минуя сервис
- ❌ **Обратные зависимости:** репозиторий не должен импортировать сервис, сервис не должен импортировать команду
- ❌ **Циклические зависимости между фичами:** фича `objects` не должна импортировать из `relations` и наоборот — вынести общее в `shared/`
- ❌ **Разрастание shared/:** не складировать в shared логику, которая принадлежит конкретной фиче
- ❌ **Бизнес-логика в командах:** команды — тонкий слой, только десериализация и вызов сервиса. Вся логика — в `services/`
- ❌ **Анемичные доменные модели:** DTO и константы в `domain/` должны содержать правила и инварианты, а не быть просто структурами данных
