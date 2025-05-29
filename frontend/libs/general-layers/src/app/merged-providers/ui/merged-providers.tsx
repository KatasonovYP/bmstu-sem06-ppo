import { StrictMode, Suspense } from 'react';
import type { PropsWithChildren } from 'react';
// import { FallbackPage } from '@/general/pages/fallback-page';
import { init, parseInitDataQuery } from '@telegram-apps/sdk-react';
import { ReduxProvider } from '@/shared/store';
import { useRawInitData, mockTelegramEnv } from '@telegram-apps/sdk-react';

export function MergedProviders(props: PropsWithChildren) {
    if (__IS_DEV__) {
        mockTelegramEnv({ launchParams: __TMA_MOCK__ });
    }
    init();
    localStorage.setItem('initDataRawStocksTracker', useRawInitData() || '');
    return (
        <StrictMode>
            <Suspense fallback={<p>fallback</p>}>
                <ReduxProvider>{props.children}</ReduxProvider>
            </Suspense>
        </StrictMode>
    );
}
