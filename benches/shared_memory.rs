use criterion::{black_box, criterion_group, criterion_main, Criterion};
use vapor_manager::telemetry::*;

pub fn convert_static(c: &mut Criterion) {
    c.bench_function("convert static data", |b| b.iter(|| {
        StaticData::from(black_box(*PageFileStatic::debug_data()))
    }));
}

pub fn convert_physics(c: &mut Criterion) {
    c.bench_function("convert physics data", |b| b.iter(|| {
        Physics::from(black_box(*PageFilePhysics::debug_data()))
    }));
}

pub fn convert_graphics(c: &mut Criterion) {
    c.bench_function("convert graphics data", |b| b.iter(|| {
        Graphics::from(black_box(*PageFileGraphics::debug_data()))
    }));
}

criterion_group!(benches, convert_static, convert_physics, convert_graphics);
criterion_main!(benches);
