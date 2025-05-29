// src/shared/store/slices/auth-slice.ts
import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import { RootState } from '../index';

interface AuthState {
    isAuth: boolean;
    accessToken: string | null;
    refreshToken: string | null;
    expiresIn: number | null;
}

const initialState: AuthState = {
    isAuth: false,
    accessToken: null,
    refreshToken: null,
    expiresIn: null,
};

export const authSlice = createSlice({
    name: 'auth',
    initialState,
    reducers: {
        setCredentials: (
            state,
            action: PayloadAction<{
                access_token: string;
                token_type: string;
                expires_in: number;
            }>,
        ) => {
            state.accessToken = action.payload.access_token;
            state.expiresIn = action.payload.expires_in;
            state.isAuth = true;
        },
    },
});

export const { setCredentials } = authSlice.actions;

export const selectIsAuth = (state: RootState) => state.auth.isAuth;
export const selectAccessToken = (state: RootState) => state.auth.accessToken;

export default authSlice.reducer;
