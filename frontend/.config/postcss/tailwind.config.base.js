const { createGlobPatternsForDependencies } = require('@nx/react/tailwind');
const path = require('path');

const ROOT = path.join(__dirname, '..', '..');
const PATH = {
    ROOT,
    APPS: path.join(ROOT, 'apps'),
};

const APPS = ['app', 'storybook'];

const appsContent = APPS.map((app) =>
    createGlobPatternsForDependencies(path.join(PATH.APPS, app)),
).flat();

function getScreens(breakpoints) {
    const keys = Object.keys(breakpoints);
    const min = keys.map((size) => [`min-${size}`, { min: breakpoints[size] }]);
    const max = keys.map((size) => [`max-${size}`, { max: breakpoints[size] }]);
    return Object.fromEntries([...min, ...max]);
}

const breakpoints = {
    '4xl': '2700px',
    '3xl': '1920px',
    '2xl': '1440px',
    xl: '1200px',
    lg: '992px',
    md: '768px',
    sm: '576px',
    xs: '420px',
};

/** @type {import('tailwindcss').Config} */
module.exports = {
    darkMode: 'class',
    content: appsContent,
    corePlugins: { preflight: false },
    theme: {
        extend: {
            fontFamily: ['Inter', 'sans-serif'],
            screens: getScreens(breakpoints),
            width: breakpoints,
        },
    },
    plugins: [],
};
