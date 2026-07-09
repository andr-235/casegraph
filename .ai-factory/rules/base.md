# Базовые правила проекта

> Автоматически обнаруженные конвенции на основе анализа кодовой базы. При необходимости отредактируйте.

## Соглашения об именовании

- Файлы Rust: `snake_case.rs` (напр. `case_service.rs`, `user_repository.rs`)
- Файлы TypeScript: `camelCase.ts` / `.tsx` (напр. `casesApi.ts`, `authTypes.ts`)
- React-страницы: `PascalCase.tsx` с суффиксом `Page` (напр. `CasesPage.tsx`)
- React-модальные окна: `PascalCase.tsx` с суффиксом `Modal` (напр. `CreateCaseModal.tsx`)
- Переменные/поля: `camelCase` в TypeScript, `snake_case` в Rust
- Типы/интерфейсы: `PascalCase` с суффиксами `Dto`, `Payload`, `Response` (TypeScript зеркалирует Rust DTO)
- Константы: `UPPER_SNAKE_CASE` в Rust, объект `as const` в TypeScript
- Tauri-команды: `snake_case`, один параметр `payload`, возврат `CommandResult<T>`

## Структура модулей

- **Фронтенд — feature-based:** каждая фича в `src/features/{name}/` содержит:
  - `api/` — вызовы Tauri-команд (обёртка `invokeCommand<T>`)
  - `model/` — TypeScript-типы (точное зеркало Rust DTO, `camelCase`)
  - `ui/` — UI-компоненты фичи
  - `lib/` — вспомогательные функции (опционально)
- **Страницы:** `src/pages/` собирают UI из фич, управляют состоянием страницы
- **Общие ресурсы:** `src/shared/` — `api/` (обёртка invoke), `lib/` (права, форматирование ошибок), `ui/` (общие компоненты)
- **Бэкенд — layer-based:**
  - `commands/` — тонкий слой Tauri-команд (сериализация/десериализация)
  - `services/` — бизнес-логика
  - `repositories/` — доступ к данным (SQL)
  - `domain/` — DTO, payload/response structs, константы
  - `db/` — подключение к SQLite, миграции

## Обработка ошибок

- Единый тип `CommandResult<T>` — `{ ok: true, data: T } | { ok: false, error: AppErrorDto }`
- `AppErrorDto` содержит `code` (префикс `ERR_`), `message` (русский), `details?`
- Все функции возвращают `Result<T, AppErrorDto>`
- Фабричные методы ошибок: `AppErrorDto::database()`, `::validation()`, `::access_denied()`, `::not_found()` и т.д.
- На фронтенде: `AppCommandError` выбрасывается из `invokeCommand<T>()`, обрабатывается через `try/catch` + `formatError(e)`
- Ошибки аудита: `best_effort` — пишутся в `eprintln!`, не прерывают основной поток

## Работа с API

- Каждая Tauri-команда принимает единственный `payload` с `#[serde(rename_all = "camelCase")]`
- Фронтенд: `invokeCommand<T>(command, { payload })` для команд с параметрами
- Фронтенд: `invokeCommand<T>(command)` для команд без параметров
- Регистрация команд: `tauri::generate_handler![]` в `lib.rs`

## Модели данных

- Rust DTO: `#[derive(Serialize/Deserialize)]`, `#[serde(rename_all = "camelCase")]`
- TypeScript: типы в `features/{name}/model/` точно зеркалируют Rust DTO
- ID: UUID v4
- Даты: ISO 8601 строки (chrono с serde)
- Коды дел: `CASE-001`, `CASE-002` и т.д. (автоинкремент)

## Аутентификация и авторизация

- argon2 для паролей, in-memory сессия (`SessionState`)
- 3 роли: `administrator`, `analyst`, `viewer`
- Матрица из 27+ операций (ключи: `"case.create"`, `"case.read"` и т.д.)
- Трёхуровневая проверка прав: ProtectedServiceContext → PolicyAwarePermissionGuard → PolicyAwarePermissionService
- Флаг `must_change_password` блокирует все операции, кроме `change_own_password`
- На фронтенде: `can(permissions, operation)` для проверки доступности действий

## Интерфейс

- Все строки UI на русском языке
- Модальные окна: `ConfirmModal` с тонами `danger`, `warning`, `neutral`
- Без внешней библиотеки роутинга — enum `BootstrapState`
