import type { UserListItemDto } from "../model/userTypes";

type UserActionsCellProps = {
  user: UserListItemDto;
  isBusy: boolean;
  onEdit: (userId: string) => void;
  onBlock: (user: UserListItemDto) => void;
  onUnblock: (user: UserListItemDto) => void;
};

export function UserActionsCell({
  user,
  isBusy,
  onEdit,
  onBlock,
  onUnblock,
}: UserActionsCellProps) {
  return (
    <div className="table-actions">
      <button type="button" onClick={() => onEdit(user.id)} disabled={isBusy}>
        Редактировать
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
