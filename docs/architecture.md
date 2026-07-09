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
│   │   └── styles/                   # CSS
│   │       ├── globals.css           #   Глобальные стили
│   │       └── tokens.css            #   Дизайн-токены (цвета, радиусы, шрифты)
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
│   │   ├── cases/                    #   CasesPage (список дел)
│   │   ├── case-workspace/           #   CaseWorkspacePage (три панели)
│   │   │   ├── CaseWorkspacePage.tsx #     Основной layout
│   │   │   ├── CaseSidebar.tsx       #     Левая панель (группы)
│   │   │   ├── CaseOverviewPage.tsx  #     Обзор дела (KPI)
│   │   │   ├── ObjectsPage.tsx       #     Таблица объектов
│   │   │   ├── RelationsPage.tsx     #     Таблица связей
│   │   │   ├── TimelinePage.tsx      #     Хронология
│   │   │   ├── GraphPage.tsx         #     Граф связей
│   │   │   └── inspector/            #     Inspector (правая панель)
│   │   │       ├── CaseInspector.tsx #       Маршрутизатор контента
│   │   │       └── useCaseInspector.ts #     Hook (open/close/invalidate)
│   │   ├── login/                    #   LoginPage
│   │   ├── first-admin/              #   FirstAdminSetupPage
│   │   └── ...                       #   Прочие страницы
│   └── shared/                       # Общий код
│       ├── api/invoke.ts             # invokeCommand<T>()
│       ├── lib/                      # can(), formatError()
│       ├── security/                 # protectedOperations
│       └── ui/                       # Переиспользуемые UI-компоненты
│           ├── AppShell.tsx          #   Каркас приложения (flex-контейнер)
│           ├── TopBar.tsx            #   Верхняя панель (breadcrumb + меню)
│           ├── ConfirmModal.tsx      #   Модальное окно подтверждения
│           └── inspector/            #   Inspector (общие компоненты)
│               ├── InspectorPanel.tsx #     Контейнер с анимацией
│               ├── InspectorHeader.tsx #    Заголовок с кнопкой закрытия
│               ├── InspectorSection.tsx #   Секция с заголовком
│               └── InspectorStat.tsx  #     Пара «метка—значение»
│
├── src-tauri/                        # Бэкенд (Rust + Tauri)
│   └── src/
│       ├── lib.rs                    # Регистрация всех команд
│       ├── commands/                 # Tauri-команды (тонкий слой)
│       │   └── case_summary_commands.rs  # get_case_summary, get_case_overview
│       ├── services/                 # Бизнес-логика
│       │   └── case_summary_service.rs   # Логика сводок и обзора
│       ├── repositories/             # Доступ к SQLite
│       │   └── case_summary_repository.rs # SQL-запросы сводок
│       ├── domain/                   # DTO, константы
│       ├── db/                       # connection.rs, migrations.rs
│       ├── security/                 # Сессия, пароли, авторизация
│       ├── audit/                    # Запись и чтение логов
│       ├── backup/                   # ZIP-архивы
│       ├── storage/                  # Файловое хранилище
│       ├── models/                   # AppSettingsDto
│       └── errors/                   # AppErrorDto, CommandResult<T>
```

## UI Architecture

### Трёхпанельный макет

CaseWorkspacePage использует трёхпанельный макет с collapsible sidebar и опциональной правой панелью Inspector:

```
┌─────────────────────────────────────────────────────────┐
│ TopBar (◈ CaseGrid / КодДела / Название дела)           │
├────────┬──────────────────────────────────┬──────────────┤
│Sidebar │        Content Area              │  Inspector   │
│ 64/232 │  (переключается по табам)        │  (360px)     │
│  px    │                                  │  push-layout │
│        │  • CaseOverviewPage              │              │
│ Группа │  • ObjectsPage                   │  Object/     │
│ 1      │  • MaterialsPage                 │  Material/   │
│ Группа │  • RelationsPage                 │  Relation/   │
│ 2      │  • GraphPage                     │  Event       │
│ Группа │  • TimelinePage                  │  Inspector   │
│ 3      │                                  │              │
│ Группа │                                  │              │
│ 4      │                                  │              │
└────────┴──────────────────────────────────┴──────────────┘
```

**Рендеринг в App.tsx:**

```tsx
// CasesPage — список дел, обёрнут в AppShell + TopBar
{!selectedCase && <CasesPage user={currentUser} ... />}

// CaseWorkspacePage — три панели при выбранном деле
{selectedCase && <CaseWorkspacePage caseItem={selectedCase} ... />}
```

### Inspector Pattern

Inspector — это read-only панель предпросмотра с быстрыми toggle-действиями. Полное редактирование — через Modal.

**Архитектура Inspector:**

| Уровень | Папка | Назначение |
|---------|-------|-----------|
| Shared (оболочка) | `shared/ui/inspector/` | `InspectorPanel` (контейнер), `InspectorHeader` (закрытие), `InspectorSection`, `InspectorStat` |
| Page-level switch | `pages/case-workspace/inspector/` | `CaseInspector` (switch по `target.type`), `useCaseInspector` (состояние, revision) |
| Feature content | `features/*/ui/` | `ObjectInspectorContent`, заглушки для Material/Relation/Event |

**Поток открытия:**

1. Пользователь кликает строку в ObjectsPage
2. Вызывается `inspector.open("object", objectId)` из `useCaseInspector`
3. `CaseWorkspacePage` передаёт `target` и `revision` в `CaseInspector`
4. `CaseInspector` рендерит `ObjectInspectorContent`
5. Повторный клик переключает Inspector на другой объект
6. Esc / × закрывает панель

**Использование revision:** при каждом `invalidate()` увеличивается счётчик `revision`. Компоненты Inspector используют его в `useEffect`/`key` для перезагрузки данных.

**Разделение Inspector/Modal:**

| | Inspector | Modal |
|---|---|---|
| Роль | Быстрый просмотр, toggle-действия | Полное редактирование |
| Область | Preview, isKey, includeInReport | Все поля, удаление, связь с материалами |
| Открытие | Клик по строке | Двойной клик / Enter / кнопка «Открыть карточку» |

### CaseSidebar

- 4 группы: **ОБЗОР** (Overview), **ДАННЫЕ** (Objects, Materials), **АНАЛИЗ** (Relations, Graph), **РЕЗУЛЬТАТ** (Timeline)
- Collapsible 64px (иконки) / 232px (иконки + подписи)
- Счётчики объектов, материалов, связей и событий загружаются из `get_case_summary`
- Переключение групп через `activeGroup` + `expanded` state

### Дизайн-токены

Тёмная тема через CSS custom properties в `src/app/styles/tokens.css`:

| Категория | Примеры токенов |
|-----------|----------------|
| Фоны | `--bg-app: #0b0e14`, `--bg-surface: #11161e`, `--bg-elevated: #1a1f2a`, `--bg-hover: rgba(255,255,255,0.05)` |
| Текст | `--text-primary: #e4e7ed`, `--text-secondary: #9ca3af`, `--text-muted: #6b7280` |
| Акцент | `--accent: #5b8def`, `--accent-hover: #7ba3f5`, `--danger: #ef4444` |
| Границы | `--border-subtle: rgba(255,255,255,0.08)`, `--border-default: rgba(255,255,255,0.12)` |
| Радиусы | `--radius-sm: 4px`, `--radius-md: 8px`, `--radius-lg: 12px` |
| Отступы | `--space-1: 4px` … `--space-6: 24px` |
| Layout | `--topbar-height: 48px`, `--inspector-width: 360px`, `--sidebar-collapsed: 64px`, `--sidebar-expanded: 232px` |
| Шрифты | `--font-ui: 'Inter', system-ui`, `--font-mono: 'JetBrains Mono', monospace` |

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
