import type { Meta, StoryObj } from '@storybook/react';

import ActivePage from './active-page';

const meta: Meta<typeof ActivePage> = {
    title: 'general/pages/active-page',
    component: ActivePage,
    tags: ['autodocs'],
};

export default meta;
type Story = StoryObj<typeof ActivePage>;

export const Primary: Story = {
    args: {},
};
