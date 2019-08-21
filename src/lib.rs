#![feature(associated_type_bounds)]

use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Add, AddAssign, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

pub trait Field:
    'static
    + Sized
    + Eq
    + PartialEq
    + Copy
    + Clone
    + Send
    + Sync
    + Debug
    + Display
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Div<Output = Self>
    + DivAssign
    + Mul<Output = Self>
    + MulAssign
    + Neg<Output = Self>
{
    const ZERO: Self;
    const ONE: Self;

    fn zero() -> Self {
        Self::ZERO
    }

    fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }

    fn one() -> Self {
        Self::ONE
    }

    fn inverse(&self) -> Option<Self>;

    fn pow(self, exp: u8) -> Self {
        let mut res = Self::one();
        for i in 0..8 {
            res.square();
            let mut tmp = res;
            tmp.mul_assign(self);
            if (((exp >> i) & 0x01) as u8) != 0 {
                res = tmp
            }
        }
        if exp.eq(&0) {
            res = Self::one()
        }
        res
    }

    fn square(&mut self) {
        *self *= *self
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GF<T>(pub T);

impl<T> GF<T> {
    pub fn new(inner: T) -> Self {
        Self(inner)
    }
}

pub type GFU8 = GF<u8>;
pub type GF256 = GFU8;
pub type GFU16 = GF<u16>;
pub type GFU32 = GF<u32>;
pub type GFU64 = GF<u64>;
pub type GFU128 = GF<u128>;

impl Field for GF<u8> {
    const ZERO: Self = Self(0);
    const ONE: Self = Self(1);

    fn inverse(&self) -> Option<Self> {
        if self.is_zero() {
            None
        } else {
            let mut res = *self;

            for _ in 0..6 {
                res.square();
                res.mul_assign(*self);
            }

            res.square();
            if self.is_zero() {
                res = Self::zero();
            }

            Some(res)
        }
    }
}

impl MulAssign for GF<u8> {
    fn mul_assign(&mut self, rhs: GF256) {
        let a = self.0;
        let mut b = rhs.0;
        // let mut t: u8;
        self.0 = 0x00;
        for i in 0..8 {
            // if (a & 0x01) != 0x00 {
            //     self.0 ^= b;
            // }
            let lsb_of_a_not_0 = !(((a >> i) & 0x01).eq(&0x00));
            if lsb_of_a_not_0 {
                *self = GF(self.0 ^ b)
            }
            // Reduce b using 0x11b, the irreducible polynomial of GF256
            // t = b & 0x80;
            // b = b << 1;
            // if t != 0 {
            //     b = b ^ 0x1b;
            // }
            let choice = (b & 0x80).eq(&0x00);
            b <<= 1;
            let tmp = b ^ 0x1b;
            if !choice {
                b = tmp
            }
        }
    }
}

impl DivAssign for GF<u8> {
    fn div_assign(&mut self, rhs: GF256) {
        *self *= rhs.inverse().unwrap();
    }
}

impl<T: Display> Display for GF<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> Add for GF<T>
where
    Self: AddAssign,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut result = self;
        result.add_assign(rhs);
        result
    }
}

impl<T: BitXor<Output = T> + Copy> AddAssign for GF<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 ^ rhs.0);
    }
}

impl<T> Sub for GF<T>
where
    Self: SubAssign,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut result = self;
        result.sub_assign(rhs);
        result
    }
}
impl<T: BitXor<Output = T> + Copy> SubAssign for GF<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self(self.0 ^ rhs.0);
    }
}

impl<T> Mul for GF<T>
where
    Self: MulAssign,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = self;
        result.mul_assign(rhs);
        result
    }
}

impl<T> Div for GF<T>
where
    Self: DivAssign,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let mut result = self;
        result.div_assign(rhs);
        result
    }
}

impl<T> Neg for GF<T> {
    type Output = Self;

    fn neg(self) -> Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::GF;
    #[test]
    fn add_sub() {
        assert_eq!((GF(5) + GF(60)) - GF(5), GF(60))
    }

    #[test]
    fn mul_div() {
        assert_eq!((GF(5) * GF(60)) / GF(5), GF(60))
    }
}
