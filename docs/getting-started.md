[← Назад к README](../README.md) · [Архитектура →](architecture.md)

# Быстрый старт

## Требования

- **Node.js** 18+
- **Rust** (устанавливается через [rustup](https://rustup.rs))
- **VS Code** (рекомендуется) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

Для Windows потребуются [Microsoft Visual C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/).

## Установка

```bash
# Клонирование
git clone <repo-url>
cd casegraph

# Установка зависимостей
npm install
```

## Первый запуск

```bash
npm run tauri dev
```

При первом запуске миграции БД выполняются автоматически. Приложение открывает окно логина. Если пользователей нет — форму создания первого администратора.

## Создание первого администратора

1. При первом запуске открывается страница **Настройка администратора**
2. Заполните имя пользователя, пароль и отображаемое имя
3. Нажмите «Создать администратора»
4. После создания — вход с теми же учётными данными

## Проверка

После входа должна открыться страница со списком дел (пустая при первом запуске). Для проверки:

```bash
npm run typecheck    # Проверка типов TypeScript
npm run build        # Сборка фронтенда
```

## Следующие шаги

- [Архитектура](architecture.md) — понять структуру проекта
- [API](api.md) — справочник Tauri-команд
- [Аутентификация](auth.md) — роли и права доступа

## См. также

- [Архитектура](architecture.md) — структура и слои проекта
- [Настройки](configuration.md) — параметры приложения
- [Аутентификация](auth.md) — безопасность и авторизация
