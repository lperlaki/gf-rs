use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
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

pub type GF256 = GF<u8>;

impl Field for GF256 {
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

impl Display for GF256 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add for GF256 {
    type Output = GF256;

    fn add(self, other: GF256) -> GF256 {
        let mut result = self;
        result.add_assign(other);
        result
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl AddAssign for GF256 {
    fn add_assign(&mut self, other: GF256) {
        *self = GF(self.0 ^ other.0);
    }
}

impl Sub for GF256 {
    type Output = GF256;

    fn sub(self, other: GF256) -> GF256 {
        let mut result = self;
        result.sub_assign(other);
        result
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl SubAssign for GF256 {
    fn sub_assign(&mut self, other: GF256) {
        *self = GF(self.0 ^ other.0);
    }
}

impl Mul for GF256 {
    type Output = GF256;

    fn mul(self, rhs: GF256) -> GF256 {
        let mut result = self;
        result.mul_assign(rhs);
        result
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl MulAssign for GF256 {
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

impl Div for GF256 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let mut result = self;
        result.div_assign(rhs);
        result
    }
}

impl DivAssign for GF256 {
    fn div_assign(&mut self, rhs: GF256) {
        *self *= rhs.inverse().unwrap();
    }
}

impl Neg for GF256 {
    type Output = GF256;

    fn neg(self) -> GF256 {
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
