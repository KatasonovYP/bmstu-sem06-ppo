import { render } from '@testing-library/react';
import { expect } from 'vitest';

import SettingsPage from './settings-page';

describe('settings-page', () => {
    it('should render successfully', () => {
        const { baseElement } = render(<SettingsPage />);
        expect(baseElement).toBeTruthy();
    });
});
