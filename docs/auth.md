[← Настройки](configuration.md) · [← Назад к README](../README.md)

# Аутентификация

## Обзор

Аутентификация на основе пароля с хэшированием argon2. In-memory сессия (без JWT/токенов, так как приложение офлайн). Три роли с разграничением прав.

## Схема входа

```
LoginPage → login(username, password) → argon2::verify_password()
    → проверка is_active == 1
    → проверка must_change_password
    → установка SessionState (in-memory)
    → возврат CurrentUserDto
```

## Роли

| Роль | Права по умолчанию |
|------|-------------------|
| `administrator` | Полный доступ: управление пользователями, настройками, бэкапами, аудит |
| `analyst` | Рабочий доступ: CRUD дел, объектов, связей, шкалы. Может создавать бэкапы (если политика разрешает) |
| `viewer` | Только чтение: просмотр дел, объектов, графа. Может экспортировать (если политика разрешает) |

Роли задаются при создании пользователя и хранятся в таблице `roles`.

## Уровни авторизации

### 1. ProtectedServiceContext::require_operation()

Проверяет:
- Не активен ли restore recovery
- Наличие активного пользователя в сессии
- Флаг `must_change_password`
- Права через PolicyAwarePermissionGuard

### 2. PolicyAwarePermissionGuard::require()

Запрашивает PolicyAwarePermissionService и при отказе пишет аудит `ACCESS_DENIED`.

### 3. PolicyAwarePermissionService::decide()

Проверяет:
- Роль пользователя
- Политики доступа из `app_settings`
- Возвращает `PermissionDecision::Allow` или `::Deny`

## Must Change Password

Если у пользователя установлен флаг `must_change_password`:

- Все операции (кроме `change_own_password`) возвращают `ERR_PASSWORD_CHANGE_REQUIRED`
- Фронтенд перехватывает ошибку и показывает `ChangePasswordPage`
- После смены пароля флаг снимается, права восстанавливаются

## Безопасность паролей

```rust
// Хэширование
use argon2::{Argon2, PasswordHasher};
use password_hash::SaltString;

let salt = SaltString::generate(&mut OsRng);
let hash = Argon2::default()
    .hash_password(password.as_bytes(), &salt)
    .unwrap()
    .to_string();

// Проверка
let verified = Argon2::default()
    .verify_password(password.as_bytes(), &parsed_hash)
    .is_ok();
```

## Создание пользователей

- `create_first_admin` — без авторизации, только если нет пользователей
- `create_user` — требует прав `administrator`

## Блокировка

- `block_user` / `unblock_user` — администратор может заблокировать/разблокировать пользователя
- Заблокированный пользователь (`is_active == 0`) не может войти

## См. также

- [Настройки](configuration.md) — политики доступа
- [API](api.md) — справочник команд
- [Быстрый старт](getting-started.md) — создание первого администратора
