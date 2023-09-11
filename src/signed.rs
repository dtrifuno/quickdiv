macro_rules! declare_signed_structs {
    ($SelfT:ident, $InnerT:ident, $BaseT:ty, $SIZE:literal) => {
        /// Faster divisor for division and modulo operations by
        #[doc = concat!($SIZE)]
        /// signed integer values.

        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub struct $SelfT {
            inner: $InnerT,
        }

        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        enum $InnerT {
            Shift($BaseT, u8),
            ShiftAndNegate($BaseT, u8),
            MultiplyShift($BaseT, $BaseT, u8),
            MultiplyAddShift($BaseT, $BaseT, u8),
            MultiplyAddShiftNegate($BaseT, $BaseT, u8),
        }
    };
}

macro_rules! signed_impl {
    ($SelfT:ident, $InnerT:ident, $BaseT:ty, $UnsignedBaseT:ty) => {
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
        #[doc = concat!("let d = ", stringify!($SelfT), "::new(-21);")]
        /// ```
        pub const fn new(d: $BaseT) -> $SelfT {
            // Forces a panic when d = 0, since we cannot use panic! in const.
            let _ = 1 / d;

            let ud = Self::abs(d);

            let shift = Self::ilog2(ud);

            let inner = if ud.is_power_of_two() {
                if d > 0 {
                    <$InnerT>::Shift(d, shift)
                } else {
                    <$InnerT>::ShiftAndNegate(d, shift)
                }
            } else {
                let (mut magic, rem) = Self::div_rem_wide_by_base(1 << (shift - 1), ud);

                let e = ud - rem;

                if e < 1 << shift {
                    <$InnerT>::MultiplyShift(d, d.signum() * (magic as $BaseT + 1), shift - 1)
                } else {
                    magic *= 2;
                    let (doubled_rem, overflowed) = rem.overflowing_mul(2);
                    if doubled_rem >= ud || overflowed {
                        magic += 1;
                    }

                    magic += 1;
                    if d > 0 {
                        <$InnerT>::MultiplyAddShift(d, magic as $BaseT, shift)
                    } else {
                        <$InnerT>::MultiplyAddShiftNegate(d, -(magic as $BaseT), shift)
                    }
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
        #[doc = concat!("let d = ", stringify!($SelfT), "::new(-15);")]
        /// assert_eq!(d.get(), -15);
        /// ```
        #[inline]
        pub const fn get(&self) -> $BaseT {
            match self.inner {
                $InnerT::Shift(d, _) => d,
                $InnerT::ShiftAndNegate(d, _) => d,
                $InnerT::MultiplyShift(d, _, _) => d,
                $InnerT::MultiplyAddShift(d, _, _) => d,
                $InnerT::MultiplyAddShiftNegate(d, _, _) => d,
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
        #[doc = concat!("let d = ", stringify!($SelfT), "::new(-9);")]
        /// assert!(d.divides(27));
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
        #[doc = concat!("let d = ", stringify!($SelfT), "::new(21);")]
        /// let rem = d.rem_of(-30);
        /// assert_eq!(rem, -9);
        /// ```
        #[inline]
        pub const fn rem_of(&self, n: $BaseT) -> $BaseT {
            n.wrapping_add((self.get().wrapping_mul(self.div_of(n))).wrapping_mul(-1))
        }


        /// Returns the result of dividing `n` by `self`.
        ///
        /// This will perform a wrapping division, i.e.
        #[doc = concat!("`", stringify!($SelfT), "::new(-1).div_of(", stringify!($BaseT) ,"::MIN)`")]
        /// will always silently return
        #[doc = concat!("`", stringify!($BaseT) ,"::MIN`")]
        /// no matter whether the program was compiled with `overflow-checks` turned off or not.
        ///
        /// # Examples
        /// ```
        #[doc = concat!("use quickdiv::", stringify!($SelfT), ";")]
        ///
        #[doc = concat!("let d = ", stringify!($SelfT), "::new(13);")]
        /// let div = d.div_of(-30);
        /// assert_eq!(div, -2);
        #[inline]
        pub const fn div_of(&self, n: $BaseT) -> $BaseT {
            match self.inner {
                $InnerT::Shift(_, shift) => {
                    let mask = (1 as $BaseT << shift).wrapping_sub(1);
                    let b = (n >> (<$BaseT>::BITS - 1)) & mask;
                    n.wrapping_add(b) >> shift
                },
                $InnerT::ShiftAndNegate(_, shift) => {
                    let mask = (1 as $BaseT << shift).wrapping_sub(1);
                    let b = (n >> (<$BaseT>::BITS - 1)) & mask;
                    let t = n.wrapping_add(b) >> shift;
                    t.wrapping_mul(-1)
                },
                $InnerT::MultiplyShift(_, magic, shift) => {
                    let q = $SelfT::mulh(magic, n) >> shift;
                    if q < 0 {
                        q + 1
                    } else {
                        q
                    }
                },
                $InnerT::MultiplyAddShift(_, magic, shift) => {
                    let q = $SelfT::mulh(magic, n);
                    let t = q.wrapping_add(n) >> shift;
                    if t < 0 {
                        t + 1
                    } else {
                        t
                    }
                },
                $InnerT::MultiplyAddShiftNegate(_, magic, shift) => {
                    let q = $SelfT::mulh(magic, n);
                    let t = q.wrapping_add(n.wrapping_mul(-1)) >> shift;
                    if t < 0 {
                        t + 1
                    } else {
                        t
                    }
                }
            }
        }

        #[inline]
        const fn abs(n: $BaseT) -> $UnsignedBaseT {
            if n < 0 {
                ((-1i8) as $UnsignedBaseT).wrapping_mul(n as $UnsignedBaseT)
            } else {
                n as $UnsignedBaseT
            }
        }

        // We have to implement our own const ilog2 to get MSRV below 1.67.
        #[inline]
        const fn ilog2(n: $UnsignedBaseT) -> u8 {
            (<$UnsignedBaseT>::BITS - 1 - n.leading_zeros()) as u8
        }
    };
}
