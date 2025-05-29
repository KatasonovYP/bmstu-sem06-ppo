import type { Meta, StoryObj } from '@storybook/react';

import HomePage from './home-page';

const meta: Meta<typeof HomePage> = {
    title: 'general/pages/home-page',
    component: HomePage,
    tags: ['autodocs'],
};

export default meta;
type Story = StoryObj<typeof HomePage>;

export const Primary: Story = {
    args: {},
};
