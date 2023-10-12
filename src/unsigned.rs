macro_rules! declare_unsigned_structs {
    ($SelfT:ident, $InnerT:ident, $BaseT:ty, $SIZE:literal) => {
        /// Faster divisor for division and modulo operations by
        #[doc = concat!($SIZE)]
        /// unsigned integer values.
        #[derive(Clone, Copy)]
        pub struct $SelfT {
            inner: $InnerT,
        }

        #[derive(Clone, Copy)]
        enum $InnerT {
            Shift($BaseT, u8),
            MultiplyShift($BaseT, $BaseT, u8),
            MultiplyAddShift($BaseT, $BaseT, u8),
        }
    };
}

macro_rules! unsigned_impl {
    ($SelfT:ident, $InnerT:ident, $BaseT:ty) => {
        /// Creates a divisor which can be used for faster computation of division and modulo by `d`.
        ///
        /// # Panics
        ///
        /// Panics if `d` equals zero.
        ///
        /// # Examples
        /// ```
        #[doc = concat!("use quickdiv::", stringify!($SelfT), ";")]
        ///
        #[doc = concat!("let d = ", stringify!($SelfT), "::new(42);")]
        /// ```
        pub const fn new(d: $BaseT) -> $SelfT {
            // Forces a panic when d = 0, since we cannot use panic! in const.
            let _ = 1 / d;

            let shift = Self::ilog2(d);

            let inner = if d.is_power_of_two() {
                <$InnerT>::Shift(d, shift)
            } else {
                let (mut magic, rem) = Self::div_rem_wide_by_base(1 << shift, d);

                let e = d - rem;

                if e < 1 << shift {
                    <$InnerT>::MultiplyShift(d, magic + 1, shift)
                } else {
                    magic = magic.wrapping_mul(2);
                    let (doubled_rem, overflowed) = rem.overflowing_mul(2);
                    if doubled_rem >= d || overflowed {
                        magic += 1;
                    }

                    <$InnerT>::MultiplyAddShift(d, magic + 1, shift)
                }
            };

            Self { inner }
        }

        /// Returns the value that was used to construct this divisor as a primitive type.
        ///
        /// # Examples
        /// ```
        #[doc = concat!("use quickdiv::", stringify!($SelfT), ";")]
        ///
        #[doc = concat!("let d = ", stringify!($SelfT), "::new(7);")]
        /// assert_eq!(d.get(), 7);
        /// ```
        #[inline]
        pub const fn get(&self) -> $BaseT {
            match self.inner {
                $InnerT::Shift(d, _) => d,
                $InnerT::MultiplyShift(d, _, _) => d,
                $InnerT::MultiplyAddShift(d, _, _) => d,
            }
        }

        /// Returns `true` if `n` is divisible by `self`.
        ///
        /// We take `0` to be divisible by all non-zero numbers.
        ///
        /// # Examples
        /// ```
        #[doc = concat!("use quickdiv::", stringify!($SelfT), ";")]
        ///
        #[doc = concat!("let d = ", stringify!($SelfT), "::new(17);")]
        /// assert!(d.divides(34));
        /// ```
        #[inline]
        pub const fn divides(&self, n: $BaseT) -> bool {
            self.rem_of(n) == 0
        }

        /// Returns the remainder of dividing `n` by `self`.
        ///
        /// # Examples
        /// ```
        #[doc = concat!("use quickdiv::", stringify!($SelfT), ";")]
        ///
        #[doc = concat!("let d = ", stringify!($SelfT), "::new(11);")]
        /// let rem = d.rem_of(30);
        /// assert_eq!(rem, 8);
        /// ```
        #[inline]
        pub const fn rem_of(&self, n: $BaseT) -> $BaseT {
            n - self.get() * self.div_of(n)
        }

        /// Returns the result of dividing `n` by `self`.
        ///
        /// # Examples
        /// ```
        #[doc = concat!("use quickdiv::", stringify!($SelfT), ";")]
        ///
        #[doc = concat!("let d = ", stringify!($SelfT), "::new(17);")]
        /// let div = d.div_of(34);
        /// assert_eq!(div, 2);
        #[inline]
        pub const fn div_of(&self, n: $BaseT) -> $BaseT {
            match self.inner {
                $InnerT::Shift(_, shift) => n >> shift,
                $InnerT::MultiplyShift(_, magic, shift) => $SelfT::mulh(magic, n) >> shift,
                $InnerT::MultiplyAddShift(_, magic, shift) => {
                    let q = $SelfT::mulh(magic, n);
                    let t = ((n - q) >> 1) + q;
                    t >> shift
                }
            }
        }

        // We have to implement our own const ilog2 to get MSRV below 1.67.
        const fn ilog2(n: $BaseT) -> u8 {
            (<$BaseT>::BITS - 1 - n.leading_zeros()) as u8
        }
    };
}
