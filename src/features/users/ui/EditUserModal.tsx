import { useEffect, useState } from "react";

import { getUserById, updateUser } from "../api/usersApi";
import type {
  RoleOptionDto,
  UserListItemDto,
  UserRole,
} from "../model/userTypes";

type EditUserModalProps = {
  userId: string;
  roles: RoleOptionDto[];
  onClose: () => void;
  onSaved: (user: UserListItemDto) => void;
};

type EditUserForm = {
  displayName: string;
  roleCode: UserRole;
  mustChangePassword: boolean;
};

export function EditUserModal({
  userId,
  roles,
  onClose,
  onSaved,
}: EditUserModalProps) {
  const [user, setUser] = useState<UserListItemDto | null>(null);
  const [form, setForm] = useState<EditUserForm | null>(null);
  const [loadState, setLoadState] = useState<"loading" | "ready" | "error">(
    "loading",
  );
  const [isSaving, setIsSaving] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  useEffect(() => {
    let isCancelled = false;

    async function loadUser() {
      setLoadState("loading");
      setErrorMessage(null);

      try {
        const response = await getUserById({ userId });

        if (isCancelled) {
          return;
        }

        setUser(response.user);
        setForm({
          displayName: response.user.displayName ?? "",
          roleCode: response.user.roleCode,
          mustChangePassword: response.user.mustChangePassword,
        });
        setLoadState("ready");
      } catch (error) {
        if (isCancelled) {
          return;
        }

        setErrorMessage(
          error instanceof Error
            ? error.message
            : "Не удалось загрузить пользователя",
        );
        setLoadState("error");
      }
    }

    void loadUser();

    return () => {
      isCancelled = true;
    };
  }, [userId]);

  function updateForm<K extends keyof EditUserForm>(
    key: K,
    value: EditUserForm[K],
  ) {
    setForm((current) => {
      if (!current) {
        return current;
      }

      return {
        ...current,
        [key]: value,
      };
    });
  }

  function validateForm(): string | null {
    if (!form) {
      return "Форма ещё не загружена";
    }

    const displayName = form.displayName.trim();

    if (displayName && displayName.length < 2) {
      return "Имя пользователя должно содержать минимум 2 символа";
    }

    return null;
  }

  async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();

    if (!form || !user) {
      return;
    }

    const validationError = validateForm();

    if (validationError) {
      setErrorMessage(validationError);
      return;
    }

    setIsSaving(true);
    setErrorMessage(null);

    try {
      const response = await updateUser({
        userId: user.id,
        displayName: form.displayName.trim() || undefined,
        roleCode: form.roleCode,
        mustChangePassword: form.mustChangePassword,
      });

      onSaved(response.user);
      onClose();
    } catch (error) {
      setErrorMessage(
        error instanceof Error
          ? error.message
          : "Не удалось сохранить пользователя",
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
            <h2>Редактировать пользователя</h2>
            <p>Изменение локальной учётной записи.</p>
          </div>

          <button type="button" onClick={onClose} disabled={isSaving}>
            ×
          </button>
        </header>

        {loadState === "loading" && (
          <div className="modal-body">Загрузка пользователя…</div>
        )}

        {loadState === "error" && (
          <div className="modal-body">
            <div role="alert" className="error-state">
              {errorMessage ?? "Ошибка загрузки"}
            </div>

            <footer className="modal-footer">
              <button type="button" onClick={onClose}>
                Закрыть
              </button>
            </footer>
          </div>
        )}

        {loadState === "ready" && user && form && (
          <form onSubmit={handleSubmit} className="modal-body">
            {errorMessage && (
              <div role="alert" className="error-state">
                {errorMessage}
              </div>
            )}

            <div className="readonly-field">
              <span>Логин</span>
              <strong>{user.username}</strong>
            </div>

            <label>
              Имя
              <input
                value={form.displayName}
                onChange={(event) =>
                  updateForm("displayName", event.target.value)
                }
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

            <div className="readonly-field">
              <span>Статус</span>
              <strong>{user.isActive ? "Активен" : "Заблокирован"}</strong>
            </div>

            <footer className="modal-footer">
              <button type="button" onClick={onClose} disabled={isSaving}>
                Отмена
              </button>

              <button type="submit" disabled={isSaving}>
                {isSaving ? "Сохранение…" : "Сохранить"}
              </button>
            </footer>
          </form>
        )}
      </div>
    </div>
  );
}
