#[macro_use]
extern crate criterion;
use rust_nbody;

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("run simulation", |b| {
    b.iter(|| rust_nbody::compute_forces(rust_nbody::init()))
  });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
