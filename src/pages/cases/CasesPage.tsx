import type { CurrentUserDto } from "../../features/auth/model/authTypes";

type Props = {
  user: CurrentUserDto;
  onLogout: () => void;
};

export function CasesPage({ user, onLogout }: Props) {
  return (
    <main style={{ padding: 32 }}>
      <h1>Список дел</h1>

      <p>
        Пользователь: <strong>{user.displayName}</strong>
      </p>

      <p>
        Роль: <strong>{user.role}</strong>
      </p>

      <button type="button" onClick={onLogout}>
        Выйти
      </button>

      <hr />

      <p>Здесь будет CasesPage.</p>
    </main>
  );
}