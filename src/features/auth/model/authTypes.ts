export type UserRole = "administrator" | "analyst" | "viewer";

export type CurrentUserDto = {
  userId: string;
  username: string;
  displayName: string;
  role: UserRole;
  isActive: boolean;
  mustChangePassword: boolean;
};

export type CreateFirstAdminPayload = {
  username: string;
  displayName: string;
  password: string;
};

export type CreateFirstAdminResponse = {
  userId: string;
  username: string;
  role: "administrator";
};

export type LoginPayload = {
  username: string;
  password: string;
};

export type LoginResponse = {
  user: CurrentUserDto;
};