import type { Meta, StoryObj } from '@storybook/react';

import DocsPage from './docs-page';

const meta: Meta<typeof DocsPage> = {
    title: 'general/pages/docs-page',
    component: DocsPage,
    tags: ['autodocs'],
};

export default meta;
type Story = StoryObj<typeof DocsPage>;

export const Primary: Story = {
    args: {},
};
