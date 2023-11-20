use std::iter::repeat_with;

use divan::black_box;

use quickdiv::*;

fn main() {
    divan::main();
}

const BATCH_SIZE: usize = 1000;
const SEED: u64 = 42;

macro_rules! fizzbuzz {
    ($name:ident, $BaseT:ident, $divides_fn:expr, $div0:expr, $div1:expr) => {
        #[divan::bench(sample_count = 1000)]
        fn $name(bencher: divan::Bencher) {
            let mut rng = fastrand::Rng::with_seed(SEED);
            let div_0 = $div0;
            let div_1 = $div1;

            bencher
                .counter(divan::counter::ItemsCount::new(BATCH_SIZE))
                .with_inputs(|| repeat_with(|| rng.$BaseT(..)).take(BATCH_SIZE).collect())
                .bench_local_refs(|values: &mut Vec<_>| {
                    let mut count_div_0s = 0;
                    let mut count_div_1s = 0;

                    for n in values {
                        if $divides_fn(*n, div_0) {
                            count_div_0s += 1;
                        }
                        if $divides_fn(*n, div_1) {
                            count_div_1s += 1;
                        }
                    }

                    (count_div_0s, count_div_1s)
                })
        }
    };
}

macro_rules! bench_fizzbuzz {
    ($DivisorT:ident, $BaseT:ident) => {
        mod $BaseT {
            use super::*;

            fizzbuzz!(compiler, $BaseT, |n, d| n % d == 0, 3, 5);
            fizzbuzz!(cpu, $BaseT, |n, d| n % d == 0, black_box(3), black_box(5));
            fizzbuzz!(
                quickdiv,
                $BaseT,
                |n, d: $DivisorT| d.divides(n),
                $DivisorT::new(3),
                $DivisorT::new(5)
            );
        }
    };
}

bench_fizzbuzz!(DivisorI8, i8);
bench_fizzbuzz!(DivisorI16, i16);
bench_fizzbuzz!(DivisorI32, i32);
bench_fizzbuzz!(DivisorI64, i64);
bench_fizzbuzz!(DivisorI128, i128);
bench_fizzbuzz!(DivisorIsize, isize);

bench_fizzbuzz!(DivisorU8, u8);
bench_fizzbuzz!(DivisorU16, u16);
bench_fizzbuzz!(DivisorU32, u32);
bench_fizzbuzz!(DivisorU64, u64);
bench_fizzbuzz!(DivisorU128, u128);
bench_fizzbuzz!(DivisorUsize, usize);
