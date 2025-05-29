// src/shared/store/base-api.ts
import { createApi } from '@reduxjs/toolkit/query/react';
import { customBaseQuery } from './custom-base-query';

// Типы ответов
export interface LoginResponse {
    access_token: string;
    token_type: string;
    expires_in: number;
}

export const baseApi = createApi({
    baseQuery: customBaseQuery,
    endpoints: (build) => ({
        login: build.mutation<LoginResponse, void>({
            query: () => ({
                url: '/api/v1/auth/login',
                method: 'POST',
            }),
        }),
    }),
});

export type GetLoginApiArg = void;
export type GetLoginApiResponse = LoginResponse;
