import { Navigate, Outlet } from 'react-router-dom';
import { staticRoutes } from '@/shared/config/const';

export function ProtectedLayout() {
    const isAuth = false;

    if (isAuth) {
        return <Outlet />;
    }

    return (
        <Navigate
            // to={staticRoutes['sign-in']}
            to={staticRoutes.home}
            replace
        />
    );
}
