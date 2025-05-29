import type { Meta, StoryObj } from '@storybook/react';

import SettingsPage from './settings-page';

const meta: Meta<typeof SettingsPage> = {
    title: 'general/pages/settings-page',
    component: SettingsPage,
    tags: ['autodocs'],
};

export default meta;
type Story = StoryObj<typeof SettingsPage>;

export const Primary: Story = {
    args: {},
};
