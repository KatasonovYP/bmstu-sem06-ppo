import { render } from '@testing-library/react';
import { expect } from 'vitest';

import ActivePage from './active-page';

describe('active-page', () => {
    it('should render successfully', () => {
        const { baseElement } = render(<ActivePage />);
        expect(baseElement).toBeTruthy();
    });
});
