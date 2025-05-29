import ReactDOM from 'react-dom/client';
import '@shared/css';
import '@/shared/config/i18n';
import { RouterProvider } from 'react-router-dom';
import { router } from '@/general/app/router-provider';
import { MergedProviders } from '@/general/app/merged-providers';

const root = ReactDOM.createRoot(
    document.getElementById('root') as HTMLElement,
);
root.render(
    <MergedProviders>
        <RouterProvider router={router} />
    </MergedProviders>,
);
