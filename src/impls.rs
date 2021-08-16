use core::{
    iter::{Product, Sum},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Neg, Not, Sub, SubAssign,
    },
};

use num_traits::{Inv, One, Pow, Zero};

use crate::GF;

// implements the unary operator "op &T"
// based on "op T" where T is expected to be `Copy`able
macro_rules! forward_ref_unop {
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl $imp for &$t {
            type Output = <$t as $imp>::Output;

            #[inline]
            fn $method(self) -> <$t as $imp>::Output {
                $imp::$method(*self)
            }
        }
    };
}

// implements binary operators "&T op U", "T op &U", "&T op &U"
// based on "T op U" where T and U are expected to be `Copy`able
macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl<'a> $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl $imp<&$u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl $imp<&$u> for &$t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &$u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    };
}

// implements "T op= &U", based on "T op= U"
// where U is expected to be `Copy`able
macro_rules! forward_ref_op_assign {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl $imp<&$u> for $t {
            #[inline]
            fn $method(&mut self, other: &$u) {
                $imp::$method(self, *other);
            }
        }
    };
}

macro_rules! gf_impl_conv {
    ($type:ty) => {
        impl From<$type> for GF<$type> {
            fn from(u: $type) -> Self {
                Self(u)
            }
        }

        impl From<GF<$type>> for $type {
            fn from(GF(u): GF<$type>) -> Self {
                u
            }
        }

        impl AsRef<GF<$type>> for $type {
            fn as_ref(&self) -> &GF<$type> {
                GF::from_ref(self)
            }
        }

        impl AsMut<GF<$type>> for $type {
            fn as_mut(&mut self) -> &mut GF<$type> {
                GF::from_mut(self)
            }
        }
    };
}

macro_rules! gf_impl_add {
    ($t:ty) => {
        impl GF<$t> {
            const ZERO: Self = Self(0);
        }

        impl Zero for GF<$t> {
            fn zero() -> Self {
                Self::ZERO
            }
            fn is_zero(&self) -> bool {
                *self == Self::ZERO
            }
        }

        impl Add for GF<$t> {
            type Output = GF<$t>;

            #[inline]
            fn add(self, other: GF<$t>) -> GF<$t> {
                self ^ other
            }
        }
        forward_ref_binop! { impl Add, add for GF<$t>, GF<$t>}

        impl AddAssign for GF<$t> {
            #[inline]
            fn add_assign(&mut self, other: GF<$t>) {
                *self = *self + other;
            }
        }
        forward_ref_op_assign! { impl AddAssign, add_assign for GF<$t>, GF<$t> }

        impl Sub for GF<$t> {
            type Output = GF<$t>;

            #[inline]
            fn sub(self, other: GF<$t>) -> GF<$t> {
                self ^ other
            }
        }
        forward_ref_binop! { impl Sub, sub for GF<$t>, GF<$t>}

        impl SubAssign for GF<$t> {
            #[inline]
            fn sub_assign(&mut self, other: GF<$t>) {
                *self = *self - other;
            }
        }
        forward_ref_op_assign! { impl SubAssign, sub_assign for GF<$t>, GF<$t> }

        impl Not for GF<$t> {
            type Output = GF<$t>;

            #[inline]
            fn not(self) -> GF<$t> {
                self
            }
        }
        forward_ref_unop! { impl Not, not for GF<$t>}

        impl BitXor for GF<$t> {
            type Output = GF<$t>;

            #[inline]
            fn bitxor(self, other: GF<$t>) -> GF<$t> {
                GF(self.0 ^ other.0)
            }
        }
        forward_ref_binop! { impl BitXor, bitxor for GF<$t>, GF<$t>}

        impl BitXorAssign for GF<$t> {
            #[inline]
            fn bitxor_assign(&mut self, other: GF<$t>) {
                *self = *self ^ other;
            }
        }
        forward_ref_op_assign! { impl BitXorAssign, bitxor_assign for GF<$t>, GF<$t> }

        impl BitOr for GF<$t> {
            type Output = GF<$t>;

            #[inline]
            fn bitor(self, other: GF<$t>) -> GF<$t> {
                GF(self.0 | other.0)
            }
        }
        forward_ref_binop! { impl BitOr, bitor for GF<$t>, GF<$t>}

        impl BitOrAssign for GF<$t> {
            #[inline]
            fn bitor_assign(&mut self, other: GF<$t>) {
                *self = *self | other;
            }
        }
        forward_ref_op_assign! { impl BitOrAssign, bitor_assign for GF<$t>, GF<$t> }

        impl BitAnd for GF<$t> {
            type Output = GF<$t>;

            #[inline]
            fn bitand(self, other: GF<$t>) -> GF<$t> {
                GF(self.0 & other.0)
            }
        }
        forward_ref_binop! { impl BitAnd, bitand for GF<$t>, GF<$t>}

        impl BitAndAssign for GF<$t> {
            #[inline]
            fn bitand_assign(&mut self, other: GF<$t>) {
                *self = *self & other;
            }
        }
        forward_ref_op_assign! { impl BitAndAssign, bitand_assign for GF<$t>, GF<$t> }

        impl Neg for GF<$t> {
            type Output = Self;
            #[inline]
            fn neg(self) -> Self {
                self
            }
        }
        forward_ref_unop! { impl Neg, neg for GF<$t>}

        impl Sum for GF<$t> {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::ZERO, Add::add)
            }
        }

        impl<'a> Sum<&'a GF<$t>> for GF<$t> {
            fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.fold(Self::ZERO, Add::add)
            }
        }
    };
}

macro_rules! gf_impl_mul {
    ($t:ty) => {
        impl GF<$t> {
            const ONE: Self = Self(1);

            pub fn inv(self) -> Self {
                Self(ALOGTABLE[255 - (LOGTABLE[self.idx()] % 255)])
            }

            pub fn pow(self, pow: usize) -> Self {
                if self == Self::ZERO && pow != 0 {
                    Self::ZERO
                } else {
                    Self(ALOGTABLE[pow * LOGTABLE[self.idx()] % 255])
                }
            }

            fn idx(self) -> usize {
                self.0 as usize
            }
        }

        impl One for GF<$t> {
            fn one() -> Self {
                Self::ONE
            }
        }

        impl Pow<$t> for GF<$t> {
            type Output = Self;

            fn pow(self, other: $t) -> Self {
                self.pow(other as usize)
            }
        }

        forward_ref_binop! { impl Pow, pow for GF<$t>, $t }

        impl Pow<usize> for GF<$t> {
            type Output = Self;

            fn pow(self, other: usize) -> Self {
                self.pow(other)
            }
        }
        forward_ref_binop! { impl Pow, pow for GF<$t>, usize }

        impl Inv for GF<$t> {
            type Output = Self;

            fn inv(self) -> Self::Output {
                self.inv()
            }
        }

        forward_ref_unop! { impl Inv, inv for GF<$t> }

        impl Mul for GF<$t> {
            type Output = GF<$t>;

            #[inline]
            fn mul(self, other: GF<$t>) -> GF<$t> {
                GF(ALOGTABLE[(LOGTABLE[self.idx()]) + (LOGTABLE[other.idx()])])
            }
        }
        forward_ref_binop! { impl Mul, mul for GF<$t>, GF<$t> }

        impl MulAssign for GF<$t> {
            #[inline]
            fn mul_assign(&mut self, other: GF<$t>) {
                *self = *self * other;
            }
        }
        forward_ref_op_assign! { impl MulAssign, mul_assign for GF<$t>, GF<$t> }

        impl Div for GF<$t> {
            type Output = GF<$t>;

            #[inline]
            fn div(self, other: GF<$t>) -> GF<$t> {
                assert!(other != Self::ZERO, "attempt to divide by zero");
                GF(ALOGTABLE[LOGTABLE[self.idx()] + 255 - LOGTABLE[other.idx()]])
            }
        }
        forward_ref_binop! { impl Div, div for GF<$t>, GF<$t>}

        impl DivAssign for GF<$t> {
            #[inline]
            fn div_assign(&mut self, other: GF<$t>) {
                *self = *self / other;
            }
        }
        forward_ref_op_assign! { impl DivAssign, div_assign for GF<$t>, GF<$t> }

        // impl Rem for GF<$t> {
        //     type Output = GF<$t>;

        //     #[inline]
        //     fn rem(self, other: GF<$t>) -> GF<$t> {
        //         GF(self.0.GF_rem(other.0))
        //     }
        // }
        // forward_ref_binop! { impl Rem, rem for GF<$t>, GF<$t>}

        // impl RemAssign for GF<$t> {
        //     #[inline]
        //     fn rem_assign(&mut self, other: GF<$t>) {
        //         *self = *self % other;
        //     }
        // }
        // forward_ref_op_assign! { impl RemAssign, rem_assign for GF<$t>, GF<$t> }

        impl Product for GF<$t> {
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::ONE, Mul::mul)
            }
        }

        impl<'a> Product<&'a GF<$t>> for GF<$t> {
            fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
                iter.fold(Self::ONE, Mul::mul)
            }
        }
    };
}
macro_rules! gf_impls {
    ($($t:ty), *) => {$(
        gf_impl_conv!{ $t }
        gf_impl_add!{ $t }
    )*}
}

gf_impls! { u8, u32, u64, u128, usize}

const LOGTABLE: [usize; 256] = crate::TABLES_U8.0;

const ALOGTABLE: [u8; 1025] = crate::TABLES_U8.1;

gf_impl_mul! {u8}
