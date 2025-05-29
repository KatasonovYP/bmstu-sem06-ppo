import { combineReducers, configureStore } from '@reduxjs/toolkit';
import {
    persistStore,
    persistReducer,
    FLUSH,
    REHYDRATE,
    PAUSE,
    PERSIST,
    PURGE,
    REGISTER,
} from 'redux-persist';
import { storage } from './custom-storage';
import { baseApi } from './base-api';
import { stocksTracker } from './api';
import { authSlice } from './slices/auth-slice';

const apiPersistConfig = {
    key: 'api',
    storage: storage,
    whitelist: [],
};

const authPersistConfig = {
    key: 'auth',
    storage: storage,
    whitelist: ['isAuth', 'accessToken', 'expiresIn'],
};

const rootReducer = combineReducers({
    // [baseApi.reducerPath]: baseApi.reducer,
    [stocksTracker.reducerPath]: persistReducer(
        apiPersistConfig,
        stocksTracker.reducer,
    ),
    [authSlice.name]: persistReducer(authPersistConfig, authSlice.reducer),
});

export const store = configureStore({
    reducer: rootReducer,
    middleware: (getDefaultMiddleware) =>
        getDefaultMiddleware({
            serializableCheck: {
                ignoredActions: [
                    FLUSH,
                    REHYDRATE,
                    PAUSE,
                    PERSIST,
                    PURGE,
                    REGISTER,
                ],
            },
        }).concat(baseApi.middleware, stocksTracker.middleware),
});

export const persistor = persistStore(store);

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
