use std::iter::repeat_with;

const BATCH_SIZE: usize = 1000;
/// Divisors correspond to the three different paths of fastdivide and quickdiv.
const DIVISORS: &[u64] = &[7, 8, 9];
const SEED: u64 = 42;

mod new {
    use super::*;

    macro_rules! bench_create_divisor {
        ($name:ident, $new_fn:expr) => {
            #[divan::bench(sample_count = 1000)]
            fn $name(bencher: divan::Bencher) {
                let mut rng = fastrand::Rng::with_seed(SEED);
                bencher
                    .counter(1u32)
                    .with_inputs(|| rng.u64(2..))
                    .bench_local_values(|n| $new_fn(n));
            }
        };
    }

    bench_create_divisor!(reciprocal, |d| reciprocal::Reciprocal::new(d).unwrap());
    bench_create_divisor!(partial_reciprocal, |d| {
        reciprocal::PartialReciprocal::new(d).unwrap()
    });
    bench_create_divisor!(fastdivide, fastdivide::DividerU64::divide_by);
    bench_create_divisor!(strength_reduce, strength_reduce::StrengthReducedU64::new);
    bench_create_divisor!(quickdiv, quickdiv::DivisorU64::new);
}

mod fixed_div_sum {
    use super::*;

    macro_rules! bench_fixed_div_sum {
        ($name:ident, $new_fn:expr, $div_fn:expr) => {
            #[divan::bench(consts = DIVISORS, sample_size = 1000)]
            fn $name<const D: u64>(bencher: divan::Bencher) {
                let mut rng = fastrand::Rng::with_seed(SEED);
                let d = $new_fn(D);

                bencher
                    .counter(BATCH_SIZE)
                    .with_inputs(|| repeat_with(|| rng.u64(2..)).take(BATCH_SIZE).collect())
                    .bench_local_refs(|values: &mut Vec<u64>| {
                        values.iter().fold(0u64, |acc, &x| {
                            acc.wrapping_add(divan::black_box($div_fn(x, d)))
                        })
                    });
            }
        };
    }

    bench_fixed_div_sum!(compiler, |x| x, |n, d| n / d);
    bench_fixed_div_sum!(cpu, divan::black_box, |n, d| n / divan::black_box(d));
    bench_fixed_div_sum!(
        fastdivide,
        fastdivide::DividerU64::divide_by,
        |n, d: fastdivide::DividerU64| d.divide(n)
    );
    bench_fixed_div_sum!(
        reciprocal,
        |d| reciprocal::Reciprocal::new(d).unwrap(),
        |n, d: reciprocal::Reciprocal| d.apply(n)
    );
    bench_fixed_div_sum!(
        partial_reciprocal,
        |d| reciprocal::PartialReciprocal::new(d).unwrap(),
        |n, d: reciprocal::PartialReciprocal| d.apply(n)
    );
    bench_fixed_div_sum!(
        strength_reduce,
        strength_reduce::StrengthReducedU64::new,
        |n, d| n / d
    );
    bench_fixed_div_sum!(quickdiv, quickdiv::DivisorU64::new, |n, d| n / d);
}

mod random_div_sum {
    use super::*;

    macro_rules! bench_random_div_sum {
        ($name:ident, $new_fn:expr, $div_fn:expr) => {
            #[divan::bench()]
            fn $name(bencher: divan::Bencher) {
                let mut rng = fastrand::Rng::with_seed(SEED);

                bencher
                    .with_inputs(|| -> (Vec<_>, Vec<_>) {
                        let values = repeat_with(|| rng.u64(..)).take(BATCH_SIZE).collect();
                        let divisors = repeat_with(|| rng.usize(..DIVISORS.len()))
                            .map(|i| $new_fn(DIVISORS[i]))
                            .take(BATCH_SIZE)
                            .collect();
                        (values, divisors)
                    })
                    .bench_local_refs(|(values, divisors)| {
                        values
                            .iter()
                            .zip(divisors.iter())
                            .fold(0u64, |acc, (&x, &d)| {
                                acc.wrapping_add(divan::black_box($div_fn(x, d)))
                            })
                    });
            }
        };
    }

    bench_random_div_sum!(cpu, divan::black_box, |n, d| n / d);
    bench_random_div_sum!(
        fastdivide,
        fastdivide::DividerU64::divide_by,
        |n, d: fastdivide::DividerU64| d.divide(n)
    );
    bench_random_div_sum!(
        reciprocal,
        |d| reciprocal::Reciprocal::new(d).unwrap(),
        |n, d: reciprocal::Reciprocal| d.apply(n)
    );
    bench_random_div_sum!(
        partial_reciprocal,
        |d| reciprocal::PartialReciprocal::new(d).unwrap(),
        |n, d: reciprocal::PartialReciprocal| d.apply(n)
    );
    bench_random_div_sum!(
        strength_reduce,
        strength_reduce::StrengthReducedU64::new,
        |n, d| n / d
    );
    bench_random_div_sum!(quickdiv, quickdiv::DivisorU64::new, |n, d| n / d);
}

fn main() {
    divan::main()
}
