import * as path from 'path';
import { formatFiles, generateFiles } from '@nx/devkit';
import { camelCase, pascalCase } from 'change-case';
import { componentGenerator as nxComponentGenerator } from '@nx/react';
import type { Tree } from '@nx/devkit';
import type { Schema as BaseSchema } from '@nx/react/src/generators/component/schema';

interface Schema extends BaseSchema {
    async: boolean;
    layer: string;
    app: string;
}

export async function componentGenerator(tree: Tree, options: Schema) {
    await nxComponentGenerator(tree, {
        ...options,
        style: 'css',
        nameAndDirectoryFormat: 'as-provided',
    });

    const reference = options.async ? 'async' : 'component';

    generateFiles(
        tree,
        path.join(__dirname, 'files', reference),
        options.directory,
        {
            ...options,
            app: options.app || 'unknown',
            layer: options.layer || 'unknown',
            pascalCase,
            camelCase,
        },
    );
    await formatFiles(tree);
}

export default componentGenerator;
