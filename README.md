# QuickDiv

[![Latest Release]][crates.io] [![Documentation]][docs.rs] ![Minimum Supported Rust Version 1.54]

[Latest Release]: https://img.shields.io/crates/v/quickdiv.svg
[crates.io]: https://crates.io/crates/quickdiv
[Documentation]: https://docs.rs/quickdiv/badge.svg
[docs.rs]: https://docs.rs/quickdiv/
[Minimum Supported Rust Version 1.54]: https://img.shields.io/badge/MSRV-1.54-blue.svg

QuickDiv is a Rust crate that allows you to speed up repeated division and
modulo operations by the same divisor, based on the
[libdivide C/C++ library](https://libdivide.com/).

On most hardware today integer division operations take longer to execute
compared to operations like multiplication and addition. Because of this,
compilers generally optimize division by a constant, by replacing it with a
cheaper sequence of shifts, multiplications and additions. This crate lets you
apply a similar algorithm to optimize division by values that are only known at
runtime.

Performance gains will vary between platforms, CPUs, and integer widths, but you
can expect dividing an integer by a precomputed divisor to be somewhere between
2 to 10 times faster compared to the built-in hardware division method. Note
that preparing the divisor is more expensive than a single unoptimized
division: it will take at least 2 divisions by the same divisor to break even.

This crate supports primitive integer types of all widths, in both signed and
unsigned variants. It requires Rust version 1.54 or greater. It is `#![no_std]`
and `#![forbid(unsafe_code)]`.

## Example

```rust
use quickdiv::DivisorU64;

fn is_quadratic_residue(q: u64, modulus: u64) -> bool {
    // Initializing a divisor is more expensive than a single
    // unoptimized division, to gain a benefit you must divide
    // multiple times by the same divisor.
    let modulus = DivisorU64::new(modulus);

    // The original value can be recovered by using ::get().
    for x in (0..modulus.get()) {
        // A divisor can be used as the second operand with
        // the / and % operators.
        if (x * x) % modulus == q {
            return true;
        }
    }

    false
}

assert!(is_quadratic_residue(152, 169));
assert!(!is_quadratic_residue(51, 111));
```

## Performance

The following benchmarks should give a rough sense of the kind of speed-up you
can expect. The numbers represent throughput in millions of elements per second
(larger is better) on various tasks.
For `Quotient Sum` we divide a collection of random integers by a fixed
divisor, in `LCG` we compute the remainder given a fixed modulus
and, finally, in `FizzBuzz` we check the divisibility of random integers by a
fixed divisor.

![](https://github.com/dtrifuno/quickdiv/blob/main/benchmarks/graph.png?raw=true)

| Task         | CPU   | Compiler | QuickDiv |
| ------------ | ----- | -------- | -------- |
| Quotient Sum | 248.3 | 838      | 823.5    |
| LCG          | 168.8 | 252.7    | 255      |
| FizzBuzz     | 41.13 | 1350     | 556      |

Note that while QuickDiv computes the remainder and checks if its
zero, the compiler uses a different method to directly check divisibility,
leading to faster performance on the `FizzBuzz` task.

### Caveats

- These results are for `u64` only. Performance can vary with width and
  signedness.
- Benchmarks were run on an AMD Ryzen 5 2600 CPU (i.e. an older x86-64 CPU).
  Some newer high-end processors like the Apple M1/M2 have very fast hardware
  division, and will experience a less dramatic speed-up.
- All tasks involved at least 1000 repeated uses of the same divisor, making
  branch prediction trivial. You will experience worse performance if you are
  instead iterating over a collection of different divisors. If you want to
  read more about this, check out
  [Paul Khuong's post](https://pvk.ca/Blog/2021/05/14/baseline-implementations-should-be-predictable/)
  about his branchfree Rust integer division library
  [Reciprocal](https://crates.io/crates/reciprocal).

If you would like to run these benchmarks yourself, check out the [`benchmarks`
crate](https://github.com/dtrifuno/quickdiv/tree/main/benchmarks) in the GitHub
repository.

## License

Licensed under any of:

- Apache License, Version 2.0, ([LICENSE-APACHE](https://raw.githubusercontent.com/dtrifuno/quickdiv/main/LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](https://raw.githubusercontent.com/dtrifuno/quickdiv/main/LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
- zlib License ([LICENSE-ZLIB](https://raw.githubusercontent.com/dtrifuno/quickdiv/main/LICENSE-ZLIB) or <https://opensource.org/license/zlib/>)

by your choice.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be multi-licensed as above, without any additional terms or conditions.
