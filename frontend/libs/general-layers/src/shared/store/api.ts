import { baseApi as api } from './base-api';
const injectedRtkApi = api.injectEndpoints({
    endpoints: (build) => ({
        listUserActives: build.query<
            ListUserActivesApiResponse,
            ListUserActivesApiArg
        >({
            query: () => ({ url: `/api/v1/actives` }),
        }),
        createActive: build.mutation<
            CreateActiveApiResponse,
            CreateActiveApiArg
        >({
            query: (queryArg) => ({
                url: `/api/v1/actives`,
                method: 'POST',
                body: queryArg.activeRequest,
            }),
        }),
        getActive: build.query<GetActiveApiResponse, GetActiveApiArg>({
            query: (queryArg) => ({
                url: `/api/v1/actives/${queryArg.activeId}`,
            }),
        }),
        updateActive: build.mutation<
            UpdateActiveApiResponse,
            UpdateActiveApiArg
        >({
            query: (queryArg) => ({
                url: `/api/v1/actives/${queryArg.activeId}`,
                method: 'PUT',
                body: queryArg.activeRequest,
            }),
        }),
        deleteActive: build.mutation<
            DeleteActiveApiResponse,
            DeleteActiveApiArg
        >({
            query: (queryArg) => ({
                url: `/api/v1/actives/${queryArg.activeId}`,
                method: 'DELETE',
            }),
        }),
        login: build.mutation<LoginApiResponse, LoginApiArg>({
            query: () => ({ url: `/api/v1/auth/login`, method: 'POST' }),
        }),
        createNotification: build.mutation<
            CreateNotificationApiResponse,
            CreateNotificationApiArg
        >({
            query: (queryArg) => ({
                url: `/api/v1/notifications`,
                method: 'POST',
                body: queryArg.notificationRequest,
            }),
        }),
        listActiveNotifications: build.query<
            ListActiveNotificationsApiResponse,
            ListActiveNotificationsApiArg
        >({
            query: (queryArg) => ({
                url: `/api/v1/notifications/all/${queryArg.activeId}`,
            }),
        }),
        getNotification: build.query<
            GetNotificationApiResponse,
            GetNotificationApiArg
        >({
            query: (queryArg) => ({
                url: `/api/v1/notifications/${queryArg.notificationId}`,
            }),
        }),
        deleteNotification: build.mutation<
            DeleteNotificationApiResponse,
            DeleteNotificationApiArg
        >({
            query: (queryArg) => ({
                url: `/api/v1/notifications/${queryArg.notificationId}`,
                method: 'DELETE',
            }),
        }),
        updateNotification: build.mutation<
            UpdateNotificationApiResponse,
            UpdateNotificationApiArg
        >({
            query: (queryArg) => ({
                url: `/api/v1/notifications/${queryArg.notificationId}`,
                method: 'PATCH',
                body: queryArg.notificationRequest,
            }),
        }),
        listUsers: build.query<ListUsersApiResponse, ListUsersApiArg>({
            query: () => ({ url: `/api/v1/users` }),
        }),
        createUser: build.mutation<CreateUserApiResponse, CreateUserApiArg>({
            query: (queryArg) => ({
                url: `/api/v1/users`,
                method: 'POST',
                body: queryArg.userRequest,
            }),
        }),
        getUser: build.query<GetUserApiResponse, GetUserApiArg>({
            query: (queryArg) => ({ url: `/api/v1/users/${queryArg.userId}` }),
        }),
        deleteUser: build.mutation<DeleteUserApiResponse, DeleteUserApiArg>({
            query: (queryArg) => ({
                url: `/api/v1/users/${queryArg.userId}`,
                method: 'DELETE',
            }),
        }),
        updateUser: build.mutation<UpdateUserApiResponse, UpdateUserApiArg>({
            query: (queryArg) => ({
                url: `/api/v1/users/${queryArg.userId}`,
                method: 'PATCH',
                body: queryArg.userRequest,
            }),
        }),
    }),
    overrideExisting: false,
});
export { injectedRtkApi as stocksTracker };
export type ListUserActivesApiResponse =
    /** status 200 List active success */ ActiveResponse[];
export type ListUserActivesApiArg = void;
export type CreateActiveApiResponse =
    /** status 200 Create active success */ ActiveResponse;
export type CreateActiveApiArg = {
    activeRequest: ActiveRequest;
};
export type GetActiveApiResponse =
    /** status 200 Get active success */ ActiveResponse;
export type GetActiveApiArg = {
    /** Active id */
    activeId: number;
};
export type UpdateActiveApiResponse =
    /** status 200 Update active success */ ActiveResponse;
export type UpdateActiveApiArg = {
    /** Active id */
    activeId: any;
    activeRequest: ActiveRequest;
};
export type DeleteActiveApiResponse =
    /** status 200 Delete active success */ ActiveResponse;
export type DeleteActiveApiArg = {
    /** Active id */
    activeId: number;
};
export type LoginApiResponse = /** status 200 Login successful */ LoginResponse;
export type LoginApiArg = void;
export type CreateNotificationApiResponse =
    /** status 200 Create notification success */ NotificationResponse;
export type CreateNotificationApiArg = {
    notificationRequest: NotificationRequest;
};
export type ListActiveNotificationsApiResponse =
    /** status 200 List notification success */ NotificationResponse[];
export type ListActiveNotificationsApiArg = {
    /** Active id */
    activeId: number;
};
export type GetNotificationApiResponse =
    /** status 200 Get notification success */ NotificationResponse;
export type GetNotificationApiArg = {
    /** Notification id */
    notificationId: number;
};
export type DeleteNotificationApiResponse =
    /** status 200 Delete notification success */ NotificationResponse;
export type DeleteNotificationApiArg = {
    /** Notification id */
    notificationId: number;
};
export type UpdateNotificationApiResponse =
    /** status 200 Update notification success */ NotificationResponse;
export type UpdateNotificationApiArg = {
    /** Notification id */
    notificationId: any;
    notificationRequest: NotificationRequest;
};
export type ListUsersApiResponse =
    /** status 200 List user success */ UserResponse[];
export type ListUsersApiArg = void;
export type CreateUserApiResponse =
    /** status 200 Create user success */ UserResponse;
export type CreateUserApiArg = {
    userRequest: UserRequest;
};
export type GetUserApiResponse =
    /** status 200 Get user success */ UserResponse;
export type GetUserApiArg = {
    /** User id */
    userId: number;
};
export type DeleteUserApiResponse =
    /** status 200 Delete user success */ UserResponse;
export type DeleteUserApiArg = {
    /** User id */
    userId: number;
};
export type UpdateUserApiResponse =
    /** status 200 Update user success */ UserResponse;
export type UpdateUserApiArg = {
    /** User id */
    userId: any;
    userRequest: UserRequest;
};
export type ActiveResponse = {
    active_id: number;
    bought_price: number;
    count: number;
    currency: string;
    security_id: string;
    user_id: number;
};
export type ActiveRequest = {
    bought_price: number;
    count: number;
    currency: string;
    security_id: string;
    user_id: number;
};
export type LoginResponse = {
    access_token: string;
    expires_in: number;
    token_type: string;
};
export type NotificationResponse = {
    active_id: number;
    limit_lower: number;
    limit_type: string;
    limit_upper: number;
    notification_id: number;
    portfolio_id: number;
};
export type NotificationRequest = {
    active_id: number;
    limit_lower: number;
    limit_type: string;
    limit_upper: number;
    portfolio_id: number;
};
export type UserResponse = {
    chat_id: number;
    first_name?: string | null;
    second_name?: string | null;
    tg_id: number;
    user_id: number;
    username: string;
};
export type UserRequest = {
    chat_id: number;
    first_name?: string | null;
    second_name?: string | null;
    tg_id: number;
    username: string;
};
export const {
    useListUserActivesQuery,
    useLazyListUserActivesQuery,
    useCreateActiveMutation,
    useGetActiveQuery,
    useLazyGetActiveQuery,
    useUpdateActiveMutation,
    useDeleteActiveMutation,
    useLoginMutation,
    useCreateNotificationMutation,
    useListActiveNotificationsQuery,
    useLazyListActiveNotificationsQuery,
    useGetNotificationQuery,
    useLazyGetNotificationQuery,
    useDeleteNotificationMutation,
    useUpdateNotificationMutation,
    useListUsersQuery,
    useLazyListUsersQuery,
    useCreateUserMutation,
    useGetUserQuery,
    useLazyGetUserQuery,
    useDeleteUserMutation,
    useUpdateUserMutation,
} = injectedRtkApi;
