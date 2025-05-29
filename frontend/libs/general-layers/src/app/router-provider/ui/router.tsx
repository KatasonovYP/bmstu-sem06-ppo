import { createBrowserRouter } from 'react-router-dom';
import { PublicLayout } from './public-layout';
import { staticRoutes } from '@/general/shared/config/const';
import { HomePage } from '@/general/pages/home-page';
import { SettingsPage } from '@/general/pages/settings-page';
import { ActivePage } from '@/general/pages/active-page';

export const router = createBrowserRouter([
    {
        path: '/',
        element: <PublicLayout />,
        children: [
            {
                path: staticRoutes.home,
                element: <HomePage />,
            },
            {
                path: staticRoutes.settings,
                element: <SettingsPage />,
            },
            {
                path: staticRoutes.active,
                element: <ActivePage />,
            },
        ],
    },
]);
