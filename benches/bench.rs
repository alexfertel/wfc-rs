use ::wfc;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn size_2(c: &mut Criterion) {
    let image = image::open("bench_data/red-maze.png").unwrap().to_rgb8();
    let pattern_set = wfc::get_patterns(&image, 2);
    let patterns = pattern_set.iter().collect();
    let solver = wfc::Wfc::new(patterns);

    let mut group = c.benchmark_group("sample-size-10");
    group.sample_size(10);
    group.bench_function("32x32-with-p-size-2", |b| {
        b.iter(|| solver.generate(black_box(32), black_box(32)))
    });
    group.finish();
}

fn size_3(c: &mut Criterion) {
    let image = image::open("bench_data/water.png").unwrap().to_rgb8();
    let pattern_set = wfc::get_patterns(&image, 3);
    let patterns = pattern_set.iter().collect();
    let solver = wfc::Wfc::new(patterns);

    let mut group = c.benchmark_group("sample-size-10");
    group.sample_size(10);
    group.bench_function("32x32 with p-size 3", |b| {
        b.iter(|| solver.generate(black_box(32), black_box(32)))
    });
    group.finish();
}

criterion_group!(benches, size_2, size_3);
criterion_main!(benches);
