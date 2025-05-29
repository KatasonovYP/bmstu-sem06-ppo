import * as process from 'process';
import defineBaseConfig from '../../.config/build/vite.config.base';

const props = {
    api: process.env.API || 'https://api-stocks-tracker.serveo.net',
    tma_mock: process.env.TMA_MOCK || 'not set',
};

export default defineBaseConfig({
    appName: 'app',
    port: 4400,
    extendDefine: {
        __API__: JSON.stringify(props.api),
        __TMA_MOCK__: JSON.stringify(props.tma_mock),
    },
});
