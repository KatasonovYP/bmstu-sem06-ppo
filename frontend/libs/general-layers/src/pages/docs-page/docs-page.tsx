import cn from 'classnames';
import cls from './docs-page.module.css';
import Markdown from 'react-markdown';
import MarkdownPreview from '@uiw/react-markdown-preview';
import docsFile from '../../../../../../README.md?raw';

interface DocsPageProps {
    className?: string;
}

export default function DocsPage(props: DocsPageProps) {
    const { className } = props;
    const markdown = docsFile
        //@ts-ignore
        .replaceAll(
            './docs/images',
            'https://github.com/KatasonovYP/bmstu-sem06-ppo/blob/feature/add-seaorm-admin-panel/docs/images',
        )
        .replaceAll('png)', 'png?raw=true)')
        .replaceAll('jpg)', 'jpg?raw=true)');
    console.log(markdown);
    return (
        <div className={cn(className, cls.docsPage)}>
            <MarkdownPreview
                source={markdown}
                style={{ padding: 16 }}
                wrapperElement={{
                    'data-color-mode': 'light',
                }}
            />
            {/* <Markdown>{markdown}</Markdown> */}
        </div>
    );
}
