macro_rules! ops_impl {
    ($SelfT:ty, $BaseT:ty) => {
        impl core::ops::Div<$SelfT> for $BaseT {
            type Output = $BaseT;

            #[inline]
            fn div(self, rhs: $SelfT) -> Self::Output {
                rhs.div_of(self)
            }
        }

        impl core::ops::DivAssign<$SelfT> for $BaseT {
            #[inline]
            fn div_assign(&mut self, rhs: $SelfT) {
                *self = rhs.div_of(*self)
            }
        }

        impl core::ops::Rem<$SelfT> for $BaseT {
            type Output = $BaseT;

            #[inline]
            fn rem(self, rhs: $SelfT) -> Self::Output {
                rhs.rem_of(self)
            }
        }

        impl core::ops::RemAssign<$SelfT> for $BaseT {
            #[inline]
            fn rem_assign(&mut self, rhs: $SelfT) {
                *self = rhs.rem_of(*self)
            }
        }
    };
}
