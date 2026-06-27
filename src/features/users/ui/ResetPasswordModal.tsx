import { useState, type FormEvent } from "react";

import { resetUserPassword } from "../api/usersApi";
import type { UserListItemDto } from "../model/userTypes";

type ResetPasswordModalProps = {
  user: UserListItemDto;
  onClose: () => void;
  onSaved: (user: UserListItemDto) => void;
};

export function ResetPasswordModal({
  user,
  onClose,
  onSaved,
}: ResetPasswordModalProps) {
  const [temporaryPassword, setTemporaryPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [isSaving, setIsSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();

    const normalizedPassword = temporaryPassword.trim();

    if (normalizedPassword.length < 8) {
      setError("Пароль должен содержать минимум 8 символов");
      return;
    }

    if (normalizedPassword !== confirmPassword.trim()) {
      setError("Пароли не совпадают");
      return;
    }

    setIsSaving(true);
    setError(null);

    try {
      const response = await resetUserPassword({
        userId: user.id,
        temporaryPassword: normalizedPassword,
      });

      onSaved(response.user);
      onClose();
    } catch (caughtError) {
      setError(
        caughtError instanceof Error
          ? caughtError.message
          : "Не удалось сбросить пароль",
      );
    } finally {
      setIsSaving(false);
    }
  }

  return (
    <div className="modal-backdrop">
      <div className="modal" role="dialog" aria-modal="true">
        <div className="modal-header">
          <h2>Сбросить пароль</h2>
          <button type="button" onClick={onClose} disabled={isSaving}>
            ×
          </button>
        </div>

        <form onSubmit={handleSubmit}>
          <div className="readonly-field">
            <span>Пользователь</span>
            <strong>{user.username}</strong>
          </div>

          <label>
            Временный пароль
            <input
              type="password"
              value={temporaryPassword}
              onChange={(event) => setTemporaryPassword(event.target.value)}
              disabled={isSaving}
              autoFocus
            />
          </label>

          <label>
            Повтор пароля
            <input
              type="password"
              value={confirmPassword}
              onChange={(event) => setConfirmPassword(event.target.value)}
              disabled={isSaving}
            />
          </label>

          <p className="form-hint">
            После сброса пользователю будет выставлен флаг обязательной смены
            пароля.
          </p>

          {error && (
            <div role="alert" className="error-state">
              {error}
            </div>
          )}

          <div className="modal-actions">
            <button type="button" onClick={onClose} disabled={isSaving}>
              Отмена
            </button>

            <button type="submit" disabled={isSaving}>
              {isSaving ? "Сохранение..." : "Сбросить пароль"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
