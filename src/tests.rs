macro_rules! tests {
    ($SelfT:ty, $BaseT:ident) => {
        #[cfg(test)]
        mod $BaseT {
            use quickcheck::*;

            use super::*;

            impl Arbitrary for $SelfT {
                fn arbitrary(g: &mut Gen) -> $SelfT {
                    let mut d = $BaseT::arbitrary(g);
                    d = if d == 0 { 1 } else { d };

                    <$SelfT>::new(d)
                }
            }

            #[test]
            #[should_panic]
            fn cannot_create_zero_divisors() {
                let _ = <$SelfT>::new(0);
            }

            quickcheck! {
                fn is_multiple_of_five(x: $BaseT) -> bool {
                    let divisor = <$SelfT>::new(5);
                    divisor.divides(x) == (x % 5 == 0)
                }
            }

            quickcheck! {
                fn anything_divides_zero(d: $SelfT) -> bool {
                    d.divides(0)
                }
            }

            quickcheck! {
                fn div_agrees_with_builtin(x: $BaseT, d: $SelfT) -> bool {
                    x / d == x.wrapping_div(d.get())
                }
            }

            quickcheck! {
                fn div_by_one_is_id(x: $BaseT) -> bool {
                    let divisor = <$SelfT>::new(1);
                    x / divisor == x
                }
            }

            quickcheck! {
                fn rem_agrees_with_builtin(x: $BaseT, d: $SelfT) -> bool {
                    x % d == x.wrapping_rem(d.get())
                }
            }

            quickcheck! {
                fn rem_one_is_zero(x: $BaseT) -> bool {
                    let divisor = <$SelfT>::new(1);
                    x % divisor == 0
                }
            }

            quickcheck! {
                fn euclidean_algorithm(x: $BaseT, d: $SelfT) -> bool {
                    d.get().wrapping_mul(x / d) + (x % d) == x
                }
            }
        }
    };
}
