import type { ReactNode } from 'react';
import { ReduxProvider } from '@/shared/store';

export function ReduxDecorator(Story: () => ReactNode) {
    return (
        <ReduxProvider>
            <Story />
        </ReduxProvider>
    );
}
