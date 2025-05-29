import cn from 'classnames';
import cls from './settings-page.module.css';

interface SettingsPageProps {
    className?: string;
}

export default function SettingsPage(props: SettingsPageProps) {
    const { className } = props;

    return (
        <div className={cn(className, cls.settingsPage)}>
            <h1>settings-page</h1>
        </div>
    );
}
