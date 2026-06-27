import { useEffect, useMemo, useState } from "react";

import type { CurrentUserDto } from "../../auth/model/authTypes";
import { formatError } from "../../../shared/lib/formatError";
import { getRoles, getUsers } from "../api/usersApi";
import { CreateUserModal } from "./CreateUserModal";
import type {
  RoleOptionDto,
  UserListItemDto,
  UserRole,
  UserStatusFilter,
} from "../model/userTypes";

const PAGE_SIZE = 50;

type LoadState = "idle" | "loading" | "ready" | "error";

type Props = {
  user: CurrentUserDto;
  onBack: () => void;
};

export function UsersPage({ user: _user, onBack }: Props) {
  const [users, setUsers] = useState<UserListItemDto[]>([]);
  const [roles, setRoles] = useState<RoleOptionDto[]>([]);
  const [total, setTotal] = useState(0);

  const [query, setQuery] = useState("");
  const [roleFilter, setRoleFilter] = useState<UserRole | "">("");
  const [statusFilter, setStatusFilter] = useState<UserStatusFilter>("all");
  const [offset, setOffset] = useState(0);

  const [loadState, setLoadState] = useState<LoadState>("idle");
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [isCreateModalOpen, setIsCreateModalOpen] = useState(false);

  const page = Math.floor(offset / PAGE_SIZE) + 1;
  const totalPages = Math.max(1, Math.ceil(total / PAGE_SIZE));

  const payload = useMemo(
    () => ({
      query: query.trim() || undefined,
      role: roleFilter || undefined,
      status: statusFilter,
      limit: PAGE_SIZE,
      offset,
    }),
    [query, roleFilter, statusFilter, offset],
  );

  async function loadUsers() {
    setLoadState("loading");
    setErrorMessage(null);

    try {
      const [usersResponse, rolesResponse] = await Promise.all([
        getUsers(payload),
        getRoles(),
      ]);

      setUsers(usersResponse.users);
      setTotal(usersResponse.total);
      setRoles(rolesResponse.roles);
      setLoadState("ready");
    } catch (error) {
      setLoadState("error");
      setErrorMessage(formatError(error));
    }
  }

  useEffect(() => {
    void loadUsers();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [payload]);

  function resetFilters() {
    setQuery("");
    setRoleFilter("");
    setStatusFilter("all");
    setOffset(0);
  }

  function goPrevPage() {
    setOffset((current) => Math.max(0, current - PAGE_SIZE));
  }

  function goNextPage() {
    setOffset((current) => {
      const next = current + PAGE_SIZE;
      return next >= total ? current : next;
    });
  }

  const roleFilterValue = roleFilter;

  async function handleUserCreated() {
    setIsCreateModalOpen(false);
    await loadUsers();
  }

  return (
    <main className="page" style={{ padding: 32 }}>
      <header className="page-header">
        <div>
          <h1>Пользователи</h1>
          <p>Локальные пользователи CaseGraph и их роли.</p>
        </div>

        <div style={{ display: "flex", gap: 8 }}>
          <button type="button" onClick={() => setIsCreateModalOpen(true)}>
            Создать пользователя
          </button>
          <button type="button" onClick={onBack}>
            Назад к делам
          </button>
        </div>
      </header>

      <div className="toolbar">
        <label>
          Поиск
          <input
            value={query}
            onChange={(event) => {
              setQuery(event.target.value);
              setOffset(0);
            }}
            placeholder="Логин или имя"
          />
        </label>

        <label>
          Роль
          <select
            value={roleFilterValue}
            onChange={(event) => {
              setRoleFilter(event.target.value as UserRole | "");
              setOffset(0);
            }}
          >
            <option value="">Все роли</option>
            {roles.map((roleOption) => (
              <option key={roleOption.id} value={roleOption.roleCode}>
                {roleOption.title}
              </option>
            ))}
          </select>
        </label>

        <label>
          Статус
          <select
            value={statusFilter}
            onChange={(event) => {
              setStatusFilter(event.target.value as UserStatusFilter);
              setOffset(0);
            }}
          >
            <option value="all">Все</option>
            <option value="active">Активные</option>
            <option value="blocked">Заблокированные</option>
          </select>
        </label>

        <button type="button" onClick={resetFilters}>
          Сбросить
        </button>

        <button type="button" onClick={() => void loadUsers()}>
          Обновить
        </button>
      </div>

      {loadState === "loading" && <p>Загрузка пользователей…</p>}

      {loadState === "error" && (
        <div role="alert" className="error-state">
          {errorMessage}
        </div>
      )}

      {loadState === "ready" && users.length === 0 && (
        <div className="empty-state">Пользователи не найдены.</div>
      )}

      {loadState === "ready" && users.length > 0 && (
        <>
          <table className="data-table" border={1} cellPadding={8} style={{ borderCollapse: "collapse" }}>
            <thead>
              <tr>
                <th>Логин</th>
                <th>Имя</th>
                <th>Роль</th>
                <th>Статус</th>
                <th>Смена пароля</th>
                <th>Последний вход</th>
                <th>Создан</th>
              </tr>
            </thead>

            <tbody>
              {users.map((userItem) => (
                <tr key={userItem.id}>
                  <td>{userItem.username}</td>
                  <td>{userItem.displayName || "—"}</td>
                  <td>{userItem.roleTitle}</td>
                  <td>
                    <UserStatusBadge isActive={userItem.isActive} />
                  </td>
                  <td>{userItem.mustChangePassword ? "Требуется" : "Нет"}</td>
                  <td>{formatDateTime(userItem.lastLoginAt)}</td>
                  <td>{formatDateTime(userItem.createdAt)}</td>
                </tr>
              ))}
            </tbody>
          </table>

          <footer className="pagination">
            <button type="button" onClick={goPrevPage} disabled={offset === 0}>
              Назад
            </button>

            <span>
              Страница {page} из {totalPages} · всего {total}
            </span>

            <button
              type="button"
              onClick={goNextPage}
              disabled={offset + PAGE_SIZE >= total}
            >
              Вперёд
            </button>
          </footer>
        </>
      )}
      {isCreateModalOpen && (
        <CreateUserModal
          roles={roles}
          onClose={() => setIsCreateModalOpen(false)}
          onCreated={handleUserCreated}
        />
      )}
    </main>
  );
}

function UserStatusBadge({ isActive }: { isActive: boolean }) {
  return (
    <span
      style={{
        color: isActive ? "green" : "crimson",
        fontWeight: "bold",
      }}
    >
      {isActive ? "Активен" : "Заблокирован"}
    </span>
  );
}

function formatDateTime(value?: string | null) {
  if (!value) {
    return "—";
  }

  return value;
}
