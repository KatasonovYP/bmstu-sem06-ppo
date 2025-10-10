import cn from 'classnames';
import cls from './docs-page.module.css';
import Markdown from 'react-markdown';
import docsFile from '../../../../../../README.md?raw';

interface DocsPageProps {
    className?: string;
}

export default function DocsPage(props: DocsPageProps) {
    const { className } = props;
    const markdown = docsFile
        .replaceAll(
            './docs/images',
            'https://github.com/KatasonovYP/bmstu-sem06-ppo/blob/trunk/docs/images',
        )
        .replaceAll('ng', 'ng?raw=true');
    console.log(markdown);
    return (
        <div className={cn(className, cls.docsPage)}>
            <Markdown>{markdown}</Markdown>
        </div>
    );
}
