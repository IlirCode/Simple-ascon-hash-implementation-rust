#![allow(unused)]
use ascon_hash_implementation::State;

// not as we have to import ascon_hash_implementation we can only bench public functions
use criterion::{black_box, criterion_group, criterion_main, Criterion};
// criterion = type that provides methods to configure and define groups of benchmarks
//

// benchmark function; name doesn't matter
fn criterion_benchmark(c: &mut Criterion) {
    let mut s: State = State::new(0, 0, 0, 0, 0);
    // bench function method defines a benchmark with a name
    // closure has to accept a Bencher, that performs the benchmarking
    // blackbox stops the compiler from replacing the function with a constant
    c.bench_function("single permutation 20 times", |b| {
        b.iter(|| black_box(s.single_permutation(8)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
