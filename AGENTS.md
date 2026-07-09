# AGENTS.md

> Этот файл обслуживается AI Factory. Не удаляйте. Описывает структуру проекта для AI-агентов.

## Обзор проекта

CaseGraph — десктопное приложение для управления делами. Tauri v2 + React 19 + TypeScript + SQLite. Офлайн-режим, три роли, визуализация связей, аудит, резервное копирование.

## Технологический стек

- **Языки:** Rust (бэкенд), TypeScript (фронтенд)
- **Фреймворк:** Tauri v2, React 19
- **База данных:** SQLite (rusqlite, bundled)
- **Сборка:** Vite 7
- **Аутентификация:** argon2

## Структура проекта

```
casegraph/
├── src/                              # Фронтенд (React + TypeScript)
│   ├── main.tsx                      # Входная точка React
│   ├── App.tsx                       # Управление состоянием (BootstrapState: loading → ... → authenticated)
│   ├── app/                          # Ядро: инициализация, роутинг, стили
│   │   ├── api/appApi.ts             # initializeApp()
│   │   ├── App.tsx                   # Состояние загрузки, условный рендеринг
│   │   ├── router.tsx                # Заглушка (роутинг через enum, без библиотеки)
│   │   └── styles/globals.css        # Глобальные стили
│   ├── features/                     # Фиче-модули (api/, model/, ui/)
│   │   ├── auth/                     # Аутентификация, матрица прав
│   │   ├── cases/                    # Управление делами
│   │   ├── materials/                # Материалы и файловое хранилище
│   │   ├── objects/                  # Объекты дела
│   │   ├── relations/                # Связи между объектами
│   │   ├── graph/                    # Графовая визуализация
│   │   ├── timeline/                 # Временная шкала
│   │   ├── audit/                    # Журнал аудита
│   │   ├── backup/                   # Резервное копирование и восстановление
│   │   ├── users/                    # Управление пользователями
│   │   └── settings/                 # Настройки приложения
│   ├── pages/                        # Страницы (собирают UI из features)
│   │   ├── cases/                    # CasesPage, CreateCaseModal
│   │   ├── case-workspace/           # CaseWorkspacePage, сайдбар, вкладки (объекты, шкала, связи, граф)
│   │   ├── login/                    # LoginPage
│   │   ├── first-admin/              # FirstAdminSetupPage
│   │   ├── change-password/          # ChangePasswordPage
│   │   ├── audit-log/                # AuditLogPage
│   │   ├── backup/                   # BackupPage
│   │   ├── materials/                # MaterialsPage
│   │   └── settings/                 # SettingsPage
│   └── shared/                       # Переиспользуемый код
│       ├── api/invoke.ts             # Обёртка Tauri invoke → CommandResult → AppCommandError
│       ├── lib/                      # can(), formatError()
│       ├── security/                 # Реестр protectedOperations
│       └── ui/ConfirmModal.tsx        # Общее модальное окно подтверждения
│
├── src-tauri/                        # Бэкенд (Rust + Tauri)
│   ├── Cargo.toml                    # Зависимости: tauri, rusqlite, argon2, uuid, zip, chrono, ...
│   └── src/
│       ├── main.rs                   # fn main() → lib::run()
│       ├── lib.rs                    # Регистрация всех Tauri-команд (45+ команд)
│       ├── commands/                 # Tauri-команды (тонкий слой сериализации)
│       │   ├── app_commands.rs       # initialize_app
│       │   ├── auth_commands.rs      # login, logout, create_first_admin, get_current_user
│       │   ├── case_commands.rs      # CRUD дел
│       │   ├── material_commands.rs  # CRUD материалов
│       │   ├── object_commands.rs    # CRUD объектов + link_to_materials
│       │   ├── relation_commands.rs  # CRUD связей
│       │   ├── graph_commands.rs     # get_graph_data
│       │   ├── timeline_commands.rs  # CRUD событий + toggle_report_include
│       │   ├── audit_commands.rs     # Чтение и экспорт аудит-лога
│       │   ├── user_management_commands.rs  # Управление пользователями
│       │   ├── settings_commands.rs  # CRUD настроек + выбор директории
│       │   ├── backup_commands.rs    # Создание, проверка, восстановление бэкапов
│       │   ├── restore_recovery_commands.rs  # Выход из режима восстановления
│       │   └── security_commands.rs  # Матрица прав
│       ├── services/                 # Бизнес-логика + валидация + guard'ы
│       ├── repositories/             # Доступ к SQLite (rusqlite)
│       ├── domain/                   # DTO, Payload, Response, константы
│       ├── db/                       # connection.rs, migrations.rs
│       ├── security/                 # session.rs, password.rs, авторизация (ProtectedOperation, PolicyAwarePermission*)
│       ├── audit/                    # Запись и чтение audit_logs (best-effort)
│       ├── backup/                   # ZIP-архивы, проверка, восстановление
│       ├── models/                   # AppSettingsDto, SettingsCatalog
│       ├── storage/                  # Файловое хранилище материалов
│       └── errors/                   # AppErrorDto, CommandResult<T>
```

## Ключевые точки входа

| Файл | Назначение |
|------|-----------|
| `src/main.tsx` | Входная точка React-приложения |
| `src/App.tsx` | Управление глобальным состоянием (BootstrapState), условный рендеринг |
| `src/shared/api/invoke.ts` | Обёртка Tauri invoke — разворачивает CommandResult, выбрасывает AppCommandError |
| `src-tauri/src/main.rs` | Точка входа Rust, вызывает `lib::run()` |
| `src-tauri/src/lib.rs` | Регистрация плагинов, SessionState, invoke_handler со всеми командами |
| `src-tauri/src/db/migrations.rs` | SQL-миграции (CREATE TABLE IF NOT EXISTS + seed-данные) |
| `src-tauri/src/security/session.rs` | In-memory сессия пользователя |
| `src-tauri/src/errors/app_error.rs` | AppErrorDto и CommandResult<T> |

## Документация

| Документ | Путь | Описание |
|----------|------|----------|
| README | README.md | Лендинг проекта |
| Быстрый старт | docs/getting-started.md | Установка, первый запуск, создание администратора |
| Архитектура | docs/architecture.md | Структура проекта, слои, правила зависимостей |
| API | docs/api.md | Справочник Tauri-команд (45+) |
| Настройки | docs/configuration.md | Параметры приложения, политики доступа |
| Аутентификация | docs/auth.md | Роли, права, безопасность |
| AGENTS.md | AGENTS.md | Структурная карта проекта для AI-агентов |

## AI-контекстные файлы

| Файл | Назначение |
|------|-----------|
| AGENTS.md | Структурная карта проекта для AI-агентов |
| .ai-factory/DESCRIPTION.md | Техническое описание проекта (стек, архитектура, требования) |
| .ai-factory/ARCHITECTURE.md | Архитектурные решения, организация папок, правила зависимостей |
| .ai-factory/config.yaml | Конфигурация AI Factory (языки, пути, git, workflow) |

## Правила для агентов

- Команды оболочки должны разбиваться на атомарные шаги — не объединять `git checkout` и `git pull` в одну строку
  - Неправильно: `git checkout main && git pull`
  - Правильно: сначала `git checkout main`, затем `git pull origin main`
