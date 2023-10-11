macro_rules! widen_mulh_impl {
    ($BaseT:ty, $WiderT:ty) => {
        /// Multiply two words together, returning only the top half of the product.
        ///
        /// Works by extending the factors to 2N-bits, using the built-in 2N-by-2N-bit
        /// multiplication and shifting right to the top half only.
        #[inline]
        const fn mulh(x: $BaseT, y: $BaseT) -> $BaseT {
            (((x as $WiderT) * (y as $WiderT)) >> <$BaseT>::BITS) as $BaseT
        }
    };
}

macro_rules! mulh_impl {
    ($BaseT:ty) => {
        /// Multiply two words together, returning only the top half of the product.
        ///
        /// Adapted from Figure 8-2 in Hacker's Delight, 2nd Ed.
        const fn mulh(x: $BaseT, y: $BaseT) -> $BaseT {
            const HALF_WIDTH_BITS: u32 = <$BaseT>::BITS / 2;
            const LOWER_HALF_MASK: $BaseT = (1 << HALF_WIDTH_BITS) - 1;

            let x_low = x & LOWER_HALF_MASK;
            let y_low = y & LOWER_HALF_MASK;
            let t = x_low.wrapping_mul(y_low);
            let k = t >> HALF_WIDTH_BITS;

            let x_high = x >> HALF_WIDTH_BITS;
            let t = x_high.wrapping_mul(y_low) + k;
            let k = t & LOWER_HALF_MASK;
            let w1 = t >> HALF_WIDTH_BITS;

            let y_high = y >> HALF_WIDTH_BITS;
            let t = x_low.wrapping_mul(y_high) + k;
            let k = t >> HALF_WIDTH_BITS;

            x_high.wrapping_mul(y_high) + w1 + k
        }
    };
}

macro_rules! widen_div_rem_impl {
    ($BaseT:ty, $WiderT:ty) => {
        /// Divide a 2N-bit dividend by an N-bit divisor with remainder, assuming
        /// that the result fits into N bits and that the lower half of bits of the
        /// dividend are all zero.
        ///
        /// Works by extending the dividend to 2N-bits and then using the built-in
        /// 2N-by-2N-bit division method.
        const fn div_rem_wide_by_base(top_half: $BaseT, d: $BaseT) -> ($BaseT, $BaseT) {
            let n = (top_half as $WiderT) << <$BaseT>::BITS;
            let quot = (n / (d as $WiderT)) as $BaseT;
            let rem = (n % (d as $WiderT)) as $BaseT;
            (quot, rem)
        }
    };
}

macro_rules! divlu_impl {
    ($BaseT:ty) => {
        /// Divide a 2N-bit dividend by an N-bit divisor with remainder, assuming
        /// that the result fits into N bits and that the lower half of bits of the
        /// dividend are all zero.
        ///
        /// Adapted from Figure 9-3 in Hacker's Delight, 2nd Ed.
        const fn div_rem_wide_by_base(top_half: $BaseT, d: $BaseT) -> ($BaseT, $BaseT) {
            const HALF_WORD_BITS: u32 = <$BaseT>::BITS / 2;

            const BASE: $BaseT = 1 << HALF_WORD_BITS;

            let s = d.leading_zeros();
            let v = d << s;
            let vn1 = v >> HALF_WORD_BITS;
            let vn0 = v & (BASE - 1);

            let un32 = top_half << s;

            let mut q1 = un32 / vn1;
            let mut rhat = un32 - q1 * vn1;

            loop {
                if q1 >= BASE || q1 * vn0 > (rhat << HALF_WORD_BITS) {
                    q1 -= 1;
                    rhat += vn1;

                    if rhat < BASE {
                        continue;
                    }
                }
                break;
            }

            let un21 = (un32 << HALF_WORD_BITS).wrapping_sub(q1.wrapping_mul(v));

            let mut q0 = un21 / vn1;
            rhat = un21 - q0 * vn1;

            loop {
                if q0 >= BASE || q0 * vn0 > (rhat << HALF_WORD_BITS) {
                    q0 -= 1;
                    rhat += vn1;

                    if rhat < BASE {
                        continue;
                    }
                }
                break;
            }

            let r = ((un21 << HALF_WORD_BITS).wrapping_sub(q0.wrapping_mul(v))) >> s;

            ((q1 << HALF_WORD_BITS) + q0, r)
        }
    };
}
