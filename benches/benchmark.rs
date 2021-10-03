extern crate criterion;
extern crate ramp;
extern crate rand;
extern crate rand_pcg;

use std::fmt;

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use ramp::ll::limb::Limb;
use ramp::{Int, RandomInt};
use rand::prelude::*;
use rand_pcg::Pcg64;

criterion_group!(benches, bench_add, bench_mul);
criterion_main!(benches);

const SEED: u64 = 10987654321;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Dim(usize, usize);



impl fmt::Display for Dim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

fn bench_add(c: &mut Criterion) {
    let mut rng = Pcg64::seed_from_u64(SEED);

    let mut group = c.benchmark_group("add");
    for xs in [1, 10, 100, 1000] {
        for ys in [1, 10, 100, 1000] {
            let x = rng.gen_int(xs * Limb::BITS);
            let y = rng.gen_int(ys * Limb::BITS);
            group.bench_with_input(BenchmarkId::from_parameter(Dim(xs, ys)), &(x, y), |b, (x, y)| {
                let x = x.clone();
                let y = y.clone();
                b.iter(|| {
                    let z = &x + &y;
                    black_box(z);
                })
            });
        }
    }
}

fn bench_mul(c: &mut Criterion) {
    bench_op(c, [1, 10, 100, 1000], |x, y| x * y);
}

fn bench_op<F, const N: usize>(c: &mut Criterion, input_ranges: [usize; N], op: F) where F: FnOnce(Int, Int) -> Int {
    let mut rng = Pcg64::seed_from_u64(SEED);

    let mut group = c.benchmark_group("pow");
    for xs in input_ranges {
        for ys in input_ranges {
            let x = rng.gen_int(xs * Limb::BITS);
            let y = rng.gen_int(ys * Limb::BITS);
            group.bench_with_input(BenchmarkId::from_parameter(Dim(xs, ys)), &(x, y), |b, (x, y)| {
                let x = x.clone();
                let y = y.clone();
                b.iter(|| {
                    let z = op(x, y);
                    black_box(z);
                })
            });
        }
    }
} 

fn bench_pow(c: &mut Criterion) {
    let mut rng = Pcg64::seed_from_u64(SEED);

    let mut group = c.benchmark_group("pow");
    for xs in [1, 10, 100, 1000] {
        for ys in [1, 10, 100, 1000] {
            let x = rng.gen_int(xs * Limb::BITS);
            let y = rng.gen_range(0..ys);
            group.bench_with_input(BenchmarkId::from_parameter(Dim(xs, ys)), &(x, y), |b, (x, y)| {
                let x = x.clone();
                let y = y.clone();
                b.iter(|| {
                    let z = &x.pow(y);
                    black_box(z);
                })
            });
        }
    }
}

