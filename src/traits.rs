macro_rules! impl_traits {
    ($SelfT:ty, $BaseT:ty) => {
        impl PartialEq for $SelfT {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.get() == other.get()
            }
        }

        impl Eq for $SelfT {}

        impl core::hash::Hash for $SelfT {
            #[inline]
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.get().hash(state);
            }
        }

        impl core::fmt::Debug for $SelfT {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}", self.get())
            }
        }

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
