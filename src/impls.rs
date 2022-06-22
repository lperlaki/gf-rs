use core::{
    iter::{Product, Sum},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[cfg(feature = "num-traits")]
use num_traits::{Inv, One, Pow, Zero};

use crate::GF;

macro_rules! deref_lhs {
    (impl<T> $trait:ident for $gf:ty {
            fn $call:ident
        }) => {
        impl<T> $trait<$gf> for &$gf
        where
            $gf: Copy,
            $gf: $trait<$gf, Output = $gf>,
        {
            type Output = $gf;

            #[inline]
            fn $call(self, rhs: $gf) -> Self::Output {
                (*self).$call(rhs)
            }
        }
    };
}

macro_rules! deref_rhs {
    (impl<T> $trait:ident for $gf:ty {
            fn $call:ident
        }) => {
        impl<T> $trait<&$gf> for $gf
        where
            $gf: Copy,
            $gf: $trait<$gf, Output = $gf>,
        {
            type Output = $gf;

            #[inline]
            fn $call(self, rhs: &$gf) -> Self::Output {
                self.$call(*rhs)
            }
        }
    };
}

macro_rules! deref_ops {
    ($(impl<T> $trait:ident for $gf:ty {
            fn $call:ident
        })*) => {
        $(
            deref_rhs! {
                impl<T> $trait for $gf {
                    fn $call
                }
            }
            deref_lhs! {
                impl<T> $trait for $gf {
                    fn $call
                }
            }
            impl<T> $trait<&'_ $gf> for &$gf
            where
                $gf: Copy,
                $gf: $trait<$gf, Output = $gf>,
            {
                type Output = $gf;

                #[inline]
                fn $call(self, rhs: &$gf) -> Self::Output {
                    (*self).$call(*rhs)
                }
            }
        )*
    }
}

macro_rules! assign_ops {
    ($(impl<T, U> $assignTrait:ident<U> for GF<T>
        where
            Self: $trait:ident,
        {
            fn $assign_call:ident(rhs: U) {
                $call:ident
            }
        })*) => {
        $(impl<T, U> $assignTrait<U> for GF<T>
        where
            Self: Copy,
            Self: $trait<U, Output = Self>,
        {
            #[inline]
            fn $assign_call(&mut self, rhs: U) {
                *self = self.$call(rhs);
            }
        })*
    }
}

deref_ops! {
    // Arithmetic

    impl<T> Add for GF<T> {
        fn add
    }

    impl<T> Sub for GF<T> {
        fn sub
    }

    impl<T> Mul for GF<T> {
        fn mul
    }

    impl<T> Div for GF<T> {
        fn div
    }
}

assign_ops! {
    // Arithmetic

    impl<T, U> AddAssign<U> for GF<T>
    where
        Self: Add,
    {
        fn add_assign(rhs: U) {
            add
        }
    }

    impl<T, U> SubAssign<U> for GF<T>
    where
        Self: Sub,
    {
        fn sub_assign(rhs: U) {
            sub
        }
    }

    impl<T, U> MulAssign<U> for GF<T>
    where
        Self: Mul,
    {
        fn mul_assign(rhs: U) {
            mul
        }
    }

    impl<T, U> DivAssign<U> for GF<T>
    where
        Self: Div,
    {
        fn div_assign(rhs: U) {
            div
        }
    }
}

macro_rules! gf_impl_conv {
    ($type:ty) => {
        impl From<$type> for GF<$type> {
            #[inline]
            fn from(u: $type) -> Self {
                Self(u)
            }
        }

        impl From<GF<$type>> for $type {
            #[inline]
            fn from(GF(u): GF<$type>) -> Self {
                u
            }
        }
    };
}

macro_rules! gf_impl_add {
    ($t:ty) => {
        impl GF<$t> {
            pub const ZERO: Self = Self(0);
        }

        #[cfg(feature = "num-traits")]
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
                Self(self.0 ^ other.0)
            }
        }

        impl Sub for GF<$t> {
            type Output = GF<$t>;

            #[inline]
            fn sub(self, other: GF<$t>) -> GF<$t> {
                Self(self.0 ^ other.0)
            }
        }

        impl Neg for GF<$t> {
            type Output = Self;
            #[inline]
            fn neg(self) -> Self {
                self
            }
        }

        impl<U> Sum<U> for GF<$t>
        where
            Self: Add<U, Output = Self>,
        {
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = U>,
            {
                iter.fold(Self::ZERO, Add::add)
            }
        }
    };
}

macro_rules! gf_impl_mul {
    ($t:ty) => {
        impl GF<$t> {
            pub const ONE: Self = Self(1);

            #[inline]
            pub fn inv(self) -> Self {
                Self(ALOGTABLE[255 - (LOGTABLE[self.idx()] % 255)])
            }

            #[inline]
            pub fn pow(self, exp: usize) -> Self {
                if self == Self::ZERO && exp != 0 {
                    Self::ZERO
                } else {
                    Self(ALOGTABLE[exp * LOGTABLE[self.idx()] % 255])
                }
            }

            #[inline]
            fn idx(self) -> usize {
                self.0 as usize
            }
        }

        impl Mul for GF<$t> {
            type Output = GF<$t>;

            #[inline]
            fn mul(self, other: GF<$t>) -> GF<$t> {
                GF(ALOGTABLE[(LOGTABLE[self.idx()]) + (LOGTABLE[other.idx()])])
            }
        }

        impl Div for GF<$t> {
            type Output = GF<$t>;

            #[inline]
            fn div(self, other: GF<$t>) -> GF<$t> {
                assert!(other != Self::ZERO, "attempt to divide by zero");
                GF(ALOGTABLE[LOGTABLE[self.idx()] + 255 - LOGTABLE[other.idx()]])
            }
        }

        impl<U> Product<U> for GF<$t>
        where
            Self: Mul<U, Output = Self>,
        {
            fn product<I>(iter: I) -> Self
            where
                I: Iterator<Item = U>,
            {
                iter.fold(Self::ONE, Mul::mul)
            }
        }

        #[cfg(feature = "num-traits")]
        mod num_traits_impl {
            use super::*;

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

            impl Pow<usize> for GF<$t> {
                type Output = Self;

                fn pow(self, other: usize) -> Self {
                    self.pow(other)
                }
            }

            impl Inv for GF<$t> {
                type Output = Self;

                fn inv(self) -> Self::Output {
                    self.inv()
                }
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

// const LOGTABLE: &'static [usize; 256] = &crate::TABLES_U8.0;
// const ALOGTABLE: &'static [u8; 1025] = &crate::TABLES_U8.1;

#[cfg(test)]
mod tests {
    #[test]
    fn check_logtables() {
        const TABLES_U8: ([usize; 256], [u8; 1025]) = crate::gen_table::gen_tables_u8(0x11D);

        assert!(TABLES_U8.0 == *super::LOGTABLE);
        assert!(TABLES_U8.1 == *super::ALOGTABLE);
    }
}

const LOGTABLE: &'static [usize; 256] = &[
    512, 255, 1, 25, 2, 50, 26, 198, 3, 223, 51, 238, 27, 104, 199, 75, 4, 100, 224, 14, 52, 141,
    239, 129, 28, 193, 105, 248, 200, 8, 76, 113, 5, 138, 101, 47, 225, 36, 15, 33, 53, 147, 142,
    218, 240, 18, 130, 69, 29, 181, 194, 125, 106, 39, 249, 185, 201, 154, 9, 120, 77, 228, 114,
    166, 6, 191, 139, 98, 102, 221, 48, 253, 226, 152, 37, 179, 16, 145, 34, 136, 54, 208, 148,
    206, 143, 150, 219, 189, 241, 210, 19, 92, 131, 56, 70, 64, 30, 66, 182, 163, 195, 72, 126,
    110, 107, 58, 40, 84, 250, 133, 186, 61, 202, 94, 155, 159, 10, 21, 121, 43, 78, 212, 229, 172,
    115, 243, 167, 87, 7, 112, 192, 247, 140, 128, 99, 13, 103, 74, 222, 237, 49, 197, 254, 24,
    227, 165, 153, 119, 38, 184, 180, 124, 17, 68, 146, 217, 35, 32, 137, 46, 55, 63, 209, 91, 149,
    188, 207, 205, 144, 135, 151, 178, 220, 252, 190, 97, 242, 86, 211, 171, 20, 42, 93, 158, 132,
    60, 57, 83, 71, 109, 65, 162, 31, 45, 67, 216, 183, 123, 164, 118, 196, 23, 73, 236, 127, 12,
    111, 246, 108, 161, 59, 82, 41, 157, 85, 170, 251, 96, 134, 177, 187, 204, 62, 90, 203, 89, 95,
    176, 156, 169, 160, 81, 11, 245, 22, 235, 122, 117, 44, 215, 79, 174, 213, 233, 230, 231, 173,
    232, 116, 214, 244, 234, 168, 80, 88, 175,
];

const ALOGTABLE: &'static [u8; 1025] = &[
    1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135, 19, 38, 76, 152, 45, 90, 180, 117,
    234, 201, 143, 3, 6, 12, 24, 48, 96, 192, 157, 39, 78, 156, 37, 74, 148, 53, 106, 212, 181,
    119, 238, 193, 159, 35, 70, 140, 5, 10, 20, 40, 80, 160, 93, 186, 105, 210, 185, 111, 222, 161,
    95, 190, 97, 194, 153, 47, 94, 188, 101, 202, 137, 15, 30, 60, 120, 240, 253, 231, 211, 187,
    107, 214, 177, 127, 254, 225, 223, 163, 91, 182, 113, 226, 217, 175, 67, 134, 17, 34, 68, 136,
    13, 26, 52, 104, 208, 189, 103, 206, 129, 31, 62, 124, 248, 237, 199, 147, 59, 118, 236, 197,
    151, 51, 102, 204, 133, 23, 46, 92, 184, 109, 218, 169, 79, 158, 33, 66, 132, 21, 42, 84, 168,
    77, 154, 41, 82, 164, 85, 170, 73, 146, 57, 114, 228, 213, 183, 115, 230, 209, 191, 99, 198,
    145, 63, 126, 252, 229, 215, 179, 123, 246, 241, 255, 227, 219, 171, 75, 150, 49, 98, 196, 149,
    55, 110, 220, 165, 87, 174, 65, 130, 25, 50, 100, 200, 141, 7, 14, 28, 56, 112, 224, 221, 167,
    83, 166, 81, 162, 89, 178, 121, 242, 249, 239, 195, 155, 43, 86, 172, 69, 138, 9, 18, 36, 72,
    144, 61, 122, 244, 245, 247, 243, 251, 235, 203, 139, 11, 22, 44, 88, 176, 125, 250, 233, 207,
    131, 27, 54, 108, 216, 173, 71, 142, 1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135,
    19, 38, 76, 152, 45, 90, 180, 117, 234, 201, 143, 3, 6, 12, 24, 48, 96, 192, 157, 39, 78, 156,
    37, 74, 148, 53, 106, 212, 181, 119, 238, 193, 159, 35, 70, 140, 5, 10, 20, 40, 80, 160, 93,
    186, 105, 210, 185, 111, 222, 161, 95, 190, 97, 194, 153, 47, 94, 188, 101, 202, 137, 15, 30,
    60, 120, 240, 253, 231, 211, 187, 107, 214, 177, 127, 254, 225, 223, 163, 91, 182, 113, 226,
    217, 175, 67, 134, 17, 34, 68, 136, 13, 26, 52, 104, 208, 189, 103, 206, 129, 31, 62, 124, 248,
    237, 199, 147, 59, 118, 236, 197, 151, 51, 102, 204, 133, 23, 46, 92, 184, 109, 218, 169, 79,
    158, 33, 66, 132, 21, 42, 84, 168, 77, 154, 41, 82, 164, 85, 170, 73, 146, 57, 114, 228, 213,
    183, 115, 230, 209, 191, 99, 198, 145, 63, 126, 252, 229, 215, 179, 123, 246, 241, 255, 227,
    219, 171, 75, 150, 49, 98, 196, 149, 55, 110, 220, 165, 87, 174, 65, 130, 25, 50, 100, 200,
    141, 7, 14, 28, 56, 112, 224, 221, 167, 83, 166, 81, 162, 89, 178, 121, 242, 249, 239, 195,
    155, 43, 86, 172, 69, 138, 9, 18, 36, 72, 144, 61, 122, 244, 245, 247, 243, 251, 235, 203, 139,
    11, 22, 44, 88, 176, 125, 250, 233, 207, 131, 27, 54, 108, 216, 173, 71, 142, 1, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

gf_impl_mul! {u8}
