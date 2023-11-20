use std::iter::repeat_with;

use divan::black_box;

use quickdiv::*;

fn main() {
    divan::main();
}

const BATCH_SIZE: usize = 1000;
const SEED: u64 = 42;

macro_rules! quotient_sum {
    ($name:ident, $BaseT:ident, $new_div_fn:expr) => {
        #[divan::bench(sample_count = 1000)]
        fn $name(bencher: divan::Bencher) {
            let mut rng = fastrand::Rng::with_seed(SEED);

            bencher
                .counter(divan::counter::ItemsCount::new(BATCH_SIZE))
                .with_inputs(|| -> (Vec<_>, _) {
                    let dividends = repeat_with(|| rng.$BaseT(..)).take(BATCH_SIZE).collect();
                    let divisor = $new_div_fn(rng.$BaseT(1..));
                    (dividends, divisor)
                })
                .bench_local_refs(|(dividends, divisor)| {
                    dividends
                        .iter()
                        .fold(0, |acc: $BaseT, x| acc.wrapping_add(*x / *divisor))
                });
        }
    };
}

macro_rules! bench_quotient_sum {
    ($DivisorT:ident, $BaseT:ident) => {
        mod $BaseT {
            use super::*;

            quotient_sum!(cpu, $BaseT, black_box);
            quotient_sum!(quickdiv, $BaseT, $DivisorT::new);
        }
    };
}

bench_quotient_sum!(DivisorI8, i8);
bench_quotient_sum!(DivisorI16, i16);
bench_quotient_sum!(DivisorI32, i32);
bench_quotient_sum!(DivisorI64, i64);
bench_quotient_sum!(DivisorI128, i128);
bench_quotient_sum!(DivisorIsize, isize);

bench_quotient_sum!(DivisorU8, u8);
bench_quotient_sum!(DivisorU16, u16);
bench_quotient_sum!(DivisorU32, u32);
bench_quotient_sum!(DivisorU64, u64);
bench_quotient_sum!(DivisorU128, u128);
bench_quotient_sum!(DivisorUsize, usize);
