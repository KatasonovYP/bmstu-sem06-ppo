import * as path from 'path';
import { componentGenerator } from '../component/generator';
import type { Tree } from '@nx/devkit';
import type { ComponentGeneratorSchema } from './schema';

const pathToLayers = (app: string) => ['libs', `${app}-layers`, 'src'];

export async function sliceGenerator(
    tree: Tree,
    options: ComponentGeneratorSchema,
) {
    const directory = path.join(
        ...pathToLayers(options.app),
        options.layer,
        options.layer === 'shared' ? 'ui' : '.',
        options.name,
    );
    await componentGenerator(tree, {
        ...options,
        directory,
    });
}

export default sliceGenerator;
