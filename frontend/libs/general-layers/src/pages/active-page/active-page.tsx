import { ChangeEvent, FormEvent, useState } from 'react';
import cn from 'classnames';
import cls from './active-page.module.css';
import { stocksTracker } from '@/shared/store';
import { Link, useParams } from 'react-router-dom';
import { staticRoutes } from '@/general/shared/config/const';

interface ActivePageProps {
    className?: string;
}

export default function ActivePage(props: ActivePageProps) {
    const { className } = props;
    const { activeId: activeIdString } = useParams();
    if (!activeIdString) {
        return <div>wrong active id</div>;
    }
    const activeId = +activeIdString;
    const { currentData: active } = stocksTracker.useGetActiveQuery({
        activeId,
    });
    const { currentData: notifications, refetch } =
        stocksTracker.useListActiveNotificationsQuery({
            activeId,
        });
    const [createNotification] = stocksTracker.useCreateNotificationMutation();
    const [deleteNotification] = stocksTracker.useDeleteNotificationMutation();

    // Form state
    const [newNotification, setNewNotification] = useState({
        active_id: activeId || '',
        limit_lower: '',
        limit_upper: '',
        limit_type: '',
    });

    // Handle input changes
    const handleInputChange = (e: ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        setNewNotification({
            ...newNotification,
            [name]: value,
        });
    };

    // Handle form submission
    const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        try {
            // Convert string values to appropriate types
            await createNotification({
                notificationRequest: {
                    active_id: parseInt(newNotification.active_id + '', 10),
                    limit_lower: parseFloat(newNotification.limit_lower),
                    limit_upper: parseFloat(newNotification.limit_upper),
                    limit_type: 'RUB',
                    portfolio_id: 0,
                },
            }).unwrap();

            // Reset form after successful submission
            setNewNotification({
                active_id: activeId || '',
                limit_lower: '',
                limit_upper: '',
                limit_type: '',
            });
            refetch();
        } catch (error) {
            console.error('Failed to create notification:', error);
        }
    };

    // Handle notification deletion
    const handleDeleteNotification = async (
        e: React.MouseEvent,
        notificationId: number,
    ) => {
        e.preventDefault(); // Prevent navigation from Link
        e.stopPropagation(); // Prevent event bubbling

        try {
            await deleteNotification({ notificationId }).unwrap();
            refetch(); // Refresh notifications list
        } catch (error) {
            console.error('Failed to delete notification:', error);
        }
    };

    return (
        <div className={cn(className, cls.activePage)}>
            <Link to={staticRoutes.home}>
                <h1>Go back</h1>
            </Link>
            <h1>security_id: {active?.security_id}</h1>
            <h1>bought_price: {active?.bought_price}</h1>
            <h1>count: {active?.count}</h1>
            <h1>currency: {active?.currency}</h1>

            <div>
                <h1>Add New Notification</h1>

                <form
                    onSubmit={handleSubmit}
                    className={cls.form}
                >
                    <div>
                        <label htmlFor='limit_lower'>Lower Limit:</label>
                        <input
                            type='number'
                            id='limit_lower'
                            name='limit_lower'
                            step='0.01'
                            value={newNotification.limit_lower}
                            onChange={handleInputChange}
                            required
                        />
                    </div>

                    <div>
                        <label htmlFor='limit_upper'>Upper Limit:</label>
                        <input
                            type='number'
                            id='limit_upper'
                            name='limit_upper'
                            step='0.01'
                            value={newNotification.limit_upper}
                            onChange={handleInputChange}
                            required
                        />
                    </div>

                    <button
                        type='submit'
                        className={cls.submitButton}
                    >
                        Create Notification
                    </button>
                </form>
            </div>

            <div>
                <h2>Notifications</h2>
                {notifications?.map((notification) => (
                    <div
                        key={notification.notification_id}
                        className={cls.notificationItem}
                    >
                        <div
                            // to={`${staticRoutes.notification}/${notification.notification_id}`}
                            className={cls.notificationLink}
                        >
                            <div>
                                <p>Lower Limit: {notification.limit_lower}</p>
                                <p>Upper Limit: {notification.limit_upper}</p>
                                <p>Limit Type: {notification.limit_type}</p>
                            </div>
                        </div>
                        <button
                            className={cls.deleteButton}
                            onClick={(e) =>
                                handleDeleteNotification(
                                    e,
                                    notification.notification_id,
                                )
                            }
                        >
                            Delete
                        </button>
                    </div>
                ))}
                {notifications?.length === 0 && (
                    <p>No notifications found for this active.</p>
                )}
            </div>
        </div>
    );
}
