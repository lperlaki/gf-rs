use core::{
    iter::{Product, Sum},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Sub, SubAssign,
    },
};

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
            fn from(u: GF<$type>) -> Self {
                u.0
            }
        }
    };
}

macro_rules! gf_impl_add {
    ($t:ty) => {
        impl GF<$t> {
            const ZERO: Self = Self(0);
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

            pub fn inverse(self) -> Self {
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

        impl Mul for GF<$t> {
            type Output = GF<$t>;

            #[inline]
            fn mul(self, other: GF<$t>) -> GF<$t> {
                GF(ALOGTABLE[(LOGTABLE[self.idx()] | 0) + (LOGTABLE[other.idx()] | 0)] | 0)
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
