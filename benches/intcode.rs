#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;

use advent_of_rust_2019::intcode_computer::Computer;
use advent_of_rust_2019::{load_file, parse_custom_separated};

fn bench_sum_of_primes(c: &mut Criterion) {
    c.bench_function("Sum Of Primes", |b| {
        let file = load_file("sum-of-primes.in");
        let program: Vec<_> = parse_custom_separated::<isize>(&file, ",").collect();

        b.iter(move || {
            let mut computer = black_box(Computer::with_input(program.clone(), || Some(100000)));

            computer.run_until_halt_or_paused(true);

            let output = black_box(computer.last_output());

            assert!(
                output == Some(454396537),
                "Expected 454396537 got {:?}",
                output
            );
        });
    });
}

criterion_group!(benches, bench_sum_of_primes);
criterion_main!(benches);
