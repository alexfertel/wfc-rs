use ::wfc;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn generate_size_2(c: &mut Criterion) {
    let image = image::open("bench_data/red-maze.png").unwrap().to_rgb8();
    let pattern_set = wfc::get_patterns(&image, 2);
    let patterns = pattern_set.iter().collect();
    let solver = wfc::Wfc::new(patterns);
    c.bench_function("generate pattern size 2", |b| {
        b.iter(|| solver.generate(black_box(10), black_box(10)))
    });
}

criterion_group!(benches, generate_size_2);
criterion_main!(benches);
