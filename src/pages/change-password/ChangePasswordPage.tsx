import { FormEvent, useState } from "react";
import { changeOwnPassword } from "../../features/users/api/usersApi";

type Props = {
  onPasswordChanged: () => void;
};

export function ChangePasswordPage({ onPasswordChanged }: Props) {
  const [currentPassword, setCurrentPassword] = useState("");
  const [newPassword, setNewPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [isSaving, setIsSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();

    const normalizedNewPassword = newPassword.trim();

    if (normalizedNewPassword.length < 8) {
      setError("Новый пароль должен содержать минимум 8 символов");
      return;
    }

    if (normalizedNewPassword !== confirmPassword.trim()) {
      setError("Пароли не совпадают");
      return;
    }

    setIsSaving(true);
    setError(null);

    try {
      await changeOwnPassword({
        currentPassword: currentPassword.trim(),
        newPassword: normalizedNewPassword,
      });

      onPasswordChanged();
    } catch (caughtError) {
      setError(
        caughtError instanceof Error
          ? caughtError.message
          : "Не удалось изменить пароль",
      );
    } finally {
      setIsSaving(false);
    }
  }

  return (
    <main className="auth-page">
      <section className="auth-card">
        <h1>Смена временного пароля</h1>

        <p className="form-hint">
          Администратор сбросил пароль. Перед продолжением задайте новый пароль.
        </p>

        <form onSubmit={handleSubmit}>
          <label>
            Временный пароль
            <input
              type="password"
              value={currentPassword}
              onChange={(event) => setCurrentPassword(event.target.value)}
              disabled={isSaving}
              autoFocus
            />
          </label>

          <label>
            Новый пароль
            <input
              type="password"
              value={newPassword}
              onChange={(event) => setNewPassword(event.target.value)}
              disabled={isSaving}
            />
          </label>

          <label>
            Повтор нового пароля
            <input
              type="password"
              value={confirmPassword}
              onChange={(event) => setConfirmPassword(event.target.value)}
              disabled={isSaving}
            />
          </label>

          {error && (
            <div role="alert" className="error-state">
              {error}
            </div>
          )}

          <button type="submit" disabled={isSaving}>
            {isSaving ? "Сохранение..." : "Сменить пароль"}
          </button>
        </form>
      </section>
    </main>
  );
}
