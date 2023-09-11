#![feature(test)]

extern crate test;

use test::{black_box, Bencher};

const TEST_SIZE: u128 = 1000;
const DIVISOR: u128 = 7;

const fn id<T>(x: T) -> T {
    x
}

macro_rules! bench_div_sum {
    ($name:ident, $div_fn:expr, $BaseT:ty) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let divisor = $div_fn(DIVISOR as $BaseT);

            const RANGE_START: u128 = 1u128 << (<$BaseT>::BITS / 2 + 1);

            let mut sum: $BaseT = 0;
            b.iter(|| {
                for n in RANGE_START..(RANGE_START + TEST_SIZE) {
                    sum = sum.wrapping_add((n as $BaseT) / divisor);
                }
                black_box(&mut sum);
            });
        }
    };
}

macro_rules! bench_new {
    ($name:ident, $div_fn:expr, $BaseT:ty) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            const RANGE_START: u128 = 1u128 << (<$BaseT>::BITS / 2 + 1);

            let values = (RANGE_START..(RANGE_START + TEST_SIZE))
                .map(|x| {
                    let v = x as $BaseT;
                    if v == 0 { 1 } else { v }
                })
                .collect::<Vec<_>>();

            b.iter(|| {
                for &n in &values {
                    let divisor = $div_fn(n as $BaseT);
                    black_box(divisor);
                }
            });
        }
    };
}

bench_div_sum!(compiler_div_u008, id, u8);
bench_div_sum!(compiler_div_u016, id, u16);
bench_div_sum!(compiler_div_u032, id, u32);
bench_div_sum!(compiler_div_u064, id, u64);
bench_div_sum!(compiler_div_u128, id, u128);

bench_div_sum!(cpu_div_u008, black_box, u8);
bench_div_sum!(cpu_div_u016, black_box, u16);
bench_div_sum!(cpu_div_u032, black_box, u32);
bench_div_sum!(cpu_div_u064, black_box, u64);
bench_div_sum!(cpu_div_u128, black_box, u128);

bench_div_sum!(qd_div_u008, quickdiv::DivisorU8::new, u8);
bench_div_sum!(qd_div_u016, quickdiv::DivisorU16::new, u16);
bench_div_sum!(qd_div_u032, quickdiv::DivisorU32::new, u32);
bench_div_sum!(qd_div_u064, quickdiv::DivisorU64::new, u64);
bench_div_sum!(qd_div_u128, quickdiv::DivisorU128::new, u128);

bench_new!(qd_new_u008, quickdiv::DivisorU8::new, u8);
bench_new!(qd_new_u016, quickdiv::DivisorU16::new, u16);
bench_new!(qd_new_u032, quickdiv::DivisorU32::new, u32);
bench_new!(qd_new_u064, quickdiv::DivisorU64::new, u64);
bench_new!(qd_new_u128, quickdiv::DivisorU128::new, u128);