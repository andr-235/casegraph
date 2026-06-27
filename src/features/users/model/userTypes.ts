export type UserRole = "administrator" | "analyst" | "viewer";

export type UserStatusFilter = "all" | "active" | "blocked";

export type UserListItemDto = {
  id: string;
  username: string;
  displayName?: string | null;
  roleCode: UserRole;
  roleTitle: string;
  isActive: boolean;
  mustChangePassword: boolean;
  lastLoginAt?: string | null;
  createdAt: string;
  updatedAt: string;
};

export type RoleOptionDto = {
  id: string;
  roleCode: UserRole;
  title: string;
};

export type GetUsersPayload = {
  query?: string;
  role?: UserRole;
  status?: UserStatusFilter;
  limit?: number;
  offset?: number;
};

export type GetUsersResponse = {
  users: UserListItemDto[];
  total: number;
};

export type CreateUserPayload = {
  username: string;
  displayName?: string;
  roleCode: UserRole;
  password: string;
  mustChangePassword: boolean;
};

export type CreateUserResponse = {
  user: UserListItemDto;
};

export type GetUserByIdPayload = {
  userId: string;
};

export type GetUserByIdResponse = {
  user: UserListItemDto;
};

export type UpdateUserPayload = {
  userId: string;
  displayName?: string;
  roleCode: UserRole;
  mustChangePassword: boolean;
};

export type UpdateUserResponse = {
  user: UserListItemDto;
};

export type GetRolesResponse = {
  roles: RoleOptionDto[];
};
