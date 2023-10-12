#![no_std]
#![forbid(unsafe_code)]

//! QuickDiv is a Rust crate that allows you to speed up repeated division and
//! modulo operations by the same divisor, based on the
//! [libdivide C/C++ library](https://libdivide.com/).
//!
//! On most hardware today integer division operations take longer to execute
//! compared to operations like multiplication and addition. Because of this,
//! compilers generally optimize division by a constant, by replacing it with a
//! cheaper sequence of shifts, multiplications and additions. This crate lets you
//! apply a similar algorithm to optimize division by values that are only known at
//! runtime.
//!
//! Performance gains will vary between platforms, CPUs, and integer widths, but you
//! can expect dividing an integer by a precomputed divisor to be somewhere between
//! 2 to 10 times faster compared to the built-in hardware division method. Note
//! that preparing the divisor is more expensive than a single unoptimized
//! division: it will take at least 2 divisions by the same divisor to break even.
//!
// This crate supports primitive integer types of all widths, in both signed and
// unsigned variants. It requires Rust version 1.54 or greater. It is `#![no_std]`
// and `#![forbid(unsafe_code)]`.
//!
//! # Example
//!
//! ```rust
//! use quickdiv::DivisorU64;
//!
//! fn is_quadratic_residue(q: u64, modulus: u64) -> bool {
//!     // Initializing a divisor is more expensive than a single
//!     // unoptimized division, to gain a benefit you must divide
//!     // multiple times by the same divisor.
//!     let modulus = DivisorU64::new(modulus);
//!
//!     // The original value can be recovered by using ::get().
//!     for x in (0..modulus.get()) {
//!         // A divisor can be used as the second operand with
//!         // the / and % operators.
//!         if (x * x) % modulus == q {
//!             return true;
//!         }
//!     }
//!
//!     false
//! }
//!
//! assert!(is_quadratic_residue(152, 169));
//! assert!(!is_quadratic_residue(51, 111));
//! ```

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctest {}

extern crate core;

#[macro_use] // import impl_traits!
mod traits;
#[macro_use] // import declare_signed_structs!, signed_impl!
mod signed;
#[macro_use] // import tests!
mod tests;
#[macro_use] // import declare_unsigned_structs!, unsigned_impl!
mod unsigned;
#[macro_use] // import widen_mulh_impl!, mulh_impl!, widen_div_rem_impl!, divlu_impl!
mod utils;

// DivisorU8

declare_unsigned_structs! { DivisorU8, InnerDivisorU8, u8, "8-bit" }

impl DivisorU8 {
    unsigned_impl! { DivisorU8, InnerDivisorU8, u8 }
    widen_mulh_impl! { u8, u16 }
    widen_div_rem_impl! { u8, u16 }
}

impl_traits! { DivisorU8, u8 }

tests! { DivisorU8, u8 }

// DivisorU16

declare_unsigned_structs! { DivisorU16, InnerDivisorU16, u16, "16-bit" }

impl DivisorU16 {
    unsigned_impl! { DivisorU16, InnerDivisorU16, u16 }
    widen_mulh_impl! { u16, u32 }
    widen_div_rem_impl! { u16, u32 }
}

impl_traits! { DivisorU16, u16 }

tests! { DivisorU16, u16 }

// DivisorU32

declare_unsigned_structs! { DivisorU32, InnerDivisorU32, u32, "32-bit" }

impl DivisorU32 {
    unsigned_impl! { DivisorU32, InnerDivisorU32, u32 }
    widen_mulh_impl! { u32, u64 }
    widen_div_rem_impl! { u32, u64 }
}

impl_traits! { DivisorU32, u32 }

tests! { DivisorU32, u32 }

// DivisorU64

declare_unsigned_structs! { DivisorU64, InnerDivisorU64, u64, "64-bit" }

impl DivisorU64 {
    unsigned_impl! { DivisorU64, InnerDivisorU64, u64 }
}

#[cfg(not(target_pointer_width = "64"))]
impl DivisorU64 {
    divlu_impl! { u64 }
    mulh_impl! { u64 }
}

#[cfg(target_pointer_width = "64")]
impl DivisorU64 {
    widen_div_rem_impl! { u64, u128 }
    widen_mulh_impl! { u64, u128 }
}

impl_traits! { DivisorU64, u64 }

tests! { DivisorU64, u64 }

// DivisorU128

declare_unsigned_structs! { DivisorU128, InnerDivisorU128, u128, "128-bit" }

impl DivisorU128 {
    unsigned_impl! { DivisorU128, InnerDivisorU128, u128 }
    mulh_impl! { u128 }
    divlu_impl! { u128 }
}

impl_traits! { DivisorU128, u128 }

tests! { DivisorU128, u128 }

// DivisorUsize

declare_unsigned_structs! { DivisorUsize, InnerDivisorUsize, usize, "pointer-sized" }

impl DivisorUsize {
    unsigned_impl! { DivisorUsize, InnerDivisorUsize, usize }
}

#[cfg(target_pointer_width = "16")]
impl DivisorUsize {
    widen_mulh_impl! { usize, u32 }
    widen_div_rem_impl! { usize, u32 }
}

#[cfg(target_pointer_width = "32")]
impl DivisorUsize {
    widen_mulh_impl! { usize, u64 }
    widen_div_rem_impl! { usize, u64 }
}

#[cfg(target_pointer_width = "64")]
impl DivisorUsize {
    widen_mulh_impl! { usize, u128 }
    widen_div_rem_impl! { usize, u128 }
}

impl_traits! { DivisorUsize, usize }

// DivisorI8

declare_signed_structs! { DivisorI8, InnerDivisorI8, i8, "8-bit" }

impl DivisorI8 {
    signed_impl! { DivisorI8, InnerDivisorI8, i8, u8 }
    widen_mulh_impl! { i8, i16 }
    widen_div_rem_impl! { u8, u16 }
}

impl_traits! { DivisorI8, i8 }

tests! { DivisorI8, i8 }

// DivisorI16

declare_signed_structs! { DivisorI16, InnerDivisorI16, i16, "16-bit" }

impl DivisorI16 {
    signed_impl! { DivisorI16, InnerDivisorI16, i16, u16 }
    widen_mulh_impl! { i16, i32 }
    widen_div_rem_impl! { u16, u32 }
}

impl_traits! { DivisorI16, i16 }

tests! { DivisorI16, i16 }

// DivisorI32

declare_signed_structs! { DivisorI32, InnerDivisorI32, i32, "32-bit" }

impl DivisorI32 {
    signed_impl! { DivisorI32, InnerDivisorI32, i32, u32 }
    widen_mulh_impl! { i32, i64 }
    widen_div_rem_impl! { u32, u64 }
}

impl_traits! { DivisorI32, i32 }

tests! { DivisorI32, i32 }

// DivisorI64

declare_signed_structs! { DivisorI64, InnerDivisorI64, i64, "64-bit" }

impl DivisorI64 {
    signed_impl! { DivisorI64, InnerDivisorI64, i64, u64 }
}

#[cfg(not(target_pointer_width = "64"))]
impl DivisorI64 {
    mulh_impl! { i64 }
    divlu_impl! { u64 }
}

#[cfg(target_pointer_width = "64")]
impl DivisorI64 {
    widen_mulh_impl! { i64, i128 }
    widen_div_rem_impl! { u64, u128 }
}

impl_traits! { DivisorI64, i64 }

tests! { DivisorI64, i64 }

// DivisorI128

declare_signed_structs! { DivisorI128, InnerDivisorI128, i128, "128-bit" }

impl DivisorI128 {
    signed_impl! { DivisorI128, InnerDivisorI128, i128, u128 }
    mulh_impl! { i128 }
    divlu_impl! { u128 }
}

impl_traits! { DivisorI128, i128 }

tests! { DivisorI128, i128 }

// DivisorIsize

declare_signed_structs! { DivisorIsize, InnerDivisorIsize, isize, "pointer-sized" }

impl DivisorIsize {
    signed_impl! { DivisorIsize, InnerDivisorIsize, isize, usize }
}

#[cfg(target_pointer_width = "16")]
impl DivisorIsize {
    widen_mulh_impl! { isize, i32 }
    widen_div_rem_impl! { usize, u32 }
}

#[cfg(target_pointer_width = "32")]
impl DivisorIsize {
    widen_mulh_impl! { isize, i64 }
    widen_div_rem_impl! { usize, u64 }
}

#[cfg(target_pointer_width = "64")]
impl DivisorIsize {
    widen_mulh_impl! { isize, i128 }
    widen_div_rem_impl! { usize, u128 }
}

impl_traits! { DivisorIsize, isize }
