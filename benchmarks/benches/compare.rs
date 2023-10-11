use std::cell::RefCell;

const BATCH_SIZE: usize = 1000;
/// Divisors correspond to the three different paths of fastdivide and quickdiv.
const DIVISORS: &[u64] = &[7, 8, 9];
const SEED: u64 = 42;

mod new {
    use super::*;

    macro_rules! bench_create_divisor {
        ($name:ident, $new_fn:expr) => {
            #[divan::bench()]
            fn $name(bencher: divan::Bencher) {
                let rng = RefCell::new(fastrand::Rng::with_seed(SEED));

                bencher
                    .counter(divan::counter::ItemsCount::new(BATCH_SIZE))
                    .with_inputs(|| {
                        (0..BATCH_SIZE)
                            .map(|_| rng.borrow_mut().u64(2..u64::MAX))
                            .collect::<Vec<_>>()
                    })
                    .bench_local_refs(|divisors| {
                        for n in divisors {
                            divan::black_box($new_fn(*n));
                        }
                    })
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
                let rng = RefCell::new(fastrand::Rng::with_seed(SEED));

                let d = $new_fn(D);

                bencher
                    .counter(divan::counter::ItemsCount::new(BATCH_SIZE))
                    .with_inputs(|| {
                        (0..BATCH_SIZE)
                            .map(|_| rng.borrow_mut().u64(..))
                            .collect::<Vec<_>>()
                    })
                    .bench_local_refs(|values| {
                        let mut sum: u64 = 0;
                        for value in values {
                            sum = sum.wrapping_add(divan::black_box($div_fn(*value, d)));
                        }
                        divan::black_box(sum);
                    });
            }
        };
    }

    bench_fixed_div_sum!(compiler, |x| x, |n, d| n / d);
    bench_fixed_div_sum!(cpu, divan::black_box, |n, d| n / d);
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
                let rng = RefCell::new(fastrand::Rng::with_seed(SEED));

                bencher
                    .with_inputs(|| {
                        let values = (0..BATCH_SIZE)
                            .map(|_| rng.borrow_mut().u64(..))
                            .collect::<Vec<_>>();

                        let divisors = (0..BATCH_SIZE)
                            .map(|_| rng.borrow_mut().usize(0..DIVISORS.len()))
                            .map(|i: usize| $new_fn(DIVISORS[i]))
                            .collect::<Vec<_>>();
                        (values, divisors)
                    })
                    .bench_local_refs(|(values, divisors)| {
                        let mut sum: u64 = 0;
                        for (&value, &d) in values.iter().zip(divisors.iter()) {
                            sum = sum.wrapping_add(divan::black_box($div_fn(value, d)));
                        }
                        divan::black_box(sum);
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
