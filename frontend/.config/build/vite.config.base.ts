import * as process from 'process';
import { defineConfig, UserConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { nxViteTsPaths } from '@nx/vite/plugins/nx-tsconfig-paths.plugin';
import svgr from 'vite-plugin-svgr';
import * as path from 'path';

const ROOT = path.resolve(__dirname, '..', '..');

const PATH = {
    ROOT,
    APPS: path.join(ROOT, 'apps'),
    LIBS: path.join(ROOT, 'libs'),
    SHARED: path.join(ROOT, 'libs', 'general-layers', 'src', 'shared'),
    DIST: path.join(ROOT, 'dist', 'apps'),
    NODE_MODULES: path.join(ROOT, 'node_modules'),
    CACHE: path.join(ROOT, 'node_modules', '.vite', 'apps'),
};

const envProps = {
    basePath: process.env.BASE_PATH || '/',
    api: process.env.API || 'https://api-stocks-tracker.serveo.net',
};

interface DefineBaseConfigProps extends UserConfig {
    appName: string;
    port: number;
    host?: string;
    extendDefine?: Record<string, string>;
}

export default function defineBaseConfig(props: DefineBaseConfigProps) {
    const {
        appName,
        host = 'localhost',
        port,
        extendDefine,
        ...otherProps
    } = props;
    return defineConfig(({ mode }) => ({
        base: envProps.basePath,
        root: path.join(PATH.APPS, appName),
        cacheDir: path.join(PATH.CACHE, appName),

        plugins: [react(), nxViteTsPaths(), svgr({ svgrOptions: {} })],

        define: {
            __IS_DEV__: mode === 'development',
            __BASE_PATH__: JSON.stringify(envProps.basePath),
            __API__: JSON.stringify(envProps.api),
            ...extendDefine,
        },

        server: {
            port: port,
            host,
            fs: {
                allow: [PATH.APPS, PATH.LIBS],
            },
        },

        preview: { port: port + 50, host },

        build: {
            outDir: path.join(PATH.DIST, appName),
            emptyOutDir: true,
            reportCompressedSize: true,
            commonjsOptions: { transformMixedEsModules: true },
        },

        resolve: {
            alias: [
                { find: new RegExp("@/([^']*)"), replacement: '@/$1/index.ts' },
            ],
        },

        test: {
            globals: true,
            cache: { dir: '../../node_modules/.vitest' },
            environment: 'jsdom',
            include: ['src/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'],
            reporters: ['default'],
            coverage: {
                reportsDirectory: '../../coverage/apps/app',
                provider: 'v8',
            },
        },

        ...otherProps,
    }));
}
