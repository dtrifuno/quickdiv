use std::iter::repeat_with;

use divan::black_box;

use quickdiv::*;

fn main() {
    divan::main()
}

const BATCH_SIZE: usize = 1000;
const SEED: u64 = 42;

macro_rules! lcg {
    ($name:ident, $BaseT:ident, $multiplier: expr, $modulus:expr) => {
        #[divan::bench(sample_count = 1000)]
        fn $name(bencher: divan::Bencher) {
            let mut rng = fastrand::Rng::with_seed(SEED);
            let modulus = $modulus;

            bencher
                .counter(divan::counter::ItemsCount::new(BATCH_SIZE))
                .with_inputs(|| rng.$BaseT(..))
                .bench_local_values(|seed| {
                    let mut prev = seed;
                    let lcg = repeat_with(|| {
                        let value = ($multiplier * prev) % modulus;
                        prev = value;
                        value
                    });
                    lcg.take(BATCH_SIZE).last()
                })
        }
    };
}

macro_rules! bench_lcg {
    ($DivisorT: ident, $BaseT:ident, $multiplier: expr, $modulus:expr) => {
        mod $BaseT {
            use super::*;

            lcg!(compiler, $BaseT, $multiplier, $modulus);
            lcg!(cpu, $BaseT, $multiplier, black_box($modulus));
            lcg!(quickdiv, $BaseT, $multiplier, $DivisorT::new($modulus));
        }
    };
}

bench_lcg!(DivisorU8, u8, 5, (1 << 3) - 1);
bench_lcg!(DivisorU16, u16, 33, (1 << 7) - 1);
bench_lcg!(DivisorU32, u32, 793, (1 << 13) - 1);
bench_lcg!(DivisorU64, u64, 48271, (1 << 31) - 1);
bench_lcg!(DivisorU128, u128, 0x5deece66d16, (1 << 61) - 1);
bench_lcg!(DivisorUsize, usize, 48271, (1 << 31) - 1);

bench_lcg!(DivisorI8, i8, 5, (1 << 3) - 1);
bench_lcg!(DivisorI16, i16, 33, (1 << 7) - 1);
bench_lcg!(DivisorI32, i32, 793, (1 << 13) - 1);
bench_lcg!(DivisorI64, i64, 48271, (1 << 31) - 1);
bench_lcg!(DivisorI128, i128, 0x5deece66d16, (1 << 61) - 1);
bench_lcg!(DivisorIsize, isize, 48271, (1 << 31) - 1);
