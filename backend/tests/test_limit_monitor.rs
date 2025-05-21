use std::sync::Arc;

use fake::{
    Fake,
    Faker,
};
use mockall::predicate::eq;
use shaku::HasComponent;
use stocks_tracker::{
    app::di_domain_module::di_domain_module,
    domain::{
        models::{
            ActiveEntity,
            NotificationEntity,
            TradeEntity,
            UserEntity,
        },
        ports::{
            domain::AbstractLimitMonitorService,
            exchange::MockExchangeRepository,
            sender::MockNotificationSender,
            storage::{
                ActiveRepository,
                NotificationRepository,
                UserRepository,
            },
        },
        services::{
            LimitMonitorService,
            PriceCacheService,
            PriceOpsService,
        },
        value_objects::Price,
    },
};

#[tokio::test]
async fn test_limit_monitor() {
    let module = di_domain_module().await;
    let user_repo: Arc<dyn UserRepository> = module.resolve();
    let active_repo: Arc<dyn ActiveRepository> = module.resolve();
    let notification_repo: Arc<dyn NotificationRepository> = module.resolve();

    let mut moex_repo = MockExchangeRepository::new();

    let sequrity_id: String = "APPL".into();
    let mut trade: TradeEntity = Faker.fake();
    trade.security_id = sequrity_id.clone();
    trade.price = 10.;

    moex_repo
        .expect_get_trades()
        .with(eq(sequrity_id.clone()))
        .returning(move |_| Ok(vec![trade.clone()]));
    let price_ops_service = PriceOpsService::new(Arc::new(moex_repo));
    let price_cache_service = PriceCacheService::new(
        active_repo.clone(),
        module.resolve(),
        Arc::new(price_ops_service),
    );

    let mut notification_sender = MockNotificationSender::new();
    notification_sender
        .expect_send_message()
        .returning(|_, _| Ok("".into()));

    let limit_monitor_service = LimitMonitorService::new(
        user_repo.clone(),
        active_repo.clone(),
        notification_repo.clone(),
        module.resolve(),
        Arc::new(price_cache_service),
        Arc::new(notification_sender),
    );

    let users_on_start_test = user_repo.list_users().await.unwrap();
    let actives_on_start_test = active_repo.list_actives().await.unwrap();
    let notifications_on_start_test = notification_repo.list_notifications().await.unwrap();

    let test_user: UserEntity = Faker.fake();
    let result_user = user_repo.create_user(test_user.clone()).await.unwrap();

    let mut test_active: ActiveEntity = Faker.fake();
    test_active.user_id = result_user.user_id;
    test_active.bought_price = Price::rub(100.);
    test_active.count = 1;
    test_active.security_id = sequrity_id;
    let result_active = active_repo
        .create_active(test_active.clone())
        .await
        .unwrap();

    let mut test_notification: NotificationEntity = Faker.fake();
    test_notification.active_id = result_active.active_id;
    test_notification.limit_lower = Price::rub(90.);
    test_notification.limit_upper = Price::rub(110.);
    test_notification.portfolio_id = 10;

    let result_notification = notification_repo
        .create_notification(test_notification.clone())
        .await
        .unwrap();

    limit_monitor_service
        .send_exeeding_messages()
        .await
        .unwrap();

    notification_repo
        .delete_notification(result_notification.notification_id)
        .await
        .unwrap();
    active_repo
        .delete_active(result_active.active_id)
        .await
        .unwrap();
    user_repo.delete_user(result_user.user_id).await.unwrap();

    let users_on_end_test = user_repo.list_users().await.unwrap();
    let actives_on_end_test = active_repo.list_actives().await.unwrap();
    let notifications_on_end_test = notification_repo.list_notifications().await.unwrap();
    assert_eq!(users_on_start_test, users_on_end_test);
    assert_eq!(actives_on_start_test, actives_on_end_test);
    assert_eq!(notifications_on_start_test, notifications_on_end_test);
}
