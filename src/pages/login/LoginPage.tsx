import { FormEvent, useState } from "react";
import { login } from "../../features/auth/api/authApi";
import type { CurrentUserDto } from "../../features/auth/model/authTypes";
import type { AppCommandError } from "../../shared/api/commandResult";

type Props = {
  onLoggedIn: (user: CurrentUserDto) => void;
};

export function LoginPage({ onLoggedIn }: Props) {
  const [username, setUsername] = useState("admin");
  const [password, setPassword] = useState("");
  const [submitting, setSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function handleSubmit(event: FormEvent) {
    event.preventDefault();
    setError(null);

    try {
      setSubmitting(true);
      const response = await login({ username, password });
      onLoggedIn(response.user);
    } catch (err) {
      const appError = err as AppCommandError;
      setError(appError.message || "Не удалось выполнить вход.");
    } finally {
      setSubmitting(false);
    }
  }

  return (
    <main style={{ padding: 32, maxWidth: 420 }}>
      <h1>Вход в CaseGraph</h1>
      <p>Локальная авторизация. Интернет не требуется.</p>

      <form onSubmit={handleSubmit}>
        <div>
          <label>
            Логин
            <input
              value={username}
              onChange={(event) => setUsername(event.target.value)}
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
              required
            />
          </label>
        </div>

        {error && <p style={{ color: "crimson" }}>{error}</p>}

        <button type="submit" disabled={submitting}>
          {submitting ? "Вход..." : "Войти"}
        </button>
      </form>
    </main>
  );
}