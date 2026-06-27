import { useState } from "react";

import { createUser } from "../api/usersApi";
import type {
  CreateUserPayload,
  RoleOptionDto,
  UserRole,
} from "../model/userTypes";

type CreateUserModalProps = {
  roles: RoleOptionDto[];
  onClose: () => void;
  onCreated: () => void;
};

type CreateUserForm = {
  username: string;
  displayName: string;
  roleCode: UserRole;
  password: string;
  confirmPassword: string;
  mustChangePassword: boolean;
};

const defaultForm: CreateUserForm = {
  username: "",
  displayName: "",
  roleCode: "analyst",
  password: "",
  confirmPassword: "",
  mustChangePassword: true,
};

export function CreateUserModal({
  roles,
  onClose,
  onCreated,
}: CreateUserModalProps) {
  const [form, setForm] = useState<CreateUserForm>(defaultForm);
  const [isSaving, setIsSaving] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  function updateForm<K extends keyof CreateUserForm>(
    key: K,
    value: CreateUserForm[K],
  ) {
    setForm((current) => ({
      ...current,
      [key]: value,
    }));
  }

  function validateForm(): string | null {
    const username = form.username.trim();

    if (username.length < 3) {
      return "Логин должен содержать минимум 3 символа";
    }

    if (form.displayName.trim() && form.displayName.trim().length < 2) {
      return "Имя пользователя должно содержать минимум 2 символа";
    }

    if (form.password.length < 8) {
      return "Пароль должен содержать минимум 8 символов";
    }

    if (form.password !== form.confirmPassword) {
      return "Пароль и подтверждение не совпадают";
    }

    return null;
  }

  async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();

    const validationError = validateForm();

    if (validationError) {
      setErrorMessage(validationError);
      return;
    }

    const payload: CreateUserPayload = {
      username: form.username.trim(),
      displayName: form.displayName.trim() || undefined,
      roleCode: form.roleCode,
      password: form.password,
      mustChangePassword: form.mustChangePassword,
    };

    setIsSaving(true);
    setErrorMessage(null);

    try {
      await createUser(payload);
      onCreated();
      onClose();
    } catch (error) {
      setErrorMessage(
        error instanceof Error
          ? error.message
          : "Не удалось создать пользователя",
      );
    } finally {
      setIsSaving(false);
    }
  }

  return (
    <div className="modal-backdrop" role="presentation">
      <div className="modal" role="dialog" aria-modal="true">
        <header className="modal-header">
          <div>
            <h2>Создать пользователя</h2>
            <p>Локальная учётная запись для работы в CaseGraph.</p>
          </div>

          <button type="button" onClick={onClose} disabled={isSaving}>
            ×
          </button>
        </header>

        <form onSubmit={handleSubmit} className="modal-body">
          {errorMessage && (
            <div role="alert" className="error-state">
              {errorMessage}
            </div>
          )}

          <label>
            Логин
            <input
              value={form.username}
              onChange={(event) => updateForm("username", event.target.value)}
              placeholder="ivanov"
              autoFocus
            />
          </label>

          <label>
            Имя
            <input
              value={form.displayName}
              onChange={(event) => updateForm("displayName", event.target.value)}
              placeholder="Иванов И.И."
            />
          </label>

          <label>
            Роль
            <select
              value={form.roleCode}
              onChange={(event) =>
                updateForm("roleCode", event.target.value as UserRole)
              }
            >
              {roles.map((role) => (
                <option key={role.id} value={role.roleCode}>
                  {role.title}
                </option>
              ))}
            </select>
          </label>

          <label>
            Временный пароль
            <input
              type="password"
              value={form.password}
              onChange={(event) => updateForm("password", event.target.value)}
              placeholder="Минимум 8 символов"
            />
          </label>

          <label>
            Повтор пароля
            <input
              type="password"
              value={form.confirmPassword}
              onChange={(event) =>
                updateForm("confirmPassword", event.target.value)
              }
            />
          </label>

          <label className="checkbox-row">
            <input
              type="checkbox"
              checked={form.mustChangePassword}
              onChange={(event) =>
                updateForm("mustChangePassword", event.target.checked)
              }
            />
            Требовать смену пароля при следующем входе
          </label>

          <footer className="modal-footer">
            <button type="button" onClick={onClose} disabled={isSaving}>
              Отмена
            </button>

            <button type="submit" disabled={isSaving}>
              {isSaving ? "Создание…" : "Создать"}
            </button>
          </footer>
        </form>
      </div>
    </div>
  );
}
