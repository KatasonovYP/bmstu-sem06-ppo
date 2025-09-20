import { ChangeEvent, FormEvent, useState } from 'react';
import cn from 'classnames';
import cls from './home-page.module.css';
import { stocksTracker } from '@/shared/store';
import { Link } from 'react-router-dom';
import { staticRoutes } from '@/general/shared/config/const';

interface HomePageProps {
    className?: string;
}

export default function HomePage(props: HomePageProps) {
    const { className } = props;
    const { currentData: actives, refetch } =
        stocksTracker.useListUserActivesQuery();
    const [createActive] = stocksTracker.useCreateActiveMutation();
    const [deleteActive] = stocksTracker.useDeleteActiveMutation();

    // Form state
    const [newActive, setNewActive] = useState({
        security_id: '',
        bought_price: '',
        count: '',
        currency: '',
        user_id: '',
    });

    const [error, setError] = useState('');

    // Handle input changes
    const handleInputChange = (e: ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;
        setNewActive({
            ...newActive,
            [name]: value,
        });
    };

    // Handle form submission
    const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        try {
            // Convert string values to appropriate types
            setError('');

            await createActive({
                activeRequest: {
                    security_id: newActive.security_id,
                    bought_price: parseFloat(newActive.bought_price),
                    count: parseInt(newActive.count, 10),
                    currency: 'RUB',
                    user_id: 0,
                },
            }).unwrap();

            // Reset form after successful submission
            setNewActive({
                security_id: '',
                bought_price: '',
                count: '',
                currency: '',
                user_id: '',
            });
            refetch();
        } catch (error) {
            console.error('Failed to delete active:', error);
            setError(`${(error as any).data.error}`);
        }
    };

    // Handle active deletion
    const handleDeleteActive = async (
        e: React.MouseEvent,
        activeId: number,
    ) => {
        e.preventDefault(); // Prevent navigation from Link
        e.stopPropagation(); // Prevent event bubbling

        try {
            await deleteActive({ activeId }).unwrap();
            refetch(); // Refresh actives list
        } catch (error) {
            console.error('Failed to delete active:', error);
        }
    };

    return (
        <div>
            <h1>Добавить новый актив</h1>

            <div className={cls.formContainer}>
                <form
                    onSubmit={handleSubmit}
                    className={cls.form}
                >
                    <div className={cls.formGroup}>
                        <label htmlFor='security_id'>ID ценной бумаги:</label>
                        <input
                            type='text'
                            id='security_id'
                            name='security_id'
                            value={newActive.security_id}
                            onChange={handleInputChange}
                            required
                        />
                    </div>

                    <div className={cls.formGroup}>
                        <label htmlFor='bought_price'>Закупочная цена:</label>
                        <input
                            type='number'
                            id='bought_price'
                            name='bought_price'
                            step='0.01'
                            value={newActive.bought_price}
                            onChange={handleInputChange}
                            required
                        />
                    </div>

                    <div className={cls.formGroup}>
                        <label htmlFor='count'>Количество:</label>
                        <input
                            type='number'
                            id='count'
                            name='count'
                            value={newActive.count}
                            onChange={handleInputChange}
                            required
                        />
                    </div>

                    <p className='text-red-500'>{error}</p>

                    <button
                        type='submit'
                        className={cls.submitButton}
                    >
                        Создать актив
                    </button>
                </form>
            </div>

            {/* List of existing actives */}
            <div>
                <h2>Your Actives</h2>
                {actives?.map((active) => (
                    <div
                        key={active.active_id}
                        className={cls.activeItem}
                    >
                        <Link
                            to={staticRoutes.active.replace(
                                ':activeId',
                                active.active_id + '',
                            )}
                            className={cls.activeLink}
                        >
                            <div>
                                <p>ID ценной бумаги: {active.security_id}</p>
                                <p>Закупочная цена: {active.bought_price}</p>
                                <p>Количество: {active.count}</p>
                                <p>Валюта: {active.currency}</p>
                            </div>
                        </Link>
                        <button
                            className={cls.deleteButton}
                            onClick={(e) =>
                                handleDeleteActive(e, active.active_id)
                            }
                        >
                            Удалить
                        </button>
                    </div>
                ))}
                {actives?.length === 0 && <p>Активы не найдены.</p>}
            </div>
        </div>
    );
}
