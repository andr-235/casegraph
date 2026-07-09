[← Быстрый старт](getting-started.md) · [← Назад к README](../README.md) · [API →](api.md)

# Архитектура

## Обзор

CaseGraph использует **Structured Modules (Technical Layers)** — модульную архитектуру с однонаправленными зависимостями. Бэкенд на Rust разделён на слои `commands → services → repositories → db`. Фронтенд на TypeScript — feature-based с разделением `api/`, `model/`, `ui/`.

## Диаграмма потока данных

```
UI (React)          →   Tauri API (invoke)   →   Rust Command   →   Service   →   Repository   →   SQLite
     ↑                                                                                               |
     └─────────────────────────── CommandResult<T> ───────────────────────────────────────────────────┘
```

## Структура проекта

```
casegraph/
├── src/                              # Фронтенд (React + TypeScript)
│   ├── main.tsx                      # Входная точка React
│   ├── App.tsx                       # Корневой компонент (BootstrapState)
│   ├── app/                          # Ядро приложения
│   │   ├── api/appApi.ts             # initializeApp()
│   │   └── styles/globals.css        # Глобальные стили
│   ├── features/                     # Фиче-модули
│   │   ├── auth/                     # Аутентификация
│   │   ├── cases/                    # Управление делами
│   │   ├── objects/                  # Объекты дела
│   │   ├── relations/               # Связи
│   │   ├── materials/               # Материалы
│   │   ├── graph/                   # Графовая визуализация
│   │   ├── timeline/                # Временная шкала
│   │   ├── audit/                   # Журнал аудита
│   │   ├── backup/                  # Резервное копирование
│   │   ├── users/                   # Пользователи
│   │   └── settings/                # Настройки
│   ├── pages/                        # Страницы (сборка из фич)
│   └── shared/                       # Общий код
│       ├── api/invoke.ts             # invokeCommand<T>()
│       ├── lib/                      # can(), formatError()
│       └── security/                 # protectedOperations
│
├── src-tauri/                        # Бэкенд (Rust + Tauri)
│   └── src/
│       ├── lib.rs                    # Регистрация всех команд
│       ├── commands/                 # Tauri-команды (тонкий слой)
│       ├── services/                 # Бизнес-логика
│       ├── repositories/             # Доступ к SQLite
│       ├── domain/                   # DTO, константы
│       ├── db/                       # connection.rs, migrations.rs
│       ├── security/                 # Сессия, пароли, авторизация
│       ├── audit/                    # Запись и чтение логов
│       ├── backup/                   # ZIP-архивы
│       ├── storage/                  # Файловое хранилище
│       ├── models/                   # AppSettingsDto
│       └── errors/                   # AppErrorDto, CommandResult<T>
```

## Правила зависимостей

### Бэкенд

Зависимости направлены строго вниз:

| Слой | Зависит от | Не зависит от |
|------|-----------|--------------|
| `commands/` | `services/`, `domain/` | — |
| `services/` | `repositories/`, `domain/`, `security/` | `commands/` |
| `repositories/` | `db/`, `domain/` | `services/`, `commands/` |
| `domain/` | — | всех остальных |
| `db/` | — | всех остальных |

- ✅ **commands → services → repositories → db**
- ❌ **repositories → services** — обратные зависимости запрещены
- ❌ **services → commands** — сервисы не знают о Tauri

### Фронтенд

| Слой | Зависит от | Не зависит от |
|------|-----------|--------------|
| `pages/` | `features/`, `shared/` | — |
| `features/` | `shared/` | других фич |
| `shared/` | — | `features/`, `pages/` |

- ✅ **pages → features → shared**
- ❌ **features/A → features/B** — фичи не зависят друг от друга
- ❌ **shared → features** — общий код не импортирует фичи

## Коммуникация

### Tauri-команда → Сервис → Репозиторий

```rust
// 1. Команда — принимает payload, вызывает сервис
#[tauri::command]
pub fn get_cases(app: AppHandle, session: State<'_, SessionState>) -> CommandResult<Vec<CaseDto>> {
    CaseService::get_cases(&app, &session).into()
}

// 2. Сервис — проверяет права, вызывает репозиторий
impl CaseService {
    pub fn get_cases(app: &AppHandle, session: &SessionState) -> Result<Vec<CaseDto>, AppErrorDto> {
        let ctx = ProtectedServiceContext::require_operation(app, ProtectedOperation::CaseRead)?;
        CaseRepository::get_all(&ctx.conn)
    }
}

// 3. Репозиторий — SQL-запрос
impl CaseRepository {
    pub fn get_all(conn: &Connection) -> Result<Vec<CaseDto>, AppErrorDto> {
        // SELECT ... FROM cases ORDER BY created_at DESC
    }
}
```

### Фронтенд → Tauri

```typescript
// features/cases/api/casesApi.ts
export function getCases(): Promise<CaseDto[]> {
  return invokeCommand<CaseDto[]>("get_cases");
}

// shared/api/invoke.ts — обёртка
export async function invokeCommand<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  const result = await invoke<CommandResult<T>>(command, args ?? {});
  if (!result.ok) throw new AppCommandError(result.error);
  return result.data;
}
```

## Ключевые принципы

1. **Однонаправленный поток:** UI → invoke → Rust Command → Service → Repository → SQLite
2. **Единый контракт:** `CommandResult<T> = { ok: true, data } | { ok: false, error: AppErrorDto }`
3. **Тонкие команды:** команды только десериализуют payload и вызывают сервис
4. **Минимальный shared:** общий код — только по-настоящему переиспользуемый
5. **Domain Awareness:** константы и правила в `domain/`, не размазаны по сервисам

## Анти-паттерны

- ❌ Пропуск слоёв: команда напрямую к БД, страница напрямую к `invoke`
- ❌ Обратные зависимости: репозиторий импортирует сервис
- ❌ Циклические зависимости между фичами
- ❌ Бизнес-логика в `commands/`
- ❌ Разрастание `shared/`

## См. также

- [Быстрый старт](getting-started.md) — установка и первый запуск
- [API](api.md) — справочник Tauri-команд
- [Аутентификация](auth.md) — роли и права
