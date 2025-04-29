use criterion::{criterion_group, criterion_main, Criterion};
use gi_ui::canvas::Canvas;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("create_and_resize", |b| b.iter(|| {
        let mut canvas = Canvas::new(200, 200);

        canvas.resize(1920, 1280);
    }));

    c.bench_function("create_and_resize_less", |b| b.iter(|| {
        let mut canvas = Canvas::new(1920, 1280);

        canvas.resize(200, 200);
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
