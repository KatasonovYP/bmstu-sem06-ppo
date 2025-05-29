import type { ConfigFile } from '@rtk-query/codegen-openapi';

const config: ConfigFile = {
    schemaFile: 'http://0.0.0.0:3000/api-docs/openapi.json',
    apiFile: '../../libs/general-layers/src/shared/store/base-api.ts',
    apiImport: 'baseApi',
    outputFile: '../../libs/general-layers/src/shared/store/api.ts',
    exportName: 'stocksTracker',
    hooks: {
        queries: true,
        lazyQueries: true,
        mutations: true,
    },
    prettierConfigFile: '../../.prettierrc',
};

export default config;
