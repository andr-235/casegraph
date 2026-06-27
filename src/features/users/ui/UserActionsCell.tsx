import type { UserListItemDto } from "../model/userTypes";

type UserActionsCellProps = {
  user: UserListItemDto;
  isBusy: boolean;
  onEdit: (userId: string) => void;
  onResetPassword: (user: UserListItemDto) => void;
  onBlock: (user: UserListItemDto) => void;
  onUnblock: (user: UserListItemDto) => void;
};

export function UserActionsCell({
  user,
  isBusy,
  onEdit,
  onResetPassword,
  onBlock,
  onUnblock,
}: UserActionsCellProps) {
  return (
    <div className="table-actions">
      <button type="button" onClick={() => onEdit(user.id)} disabled={isBusy}>
        Редактировать
      </button>

      <button
        type="button"
        onClick={() => onResetPassword(user)}
        disabled={isBusy}
      >
        Сбросить пароль
      </button>

      {user.isActive ? (
        <button
          type="button"
          onClick={() => onBlock(user)}
          disabled={isBusy}
        >
          Заблокировать
        </button>
      ) : (
        <button
          type="button"
          onClick={() => onUnblock(user)}
          disabled={isBusy}
        >
          Разблокировать
        </button>
      )}
    </div>
  );
}
