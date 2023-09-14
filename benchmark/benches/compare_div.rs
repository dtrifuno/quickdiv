use criterion::*;

const SAMPLES: usize = 1000;

const DIVISOR: u64 = 7;

const SEED: u64 = 314;
const MULTIPLIER: u64 = 0x5851f42d4c957f2d;
const INCREMENT: u64 = 0x14057b7ef767814f;

fn lcg_values(size: usize) -> Vec<u64> {
    let mut values = Vec::with_capacity(size);

    let mut prev = SEED;
    for _ in 0..size {
        let value = prev.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT);
        values.push(value);
        prev = value;
    }

    values
}

fn compiler_div_u064<const D: u64>(c: &mut Criterion) {
    let values = black_box(lcg_values(SAMPLES));

    c.bench_function("compiler_div_u064", |b| {
        b.iter(|| {
            let mut sum: u64 = 0;
            for &n in &values {
                sum = sum.wrapping_add(n / D)
            }

            black_box(sum)
        })
    });
}

fn cpu_div_u064<const D: u64>(c: &mut Criterion) {
    let values = black_box(lcg_values(SAMPLES));
    let d = black_box(D);

    c.bench_function("cpu_div_u064", |b| {
        b.iter(|| {
            let mut sum: u64 = 0;
            for &n in &values {
                sum = sum.wrapping_add(n / d)
            }

            black_box(sum)
        })
    });
}

fn fastdivide_div_u064<const D: u64>(c: &mut Criterion) {
    let values = black_box(lcg_values(SAMPLES));

    let d = fastdivide::DividerU64::divide_by(D);

    c.bench_function("fastdivide_div_u064", |b| {
        b.iter(|| {
            let mut sum: u64 = 0;
            for &n in &values {
                sum = sum.wrapping_add(d.divide(n))
            }

            black_box(sum)
        })
    });
}

fn reciprocal_div_u064<const D: u64>(c: &mut Criterion) {
    let values = black_box(lcg_values(SAMPLES));

    let d = reciprocal::Reciprocal::new(D).unwrap();

    c.bench_function("reciprocal_div_u064", |b| {
        b.iter(|| {
            let mut sum: u64 = 0;
            for &n in &values {
                sum = sum.wrapping_add(d.apply(n));
            }

            black_box(sum)
        })
    });
}

fn strength_reduce_div_u064<const D: u64>(c: &mut Criterion) {
    let values = black_box(lcg_values(SAMPLES));

    let d = strength_reduce::StrengthReducedU64::new(D);

    c.bench_function("strength_reduce_div_u064", |b| {
        b.iter(|| {
            let mut sum: u64 = 0;
            for &n in &values {
                sum = sum.wrapping_add(n / d)
            }

            black_box(sum)
        })
    });
}


fn quickdiv_div_u064<const D: u64>(c: &mut Criterion) {
    let values = black_box(lcg_values(SAMPLES));

    let d = quickdiv::DivisorU64::new(D);

    c.bench_function("quickdiv_div_u064", |b| {
        b.iter(|| {
            let mut sum: u64 = 0;
            for &n in &values {
                sum = sum.wrapping_add(n / d)
            }

            black_box(sum)
        })
    });
}


criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = compiler_div_u064::<DIVISOR>, quickdiv_div_u064::<DIVISOR>, reciprocal_div_u064::<DIVISOR>, fastdivide_div_u064::<DIVISOR>, strength_reduce_div_u064::<DIVISOR>, cpu_div_u064::<DIVISOR>
}

criterion_main!(benches);