use std::sync::Arc;

use criterion::{
    Criterion,
    black_box,
    criterion_group,
    criterion_main,
};
use domain::{
    models::ActiveEntity,
    ports::{
        domain::AbstractPriceOpsService,
        exchange::MockExchangeRepository,
    },
    services::PriceOpsService,
    value_objects::Price,
};

fn bench_get_actives_bought_price(c: &mut Criterion) {
    let mut actives = fake::vec![ActiveEntity; 2];

    actives[0].bought_price = Price::rub(100.);
    actives[0].count = 2;

    actives[1].bought_price = Price::rub(30.);
    actives[1].count = 3;

    let mock_exchange_repository = MockExchangeRepository::new();
    let service = PriceOpsService::new(Arc::new(mock_exchange_repository));

    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("get_actives_bought_price", |b| {
        b.to_async(&rt).iter(|| {
            let service = service.clone();
            let actives = actives.clone();
            async move {
                service
                    .get_actives_bought_price(black_box(actives))
                    .await
                    .unwrap()
            }
        });
    });
}

criterion_group!(benches, bench_get_actives_bought_price);
criterion_main!(benches);
