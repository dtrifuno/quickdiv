use divan::black_box;
use quickdiv::*;
use std::cell::RefCell;

const BATCH_SIZE: usize = 1000;
const SEED: u64 = 42;

macro_rules! quotient_sum {
    ($name:ident, $BaseT:ident, $new_fn:expr) => {
        #[divan::bench()]
        fn $name(bencher: divan::Bencher) {
            let rng = RefCell::new(fastrand::Rng::with_seed(SEED));

            bencher
                .counter(divan::counter::ItemsCount::new(BATCH_SIZE))
                .with_inputs(|| {
                    let d = $new_fn(rng.borrow_mut().$BaseT(1..));
                    let values = (0..BATCH_SIZE)
                        .map(|_| rng.borrow_mut().$BaseT(..))
                        .collect::<Vec<_>>();
                    (values, d)
                })
                .bench_local_refs(|(values, d)| {
                    let mut sum: $BaseT = 0;
                    for value in values {
                        sum = sum.wrapping_add(black_box(*value / *d));
                    }
                    black_box(sum);
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

fn main() {
    divan::main();
}
