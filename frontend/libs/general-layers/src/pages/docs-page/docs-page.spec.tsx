import { render } from '@testing-library/react';
import { expect } from 'vitest';

import DocsPage from './docs-page';

describe('docs-page', () => {
    it('should render successfully', () => {
        const { baseElement } = render(<DocsPage />);
        expect(baseElement).toBeTruthy();
    });
});
