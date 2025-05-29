import { render } from '@testing-library/react';
import { expect, describe, it } from 'vitest';

import HomePage from './home-page';
import React from 'react';

describe('home-page', () => {
    it('should render successfully', () => {
        const { baseElement } = render(<HomePage />);
        expect(baseElement).toBeTruthy();
    });
});
