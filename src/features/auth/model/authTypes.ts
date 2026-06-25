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