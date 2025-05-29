import { useMatches } from 'react-router-dom';
// import type { BackgroundProps } from '@/shared/ui/background';

export function useBackgroundPropsByPath() {
    // props: Record<string, BackgroundProps>,
    const matches = useMatches();
    const parentPath = matches.length >= 3 ? matches[2].pathname : 'default';
    // return parentPath in props && props[parentPath];
}
