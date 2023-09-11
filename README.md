# QuickDiv

[![Latest Release]][crates.io] [![Documentation]][docs.rs] ![Minimum Supported Rust Version 1.54]

[Latest Release]: https://img.shields.io/crates/v/quickdiv.svg
[crates.io]: https://crates.io/crates/quickdiv
[Documentation]: https://docs.rs/quickdiv/badge.svg
[docs.rs]: https://docs.rs/quickdiv/
[Minimum Supported Rust Version 1.54]: https://img.shields.io/badge/MSRV-1.54-blue.svg

QuickDiv is a Rust crate that allows you to speed up repeated division and modulo operations by the same divisor,
based on the [libdivide C/C++ library](https://libdivide.com/).

On most hardware today integer division operations take longer to execute compared to operations
like multiplication and addition. Because of this, compilers generally optimize division by a constant, by replacing
it with a cheaper sequence of shifts, multiplications and additions. This crate lets you apply a similar
algorithm to optimize division by values that are only known at runtime.

Performance gains will vary between platforms, CPUs, and integer widths, but you can expect dividing an integer
by a precomputed divisor to be somewhere between 2 to 10 times faster compared to the built-in hardware division
method. Note that preparing the divisor is more expensive than a single unoptimized division: it will take at
least 2 divisions by the same divisor to break even.

This crate supports primitive integer types of all widths, in both signed and unsigned variants.
It requires Rust version 1.54 or greater. It is `#![no_std]` and `#![forbid(unsafe_code)]`.

## Example

```rust
use quickdiv::DivisorU64;

fn is_quadratic_residue(q: u64, modulus: u64) -> bool {
    // Initializing a divisor is more expensive than a single unoptimized
    // division, to gain benefit you must divide multiple times by same divisor.
    let modulus = DivisorU64::new(modulus);

    // The original value can be recovered using ::get().
    for x in (0..modulus.get()) {
        // A divisor can be used as the right-hand side operand with
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
