import { FormEvent, useState } from "react";
import { createFirstAdmin } from "../../features/auth/api/authApi";
import type { AppCommandError } from "../../shared/api/commandResult";

type Props = {
  onCreated: () => void;
};

export function FirstAdminSetupPage({ onCreated }: Props) {
  const [username, setUsername] = useState("admin");
  const [displayName, setDisplayName] = useState("Администратор");
  const [password, setPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    setError(null);

    if (password !== confirmPassword) {
      setError("Пароли не совпадают.");
      return;
    }

    try {
      setSubmitting(true);

      await createFirstAdmin({
        username,
        displayName,
        password,
      });

      onCreated();
    } catch (err) {
      const appError = err as AppCommandError;
      setError(appError.message || "Не удалось создать администратора.");
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <main style={{ padding: 32, maxWidth: 420 }}>
      <h1>Первичная настройка CaseGraph</h1>
      <p>Создайте первого локального администратора.</p>

      <form onSubmit={handleSubmit}>
        <div>
          <label>
            Логин
            <input
              value={username}
              onChange={(event) => setUsername(event.target.value)}
              minLength={3}
              required
            />
          </label>
        </div>

        <div>
          <label>
            Имя
            <input
              value={displayName}
              onChange={(event) => setDisplayName(event.target.value)}
              minLength={2}
              required
            />
          </label>
        </div>

        <div>
          <label>
            Пароль
            <input
              type="password"
              value={password}
              onChange={(event) => setPassword(event.target.value)}
              minLength={8}
              required
            />
          </label>
        </div>

        <div>
          <label>
            Повтор пароля
            <input
              type="password"
              value={confirmPassword}
              onChange={(event) => setConfirmPassword(event.target.value)}
              minLength={8}
              required
            />
          </label>
        </div>

        {error && (
          <p style={{ color: "crimson" }}>
            {error}
          </p>
        )}

        <button type="submit" disabled={submitting}>
          {submitting ? "Создание..." : "Создать администратора"}
        </button>
      </form>
    </main>
  );
}