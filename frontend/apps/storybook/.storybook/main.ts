import type { StorybookConfig } from '@storybook/react-vite';

export default {
    stories: ['../../../libs/**/*.stories.@(js|jsx|ts|tsx|mdx)'],
    addons: [
        '@storybook/addon-essentials',
        '@storybook/addon-interactions',
        'storybook-dark-mode',
    ],
    framework: {
        name: '@storybook/react-vite',
        options: {
            builder: { viteConfigPath: 'apps/app/vite.config.mts' },
        },
    },
} satisfies StorybookConfig;
