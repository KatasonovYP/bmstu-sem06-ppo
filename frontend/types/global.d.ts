/// <reference types='vitest' />
/// <reference types='vite-plugin-svgr/client' />
declare const __IS_DEV__: boolean;
declare const __BASE_PATH__: string;
declare const __API__: string;
declare const __TMA_MOCK__: string;
declare module '*.md?raw' {
    const value: string;
    export default value;
}
