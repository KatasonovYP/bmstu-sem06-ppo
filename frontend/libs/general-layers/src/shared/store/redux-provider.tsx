import { Provider } from 'react-redux';
import { persistStore } from 'redux-persist';
import { PersistGate } from 'redux-persist/integration/react';
import { persistor, store } from './store';
import type { PropsWithChildren } from 'react';

persistStore(store); // persist the store

export function ReduxProvider({ children }: PropsWithChildren) {
    return (
        <Provider store={store}>
            <PersistGate
                loading={null}
                persistor={persistor}
            >
                {children}
            </PersistGate>
        </Provider>
    );
}
