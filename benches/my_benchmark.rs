#![allow(unused)]
// use ascon_hash_implementation::State;
// use ascon_hash_implementation::{
//     vec_u8_to_u64_and_pad, vec_u8_to_u64_and_pad_version_2, StringToU8,
// };
use ascon_hash_implementation::parallel_implementations::parallel_implementations::ascon_hash_pool;

use ascon_hash_implementation::*;

// Implementation from https://docs.rs/ascon/latest/ascon/ for comparison with my code
fn round(x: [u64; 5], c: u64) -> [u64; 5] {
    // S-box layer
    let x0 = x[0] ^ x[4];
    let x2 = x[2] ^ x[1] ^ c; // with round constant
    let x4 = x[4] ^ x[3];

    let tx0 = x0 ^ (!x[1] & x2);
    let tx1 = x[1] ^ (!x2 & x[3]);
    let tx2 = x2 ^ (!x[3] & x4);
    let tx3 = x[3] ^ (!x4 & x0);
    let tx4 = x4 ^ (!x0 & x[1]);

    let tx1 = tx1 ^ tx0;
    let tx3 = tx3 ^ tx2;
    let tx0 = tx0 ^ tx4;

    let x0 = tx0 ^ tx0.rotate_right(9);
    let x1 = tx1 ^ tx1.rotate_right(22);
    let x2 = tx2 ^ tx2.rotate_right(5);
    let x3 = tx3 ^ tx3.rotate_right(7);
    let x4 = tx4 ^ tx4.rotate_right(34);
    [
        tx0 ^ x0.rotate_right(19),
        tx1 ^ x1.rotate_right(39),
        !(tx2 ^ x2.rotate_right(1)),
        tx3 ^ x3.rotate_right(10),
        tx4 ^ x4.rotate_right(7),
    ]
}

fn permute_12_theirs(mut arr: [u64; 5]) -> [u64; 5] {
    arr = round(
        round(
            round(
                round(
                    round(
                        round(
                            round(
                                round(
                                    round(round(round(round(arr, 0xf0), 0xe1), 0xd2), 0xc3),
                                    0xb4,
                                ),
                                0xa5,
                            ),
                            0x96,
                        ),
                        0x87,
                    ),
                    0x78,
                ),
                0x69,
            ),
            0x5a,
        ),
        0x4b,
    );
    arr
}

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
    c.bench_function("round-function", |b| {
        b.iter(|| round(black_box([0, 0, 0, 0, 0]), 8))
    });
    c.bench_function("single_permutation_bench", |b| {
        b.iter(|| black_box(s.single_permutation(8)))
        // other option iter_with_large_drop  -> drops the results on each iteration
    });
    c.bench_function("single_permutation_concurent_bench", |b| {
        b.iter(|| black_box(s.single_permutation_concurrent_mutex(8)))
        // other option iter_with_large_drop  -> drops the results on each iteration
    });
    c.bench_function("single_permutation_with_tupples", |b| {
        b.iter(|| black_box(s.single_permutation_with_tupples(8)))
        // other option iter_with_large_drop  -> drops the results on each iteration
    });
}

fn compare_for_and_it(c: &mut Criterion) {
    let mut s_0: State = State::new(0, 0, 0, 0, 0);
    let mut s_1 = State::new(0, 0, 0, 0, 0);
    let mut arr: [u64; 5] = [0, 0, 0, 0, 0];
    c.bench_function("permutation_12_iter", |b| {
        b.iter(|| black_box(s_0.permutation_12()))
        // other option iter_with_large_drop  -> drops the results on each iteration
    });
    c.bench_function("permutation_12_for", |b| {
        b.iter(|| black_box(s_0.permutation_12_for()))
    });
    c.bench_function("permute_12_theirs", |b| {
        b.iter(|| permute_12_theirs(black_box(arr)))
    });
}

fn padding_functions(c: &mut Criterion) {
    // convert_pad_into_blocks()
    // string_to_pad()
    // vec_u8_to_u64_and_pad

    c.bench_function("convert_u8_to_u64_and_add_adding", |b| {
        b.iter(|| {
            vec_u8_to_u64_and_pad(black_box(vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
            ]))
        })
    });
    c.bench_function("convert_u8_to_u64_and_add_adding", |b| {
        b.iter(|| {
            vec_u8_to_u64_and_pad_version_2(black_box(vec![
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
            ]))
        })
    });
    c.bench_function("convert_string_to_u8", |b| {
        b.iter(|| "aßsjof9ßoipamkf3ap0fnafs".string_to_u8())
    });
}

fn ascon_full_parallel_bench(c: &mut Criterion) {
    let input = "some bytes".to_string();
    c.bench_function("ascon", |b| {
        b.iter(|| {
            ascon_hash(black_box(&input));
        })
    });
    c.bench_function("ascon parallel", |b| {
        b.iter(|| {
            ascon_hash_pool(black_box(&input));
        })
    });
}

criterion_group!(benches, criterion_benchmark, compare_for_and_it); // creates a benchmark group benches (1st) that
                                                                    // contains the criterion_benchmark function (2nd)

criterion_group!(benches_pad, padding_functions);
criterion_group!(ascon_full, ascon_full_parallel_bench);

criterion_main!(benches, benches_pad); // macro that creates a main function that executes the bench mark for us
                                       // criterion_main!(benches); // macro that creates a main function that executes the bench mark for us
                                       // criterion_main!(benches_pad); // macro that creates a main function that executes the bench mark for us
