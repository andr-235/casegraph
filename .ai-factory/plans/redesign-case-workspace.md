# План реализации: Редизайн рабочего пространства дела

Ветка: main (git.create_branches: false)
Создан: 2026-07-09
Обновлён: 2026-07-09 (рефайнмент 2 — 1 исправление, 4 улучшения; рефайнмент 3 — 1 исправление, 3 улучшения)

## Settings
- Testing: yes
- Logging: verbose
- Docs: yes

## Research Context

Тема: Редизайн рабочего пространства дела CaseGraph — тёмный desktop workspace с AppShell, сгруппированным sidebar, data table и правым Inspector вместо модалок.

Решения (из explore mode):
- Inspector: push-макет (sidebar → content → inspector), 360px, resize в будущем
- Inspector readonly + быстрые toggle-действия (★ ключевой, ● в справке), редактирование — через Modal
- Inspector close: Esc/×, клик по другой строке — переключение, повторный клик — ничего
- Инвалидация Inspector: revision counter в useCaseInspector
- Sidebar: grouping (ОБЗОР / ДАННЫЕ / АНАЛИЗ / РЕЗУЛЬТАТ), collapsible (64px/232px), счётчики из get_case_summary
- Колонка «В справке» в таблице объектов: оставить (●/○)
- Overview: один endpoint get_case_overview, без audit log, агрегация из рабочих таблиц
- Settings: убрать из CaseWorkspaceSection, глобальные настройки — на уровне App
- CaseSummary: fetch при входе + refetch после структурных мутаций (create/delete)
- Архитектура Inspector: shared/ui/inspector/ (оболочка) + pages/case-workspace/inspector/ (switch) + features/*/ui/ (контент)
- Первый vertical slice: AppShell + Sidebar + ObjectsPage + ObjectInspector
- **Known limitation:** глобальные страницы (Настройки, Резервные копии, Журнал аудита, Пользователи) при навигации из TopBar-меню рендерятся без AppShell/TopBar — они имеют собственный `onBack` и inline header. После первого slice они визуально выпадают из тёмной темы. Планируется отдельная задача по адаптации всех глобальных страниц под AppShell
- Дизайн-токены: CSS custom properties (цвета, радиусы, шрифты, spacing, layout)

## Commit Plan

- **Commit 1** (после задач 1–5): "feat: add CSS design tokens, TS types, Rust module registration, and backend case summary/overview endpoints"
- **Commit 2** (после задач 6–9): "feat: add AppShell, TopBar, redesigned Sidebar, three-panel workspace layout, and Inspector framework"
- **Commit 3** (после задач 10–12): "feat: add ObjectInspectorContent, redesigned ObjectsPage, and Esc handler"
- **Commit 4** (после задач 13–15): "feat: redesign CaseOverviewPage, adapt remaining pages, add tests and docs"

## Tasks

### Этап 1: Фундамент

- [x] **Task 1: CSS-токены, сбросы стилей и TypeScript-типы для нового UI**
  - Файлы: `src/app/styles/tokens.css` (новый), `src/app/styles/globals.css` (импорт + сбросы), `src/shared/types/workspaceTypes.ts` (новый)
  - CSS custom properties: цвета (bg-app: `#0b0e14`, bg-sidebar: `#10141c`, bg-surface: `#151a24`, bg-elevated: `#1a202c`, bg-hover: `#1d2532`, bg-selected: `#162a46`; text-primary: `#e7eaf0`, text-secondary: `#919aaa`, text-muted: `#626c7c`; accent: `#5b8def`, accent-hover: `#76a0f5`; success: `#47b881`, warning: `#d9a441`, danger: `#e05d65`; border-subtle: `#202735`, border-default: `#2a3342`)
  - Layout-токены: `--sidebar-width: 232px`, `--sidebar-collapsed-width: 64px`, `--inspector-width: 360px`, `--topbar-height: 60px`
  - Радиусы: `--radius-sm: 6px`, `--radius-md: 8px`, `--radius-lg: 12px`
  - Отступы: `--space-1..6` (4px..24px)
  - Шрифты: `--font-ui: Inter, "Segoe UI", sans-serif`, `--font-mono: "JetBrains Mono", "Cascadia Code", monospace`
  - Глобальные сбросы в `globals.css`: `@import './tokens.css'` первой строкой, `body { background: var(--bg-app); color: var(--text-primary); font-family: var(--font-ui); margin: 0; }`, сброс цветов ссылок, стилизация скроллбара под тёмную тему (`scrollbar-color: var(--border-default) transparent`)
  - **Важно:** добавить `import "./app/styles/globals.css"` в `src/main.tsx` — текущий `globals.css` (550 строк) никуда не импортирован, что делает все CSS-правила мёртвым кодом
  - TS-типы: `CaseInspectorTarget` (discriminated union: object/material/relation/event), `CaseWorkspaceSection` (без settings: overview, materials, objects, relations, graph, timeline, report), `SidebarGroup`, `InspectorState { target: CaseInspectorTarget | null, revision: number }`, `CaseSummaryDto`, `CaseOverviewDto`
  - Логирование: INFO — количество определённых токенов и типов
  - Тесты: unit-тест на корректность структуры CaseInspectorTarget (все варианты union)

- [x] **Task 2: Бэкенд — доменные DTO и репозитории для CaseSummary и CaseOverview**
  - Файлы: `src-tauri/src/domain/case_summary.rs` (новый), `src-tauri/src/domain/case_overview.rs` (новый), `src-tauri/src/repositories/case_summary_repository.rs` (новый)
  - DTO (codebase-паттерн — все DTO используют `#[derive(Debug, Clone, Serialize)]`/`#[derive(Debug, Deserialize)]` и `#[serde(rename_all = "camelCase")]`):
    - `CaseSummaryDto` — поля: `objectCount: i64`, `keyObjectCount: i64`, `materialCount: i64`, `integrityIssueCount: i64`, `relationCount: i64`, `eventCount: i64`, `reportEventCount: i64`, `updatedAt: String` (макс. `updated_at` среди всех затронутых записей, ISO-строка). Сериализация с `#[serde(rename_all = "camelCase")]`
    - `CaseOverviewDto` — поля: `caseItem: CaseDto`, `summary: CaseSummaryDto`, `keyObjects: Vec<ObjectPreviewDto>`, `recentActivity: Vec<ActivityItemDto>`. Сериализация с `#[serde(rename_all = "camelCase")]`
    - `ObjectPreviewDto` — поля: `id: String`, `objectCode: String`, `objectType: String`, `title: String`, `isKey: bool`
    - `ActivityItemDto` — поля: `entityType: String`, `entityId: String`, `code: String`, `title: String`, `timestamp: String` (ISO), `action: String`
  - Payload-структуры (codebase-паттерн — все команды принимают `payload: SomePayload`, а не plain String): `GetCaseSummaryPayload { case_id: String }` — `#[derive(Debug, Deserialize)]` + `#[serde(rename_all = "camelCase")]` в `domain/case_summary.rs`; `GetCaseOverviewPayload { case_id: String }` — аналогично в `domain/case_overview.rs`
  - Репозиторий: `count_objects(conn, case_id)` — `SELECT COUNT(*) FROM objects WHERE case_id = ? AND is_deleted = 0`
  - Репозиторий: `count_key_objects(conn, case_id)` — `SELECT COUNT(*) FROM objects WHERE case_id = ? AND is_deleted = 0 AND is_key = 1`
  - Репозиторий: `count_materials(conn, case_id)` — `SELECT COUNT(*) FROM materials WHERE case_id = ? AND is_deleted = 0`
  - Репозиторий: `count_materials_with_integrity_issues(conn, case_id)` — `SELECT COUNT(*) FROM materials WHERE case_id = ? AND is_deleted = 0 AND integrity_status IN ('mismatch', 'missing', 'read_error')`. Значения из `domain/material_integrity_status.rs`: `MATERIAL_INTEGRITY_MISMATCH = "mismatch"`, `MATERIAL_INTEGRITY_MISSING = "missing"`, `MATERIAL_INTEGRITY_READ_ERROR = "read_error"`. Статус `not_checked` НЕ считается проблемой — это значение по умолчанию для непроверенных материалов
  - Репозиторий: `count_relations(conn, case_id)` — `SELECT COUNT(*) FROM relations WHERE case_id = ? AND is_deleted = 0`
  - Репозиторий: `count_events(conn, case_id)` — `SELECT COUNT(*) FROM timeline_events WHERE case_id = ? AND is_deleted = 0`
  - Репозиторий: `count_report_events(conn, case_id)` — `SELECT COUNT(*) FROM timeline_events WHERE case_id = ? AND is_deleted = 0 AND include_in_report = 1`
  - Репозиторий: `get_key_objects(conn, case_id, limit)` — `SELECT id, object_code, object_type, title, is_key FROM objects WHERE case_id = ? AND is_deleted = 0 AND is_key = 1 ORDER BY updated_at DESC LIMIT ?`
  - Репозиторий: `get_recent_activity(conn, case_id, limit)` — UNION ALL трёх запросов (objects, materials, relations) с `ORDER BY updated_at DESC LIMIT ?`, маппинг в `ActivityItemDto`
  - Логирование: DEBUG на каждый count-запрос с case_id и результатом, INFO при создании репозитория; ERROR на ошибки SQL
  - Тесты: интеграционные тесты на каждый count-метод с тестовой БД (in-memory SQLite). Паттерн: `Connection::open_in_memory()` + `run_migrations(&conn)` для создания схемы (или минимальный поднабор `CREATE TABLE IF NOT EXISTS` для задействованных таблиц: `objects`, `materials`, `relations`, `timeline_events`). Тесты в отдельном файле `src-tauri/src/repositories/case_summary_repository_tests.rs` (по аналогии с `*_tests.rs` в security/)

- [x] **Task 3: Бэкенд — сервисы и Tauri-команды для get_case_summary и get_case_overview**
  - Файлы: `src-tauri/src/services/case_summary_service.rs` (новый), `src-tauri/src/commands/case_summary_commands.rs` (новый)
  - Сервис: `CaseSummaryService::get_case_summary(app, session, case_id)` — вызывает `ProtectedServiceContext::require_operation(app, ProtectedOperation::CaseRead)`, затем все count-методы репозитория, собирает `CaseSummaryDto`
  - Сервис: `CaseSummaryService::get_case_overview(app, session, case_id)` — вызывает `get_case_summary` + `CaseRepository::get_case_by_id` (переиспользует существующий репозиторий) + `get_key_objects` + `get_recent_activity`, собирает `CaseOverviewDto`
  - Команды: `#[tauri::command] get_case_summary(app, session, payload: GetCaseSummaryPayload) → CommandResult<CaseSummaryDto>`, `#[tauri::command] get_case_overview(app, session, payload: GetCaseOverviewPayload) → CommandResult<CaseOverviewDto>`
  - Параметр `payload: GetCaseSummaryPayload` (codebase-паттерн — все команды с аргументами используют payload-структуры, см. `object_commands.rs`, `case_commands.rs`)
  - Логирование: DEBUG — вход/выход сервиса с case_id и количеством сущностей по каждому типу; INFO — успешное выполнение команды; ERROR — ошибки БД, случай когда дело не найдено, отказано в доступе
  - Тесты: интеграционные тесты (1) создание дела → проверка counts = 0 → добавление объектов/материалов/связей → проверка counts > 0 → проверка структуры CaseOverviewDto; (2) пустое дело (только создано) — все counts = 0, key_objects = [], recent_activity = []. Инфраструктура: `Connection::open_in_memory()` + `run_migrations(&conn)` для схемы БД. Тесты сервиса — в отдельном файле `src-tauri/src/services/case_summary_service_tests.rs`. Для `AppHandle` в тестах потребуется mocked app или рефакторинг под injectable connection

- [x] **Task 4: Регистрация новых Rust-модулей в mod.rs**
  - Файлы: `src-tauri/src/domain/mod.rs`, `src-tauri/src/repositories/mod.rs`, `src-tauri/src/services/mod.rs`, `src-tauri/src/commands/mod.rs`, `src-tauri/src/lib.rs`
  - `domain/mod.rs`: добавить `pub mod case_summary;` и `pub mod case_overview;` (алфавитный порядок, после `case_status`)
  - `repositories/mod.rs`: добавить `pub mod case_summary_repository;` (алфавитный порядок, после `case_repository`)
  - `services/mod.rs`: добавить `pub mod case_summary_service;` (алфавитный порядок, после `case_service`)
  - `commands/mod.rs`: добавить `pub mod case_summary_commands;` (алфавитный порядок, после `case_commands`)
  - `lib.rs`: добавить импорт `use commands::case_summary_commands::{get_case_summary, get_case_overview};` и зарегистрировать обе команды в `generate_handler![]` (после `update_case_status`)
  - Логирование: не требуется (чисто структурные изменения)
  - Тесты: проверка компиляции (`cargo check` или `cargo build`)

- [x] **Task 5: Фронтенд API — функции getCaseSummary и getCaseOverview**
  - Файлы: `src/features/cases/api/casesApi.ts` (добавить функции), `src/features/cases/model/caseTypes.ts` (добавить DTO)
  - Импортировать/определить `CaseSummaryDto`, `CaseOverviewDto`, `ObjectPreviewDto`, `ActivityItemDto` — либо в `caseTypes.ts`, либо реэкспорт из `src/shared/types/workspaceTypes.ts`
  - Функции: `getCaseSummary(caseId: string) → Promise<CaseSummaryDto>`, `getCaseOverview(caseId: string) → Promise<CaseOverviewDto>`
  - Паттерн: `invokeCommand<CaseSummaryDto>("get_case_summary", { payload: { caseId } })` (codebase-паттерн — все вызовы с аргументами используют `{ payload: ... }`, см. `objectsApi.ts`, `casesApi.ts`)
  - `getCaseOverview` возвращает `CaseOverviewDto`, где `caseItem: CaseDto` — существующий тип, переиспользуется из `get_case_by_id` на бэкенде
  - Логирование: DEBUG — вызов функции с case_id; ERROR — ошибка invoke (логируется в shared/invoke.ts)
  - Тесты: mock-тест на структуру вызова — проверка, что `invokeCommand` вызывается с правильным именем команды и аргументом

### Этап 2: Оболочка приложения

- [x] **Task 6: Компоненты AppShell и TopBar**
  - Файлы: `src/shared/ui/AppShell.tsx` (новый), `src/shared/ui/TopBar.tsx` (новый)
  - AppShell: принимает `children: ReactNode`, рендерит flex-контейнер на `height: 100vh`, `background: var(--bg-app)`, `flex-direction: column`
  - TopBar: высота `var(--topbar-height)`, `border-bottom: 1px solid var(--border-subtle)`, flex row, `align-items: center`, `padding: 0 var(--space-5)`
  - Слева: логотип «◈ CaseGraph» + разделитель «/» + `caseCode` + «/» + `title` (обрезать title до ~40 символов с `...`). Принимает пропсы `caseCode?: string`, `title?: string` — когда не в workspace, показывать только логотип
  - Справа: визуальная подсказка поиска «Ctrl+K» (серая pill-кнопка, без функциональности), кнопка «Обновить» (⟳, вызывает `onRefresh`), меню пользователя (👤 `displayName` ▾)
  - Выпадающее меню пользователя: абсолютно-позиционированный dropdown с пунктами «Настройки», «Резервные копии», «Журнал аудита», «Пользователи», разделитель, «Выйти». Пункты «Настройки»/«Копии»/«Аудит»/«Пользователи» вызывают соответствующие коллбэки (`onOpenSettings`, `onOpenBackup`, `onOpenAuditLog`, `onOpenUsers`), «Выйти» — `onLogout`
  - Пропсы TopBar: `caseCode?, title?, displayName, onRefresh, onOpenSettings, onOpenBackup, onOpenAuditLog, onOpenUsers, onLogout`
  - Поведение `onRefresh` (задаётся в CaseWorkspacePage): перезагружает `getCaseSummary(caseId)` для обновления счётчиков сайдбара и инкрементит `revision` в `useCaseInspector` для перезагрузки текущего Inspector-контента. Если приложение не в workspace — `onRefresh` может быть `undefined` или no-op
  - Логирование: DEBUG — рендер TopBar с displayName; INFO — клик по кнопке «Обновить», клик по пункту меню
  - Тесты: snapshot-тест AppShell с children; тест рендера TopBar с пропсами и без caseCode/title; тест открытия/закрытия dropdown меню

- [x] **Task 7: Редизайн CaseSidebar — группировка, сворачивание, счётчики**
  - Файлы: `src/pages/case-workspace/CaseSidebar.tsx` (полная замена)
  - Тип `CaseWorkspaceSection` (без settings): `"overview" | "materials" | "objects" | "relations" | "graph" | "timeline" | "report"`
  - Группировка: `ОБЗОР` (overview), `ДАННЫЕ` (objects, materials, relations), `АНАЛИЗ` (graph, timeline), `РЕЗУЛЬТАТ` (report)
  - Collapsible: кнопка сворачивания/разворачивания (иконка ◀▶), CSS transition `width 200ms ease`
  - Состояния: `expanded` (`var(--sidebar-width)`) — группа, иконка, название, счётчик; `collapsed` (`var(--sidebar-collapsed-width)`) — только иконки, tooltip при hover с названием раздела
  - Счётчики: загрузка `getCaseSummary(caseId)` при монтировании и при изменении `caseId`; маппинг `summary.objectCount` → счётчик «Объекты», `summary.materialCount` → «Материалы», `summary.relationCount` → «Связи», `summary.eventCount` → «Хронология»
  - Активный раздел: `background: var(--bg-selected)`, `border-left: 3px solid var(--accent)`
  - Внизу: разделитель (`border-top: 1px solid var(--border-subtle)`) + кнопка «← Все дела»
  - **Иконки временные:** группам и разделам назначаются Unicode-символы (◈⌕⇄◷). Они рендерятся по-разному на разных платформах. TODO: заменить на SVG-иконки после первого vertical slice
  - Логирование: DEBUG — загрузка summary (логировать значения всех счётчиков); INFO — переключение раздела (логировать старый и новый section); DEBUG — сворачивание/разворачивание
  - Тесты: рендер всех групп при expanded; рендер только иконок при collapsed; отображение счётчиков после мок-загрузки summary; вызов onSectionChange при клике

- [x] **Task 8: Рефакторинг CaseWorkspacePage — трёхпанельный макет**
  - Файлы: `src/pages/case-workspace/CaseWorkspacePage.tsx` (рефакторинг), `src/App.tsx` (адаптация)
  - **Зависит от:** Task 9 (useCaseInspector) — макет CaseWorkspacePage использует хук `useCaseInspector` для управления состоянием правой панели Inspector
  - Новый макет: `AppShell > TopBar + [CaseSidebar | WorkspaceContent | CaseInspector]`
  - WorkspaceContent: `flex: 1`, `overflow: auto`, `padding: var(--space-6)`, рендерит активную секцию через switch по `activeSection`
  - CaseInspector: условный рендер справа; когда `inspector.target !== null` → `width: var(--inspector-width)`, иначе `width: 0`. CSS transition `width 200ms ease`, `overflow: hidden` в свёрнутом состоянии
  - Удалить: рендер `SettingsPage` из CaseWorkspacePage (включая условие `activeSection === "settings" && can(...)`), `PlaceholderSection` (за ненадобностью), `sectionTitles` для `"settings"` и `"report"`
  - Убрать `"settings"` из `CaseWorkspaceSection` и `sectionTitles`; `"report"` оставить как заглушку (без PlaceholderSection — просто div с текстом «Справка — будет реализована позже»)
  - Пропсы CaseWorkspacePage: убрать `onLogout` (теперь в TopBar), добавить `onRefresh` (передаётся в TopBar), добавить `onOpenSettings/onOpenBackup/onOpenAuditLog/onOpenUsers` (пробрасываются в TopBar)
  - В `App.tsx`: CaseWorkspacePage передаёт новые коллбэки. Убедиться, что SettingsPage рендерится только через глобальный `showSettings` флаг, а не внутри workspace
  - Inspector state: импортировать `useCaseInspector` из Task 9, передать `inspector` и методы в CaseInspector; `close()` вызывается при смене `activeSection`
  - Логирование: DEBUG — смена activeSection (логировать старую и новую); INFO — открытие/закрытие Inspector (entityType + entityId)
  - Тесты: рендер с разными activeSection; проверка, что SettingsPage не рендерится внутри workspace; проверка, что `close()` вызывается при смене секции

### Этап 3: Inspector и объекты

- [x] **Task 9: InspectorPanel (shared) + useCaseInspector hook + CaseInspector**
  - Файлы: `src/shared/ui/inspector/InspectorPanel.tsx` (новый), `src/shared/ui/inspector/InspectorHeader.tsx` (новый), `src/shared/ui/inspector/InspectorSection.tsx` (новый), `src/shared/ui/inspector/InspectorStat.tsx` (новый), `src/pages/case-workspace/inspector/useCaseInspector.ts` (новый), `src/pages/case-workspace/inspector/CaseInspector.tsx` (новый), `src/pages/case-workspace/inspector/caseInspectorTypes.ts` (новый)
  - InspectorPanel: `border-left: 1px solid var(--border-subtle)`, `background: var(--bg-surface)`, `padding: var(--space-5)`, `overflow-y: auto`, transition width. Пропсы: `children`, `visible: boolean`
  - InspectorHeader: код сущности (моноширинный) + заголовок (жирный) + кнопка закрытия (×), кнопка «Открыть карточку →» (accent-цвет, `onOpenFullCard`). Пропсы: `code`, `title`, `onClose`, `onOpenFullCard?`
  - InspectorSection: группировка контента с заголовком (`font-size: 13px`, `color: var(--text-muted)`, uppercase). Пропсы: `title`, `children`
  - InspectorStat: метрика — label (сверху, muted) + value (снизу, жирный). Пропсы: `label`, `value`
  - useCaseInspector: `{ target: CaseInspectorTarget | null, revision: number }`, методы: `open(type, id)` — устанавливает target и сбрасывает revision в 0; `close()` — сбрасывает target в null; `invalidate()` — `revision++` (триггерит refetch в InspectorContent)
  - CaseInspector: switch по `target.type`, рендерит соответствующий `*InspectorContent`, передаёт `caseId` (из пропсов), `entityId` (из target.id), `revision`, и коллбэки (`onClose`, `onOpenFullCard`, `onInvalidate`). Если `target === null` → рендерит `null`
  - Логирование: DEBUG — open (entityType, entityId), close, invalidate (новый revision); INFO — переключение между сущностями
  - Тесты: unit-тест useCaseInspector (open устанавливает target и revision=0, close сбрасывает target, invalidate инкрементит revision); рендер CaseInspector с разными target

- [x] **Task 10: ObjectInspectorContent + разделение ObjectCardModal на ObjectEditModal**
  - Файлы: `src/features/objects/ui/ObjectInspectorContent.tsx` (новый), `src/features/objects/ui/ObjectEditModal.tsx` (новый — извлечено из ObjectCardModal), `src/features/objects/ui/ObjectCardModal.tsx` (удалить)
  - ObjectInspectorContent: пропсы — `caseId: string`, `objectId: string`, `revision: number`, `onClose: () => void`, `onOpenFullCard: () => void`, `onUpdated: (item: ObjectListItemDto) => void`
  - Загружает `ObjectDetailsDto` через `getObjectById(caseId, objectId)`, эффект зависит от `[objectId, revision]`
  - Inspector показывает: OBJ-код (моноширинный), ★ (если isKey), тип (русская метка через `getObjectTypeLabel`), название, значение, описание (до 3 строк, с `...` если длиннее)
  - Статистика: `InspectorStat` для связей (N) и материалов (N)
  - Связанные материалы: список (прокручиваемый, max-height 200px) — `materialCode` + `title`
  - Быстрые действия: toggle «Ключевой» (`★` — вызывает `updateObject` с `isKey: !current`, затем `invalidate()` и `onUpdated`), toggle «В справке» (`●`/`○` — вызывает `updateObject` с `includeInReport: !current`)
  - Кнопка «Открыть полную карточку →»: вызывает `onOpenFullCard()`, родитель (ObjectsPage) устанавливает `editingObjectId`
  - ObjectEditModal: извлекается из `ObjectCardModal.tsx` — форма редактирования полей (title, value, description, confidenceNote, isKey, includeInReport), привязка материалов (`linkObjectToMaterials`), удаление (`softDeleteObject`). Пропсы: `caseId`, `objectId`, `onClose`, `onUpdated`, `onDeleted`. `ObjectCardModal.tsx` удаляется
  - **Важно:** подтверждение удаления должно использовать `shared/ui/ConfirmModal` с `tone="danger"`, а не `window.confirm` (сейчас строка 191 ObjectCardModal.tsx)
  - Логирование: DEBUG — загрузка деталей объекта (objectCode); INFO — toggle isKey/includeInReport (старое → новое значение); INFO — открытие EditModal
  - Тесты: рендер InspectorContent с мок-данными ObjectDetailsDto; проверка вызова `updateObject` при toggle isKey и includeInReport; рендер EditModal с формой

- [ ] **Task 11: Редизайн ObjectsPage — таблица, фильтры, интеграция с Inspector**
  - Файлы: `src/pages/case-workspace/ObjectsPage.tsx` (рефакторинг)
  - Фильтр-бар: строка поиска (фильтр по `title` и `value`, debounce 200ms не обязателен — фильтрация мгновенная на небольших объёмах), выпадающие списки «Тип» (все + 12 значений из `objectTypeOptions`), «Ключевые» (все/да/нет), «В справке» (все/да/нет), кнопка «Сбросить» (сбрасывает все фильтры)
  - Фильтрация: клиентская, `useMemo` — фильтрует `items` по поисковой строке (`.includes()`, case-insensitive) и выбранным значениям фильтров
  - Таблица: компактные строки, `border-bottom: 1px solid var(--border-subtle)`, без вертикальных разделителей
  - Колонки: Код (моноширинный), Тип (русская метка), Название, Мат. (число), Связи (число), Ключ. (★ или пусто), Справка (● или ○)
  - Строки: `cursor: pointer`, hover → `background: var(--bg-hover)`, selected → `background: var(--bg-selected)`
  - Клик по строке → `inspector.open("object", objectItem.id)`
  - Двойной клик или Enter на выбранной строке → открыть ObjectEditModal (через `setEditingObjectId`)
  - Кнопка «+ Объект» → открывает `CreateObjectModal`
  - После создания: `onCreated` → добавить в начало списка + `inspector.invalidate()` если Inspector открыт
  - После обновления из Inspector (toggle): `onUpdated` → обновить элемент в списке
  - После удаления из EditModal: `onDeleted` → удалить из списка + `inspector.close()`
  - Убрать старую таблицу с 9 колонками, убрать рендер `ObjectCardModal` (заменён на Inspector + EditModal)
  - Логирование: DEBUG — применение фильтров (количество результатов до/после); INFO — клик по строке → Inspector (objectCode); INFO — двойной клик → EditModal
  - Тесты: рендер таблицы с мок-данными (5+ объектов); фильтрация по поиску (ввод текста → меньше строк); фильтрация по типу; клик по строке вызывает `inspector.open`

- [ ] **Task 12: Обработчик клавиши Esc для закрытия Inspector**
  - Файлы: `src/pages/case-workspace/CaseWorkspacePage.tsx` (добавить useEffect), или `src/shared/ui/inspector/InspectorPanel.tsx`
  - Добавить `useEffect` с `keydown`-листенером на `window`: при нажатии `Escape` вызывать `inspector.close()`
  - **Не закрывать Inspector**, если фокус находится внутри `input`, `textarea`, или `select` (проверка `document.activeElement?.tagName`), или если открыта модалка (проверка наличия `.modal-backdrop` в DOM)
  - Если Inspector уже закрыт (`target === null`) — ничего не делать
  - Очистка листенера в cleanup-функции `useEffect`
  - Логирование: DEBUG — нажатие Esc (логировать, был ли Inspector открыт); DEBUG — Esc проигнорирован (фокус в поле ввода)
  - Тесты: unit-тест — симуляция `Escape` при открытом Inspector → `close()` вызван; симуляция `Escape` при фокусе в `<input>` → `close()` не вызван

### Этап 4: Распространение и полировка

- [ ] **Task 13: Редизайн CaseOverviewPage с метриками из getCaseOverview**
  - Файлы: `src/pages/case-workspace/CaseOverviewPage.tsx` (рефакторинг)
  - **Зависит от:** Task 11 (ObjectsPage + Inspector) — Overview может навигировать на ObjectsPage с открытым Inspector при клике на ключевой объект
  - Загрузка: `getCaseOverview(caseId)` при монтировании; показывать скелетон (серые placeholder-блоки) во время загрузки
  - Статус: выпадающий список (уже есть), обёрнут в карточку с фоном `var(--bg-surface)` и `border-radius: var(--radius-md)`
  - KPI-карточки: 4 блока в ряд (`display: grid; grid-template-columns: repeat(4, 1fr); gap: var(--space-4)`). Каждая: `min-width: 140px`, `padding: var(--space-5)`, `background: var(--bg-surface)`, `border-radius: var(--radius-md)`, `border: 1px solid var(--border-subtle)`. Иконка (Unicode-символ без эмодзи: ◈⌕⇄◷ — временное, TODO: заменить на SVG-иконки) + число (крупный шрифт, `color: var(--text-primary)`) + подпись (`color: var(--text-secondary)`, `font-size: 13px`)
  - Ключевые объекты: горизонтальный список pill-кнопок `[★ OBJ-001 Иванов Иван]`, клик → `onSectionChange("objects")` + `inspector.open("object", id)` (требует доступа к inspector из CaseWorkspacePage → передать коллбэк `onNavigateToObject(objectId)`)
  - Последняя активность: список из 5–7 записей, каждая строка: `entityType`-иконка + `code` + `title` + относительное время (`timestamp`)
  - Форма редактирования: сохранить существующую логику `updateCase` и `updateCaseStatus`, но обернуть в раскрывающуюся секцию (collapsible details/summary или условный рендер по кнопке «Редактировать»)
  - Логирование: DEBUG — загрузка overview (количество ключевых объектов, активностей); INFO — клик по ключевому объекту → навигация
  - Тесты: рендер overview с мок-данными; проверка отображения KPI-карточек (4 штуки); проверка клика по ключевому объекту

- [ ] **Task 14: Адаптация остальных страниц под новый макет + удаление settings из workspace**
  - Файлы: `src/pages/case-workspace/RelationsPage.tsx`, `src/pages/case-workspace/GraphPage.tsx`, `src/pages/case-workspace/TimelinePage.tsx`, `src/pages/materials/MaterialsPage.tsx`, `src/pages/cases/CasesPage.tsx`, `src/App.tsx`
  - MaterialsPage: убрать дублирующий `<header>` (информация о деле теперь в TopBar), перевести на токены (`background: var(--bg-surface)`, `border: 1px solid var(--border-subtle)` для карточек), интегрировать MaterialInspectorContent — минимальная заглушка (Inspector показывает `materialCode` + `title` + `materialType` + `integrityStatus`; кнопка «Открыть карточку» ведёт на существующий MaterialCardModal)
  - RelationsPage: адаптировать макет (убрать header, перевести на токены), RelationInspectorContent — заглушка (Inspector показывает `relationCode` + `relationType` + source → target + `confidenceLevel`; кнопка «Открыть карточку» ведёт на существующий RelationCardModal)
  - GraphPage: адаптировать макет, NodeInspectorContent при клике на ноду графа — заглушка (Inspector показывает OBJ-код + название + тип; клик по ноде вызывает `inspector.open("object", nodeId)`)
  - TimelinePage: адаптировать макет, EventInspectorContent — заглушка (Inspector показывает `eventCode` + `title` + `eventDate` + `eventType`; кнопка «Открыть карточку» ведёт на существующий EventCardModal)
  - Все страницы: убрать `section > header > h1 + p` (информация о деле в TopBar), использовать consistent padding (`var(--space-6)`)
  - CasesPage: заменить собственный `<header>` (inline styles, светлая тема, `<hr />`) на AppShell + TopBar (только логотип «◈ CaseGraph» + меню пользователя, без caseCode/title). Список дел остаётся без изменений
  - `App.tsx`: финальная проверка — `CaseWorkspaceSection` содержит 7 значений без `"settings"`; `SettingsPage` рендерится только через `showSettings && can(permissions, settingsRead)` на уровне App
  - Логирование: DEBUG — рендер каждой адаптированной страницы (имя страницы + caseId)
  - Тесты: smoke-тест рендера каждой страницы в новом макете (проверка отсутствия старого header'а с h1)

- [ ] **Task 15: Тесты и документация**
  - Файлы: тесты в `src/__tests__/` и `src-tauri/tests/`, `docs/architecture.md` (обновление), `docs/api.md` (обновление)
  - Фронтенд-тесты: рендер CaseSidebar (expanded/collapsed/группы/счётчики после мок-API), InspectorPanel (открытие/закрытие/переключение между сущностями), ObjectsPage (таблица с данными, фильтрация по поиску и типу, выделение строки, двойной клик → EditModal), useCaseInspector (open/close/invalidate/revision), Esc-обработчик (закрытие, игнорирование при фокусе в input)
  - Бэкенд-тесты: интеграционные тесты `get_case_summary` (создание дела → counts = 0 → добавление объектов/материалов/связей/событий → counts > 0 → проверка конкретных значений), `get_case_overview` (key_objects содержит ★-объекты, recent_activity содержит последние изменения), тест на пустое дело (все counts = 0, key_objects = [], recent_activity = [])
  - Документация: обновить `docs/architecture.md` — добавить секцию «UI Architecture» с описанием трёхпанельного макета (AppShell → TopBar + Sidebar + Content + Inspector), паттерна Inspector (shared shell + page-level switch + feature content), дизайн-токенов. Обновить `docs/api.md` — добавить `get_case_summary` (аргументы, возвращаемый тип, пример ответа) и `get_case_overview` (аргументы, возвращаемый тип, пример ответа)
  - Финальная проверка: `cargo test` (все Rust-тесты), `npm test` (все frontend-тесты) — оба должны проходить без ошибок
  - Логирование: не требуется (задача на тесты и доку)
