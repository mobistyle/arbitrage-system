use criterion::{criterion_group, criterion_main, Criterion};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

fn process_prices_benchmark(c: &mut Criterion) {
    c.bench_function("process 1000 prices", |b| {
        b.iter(|| {
            let price1 = dec!(100.0);
            let price2 = dec!(101.0);
            let _spread = (price2 - price1) * dec!(100) / price1;
        })
    });
}

criterion_group!(benches, process_prices_benchmark);
criterion_main!(benches);
