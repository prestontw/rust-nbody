#[macro_use]
extern crate criterion;
use rust_nbody;

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("run simulation", |b| {
    let bodies = rust_nbody::init();
    b.iter(|| rust_nbody::compute_forces(&bodies, &bodies))
  });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
