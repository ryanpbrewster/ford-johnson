use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::seq::SliceRandom;
use rand::SeedableRng;

fn criterion_benchmark(c: &mut Criterion) {
    for n in vec![10, 100, 1_000, 10_000] {
        c.bench_function(&format!("sort preordered unique {}", n), |b| {
            let mut xs: Vec<usize> = (0..n).collect();
            b.iter(|| {
                ford_johnson::sort(&mut xs, &mut |a, b| a.cmp(&b));
            })
        });
    }

    for n in vec![10, 100, 1_000, 10_000] {
        c.bench_function(&format!("sort shuffled non-unique {}", n), |b| {
            let mut xs: Vec<usize> = (0..n / 10).cycle().take(n).collect();
            b.iter(|| {
                let mut prng = rand_pcg::Pcg32::seed_from_u64(42);
                xs.shuffle(&mut prng);
                ford_johnson::sort(&mut xs, &mut |a, b| a.cmp(&b));
            })
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
