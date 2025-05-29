// src/shared/store/custom-base-query.ts
import { fetchBaseQuery } from '@reduxjs/toolkit/query';
import { Mutex } from 'async-mutex';
import type {
    BaseQueryFn,
    FetchArgs,
    FetchBaseQueryError,
} from '@reduxjs/toolkit/query';
import { RootState } from './store';
import { setCredentials } from './slices/auth-slice';

// Mutex для предотвращения множественных запросов обновления токена
const mutex = new Mutex();

// Создаем базовый запрос с токеном авторизации
const baseQueryWithAuth = fetchBaseQuery({
    baseUrl: __API__,
    prepareHeaders: (headers, { getState }) => {
        // Получаем текущий токен из Redux store
        const token = (getState() as RootState).auth.accessToken;
        if (token) {
            headers.set('Authorization', `Bearer ${token}`);
        }
        headers.set('Content-Type', 'application/json');
        return headers;
    },
});

// Создаем запрос для TMA авторизации
const baseQueryTMA = fetchBaseQuery({
    baseUrl: __API__,
});

export const customBaseQuery: BaseQueryFn<
    string | FetchArgs,
    unknown,
    FetchBaseQueryError
> = async (args, api, extraOptions) => {
    // Проверяем, заблокирован ли mutex, если да - ждем его разблокировки
    // Это предотвращает множественные запросы на обновление токена
    await mutex.waitForUnlock();
    let result = await baseQueryWithAuth(args, api, extraOptions);

    // Если получаем 401 Unauthorized, пробуем обновить токен
    if (result.error && result.error.status === 401) {
        // Проверяем, можем ли мы получить блокировку для обновления токена
        if (!mutex.isLocked()) {
            const release = await mutex.acquire();
            try {
                // Выполняем запрос login для получения нового токена
                const refreshResult = await baseQueryTMA(
                    {
                        url: `/api/v1/auth/login?${localStorage.getItem('initDataRawStocksTracker')}`,
                        method: 'POST',
                    },
                    api,
                    extraOptions,
                );

                // Если успешно получили новый токен
                if (refreshResult.data) {
                    // Сохраняем новый токен в Redux store
                    api.dispatch(setCredentials(refreshResult.data as any));

                    // Повторяем исходный запрос с новым токеном
                    result = await baseQueryWithAuth(args, api, extraOptions);
                } else {
                    // Если не удалось получить новый токен, возвращаем ошибку авторизации
                    console.error('Не удалось обновить токен');
                }
            } finally {
                // Важно всегда освобождать mutex
                release();
            }
        } else {
            // Если mutex заблокирован, ждем его разблокировки и повторяем запрос
            await mutex.waitForUnlock();
            result = await baseQueryWithAuth(args, api, extraOptions);
        }
    }

    return result;
};
