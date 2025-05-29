export interface LoginRequest {}

export interface LoginResponse {
    accessToken: string;
    refreshToken: string;
    lifetime: string;
}
