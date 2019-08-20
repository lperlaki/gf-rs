use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

pub trait Field:
    'static
    + Sized
    + Eq
    + Copy
    + Clone
    + Send
    + Sync
    + Debug
    + Display
    + Add<Output = Self>
    + AddAssign
    + Div<Output = Self>
    + DivAssign
    + Mul<Output = Self>
    + MulAssign
    + Neg<Output = Self>
    + Sub<Output = Self>
    + SubAssign
{
    /// Returns the zero element of the field, the additive identity.
    fn zero() -> Self;

    /// Returns the one element of the field, the multiplicative identity.
    fn one() -> Self;

    /// Returns true iff this element is zero.
    fn is_zero(&self) -> bool;

    /// Computes the multiplicative inverse of this element, if nonzero.
    fn inverse(&self) -> Option<Self>;

    /// Squares this element.
    fn square(&mut self);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GF<T>(pub T);

pub type GF256 = GF<u8>;

impl GF256 {
    pub fn pow(self, elem: u8) -> Self {
        let mut res = GF256::one();
        for i in 0..8 {
            res.square();
            let mut tmp = res;
            tmp.mul_assign(self);
            if (((elem >> i) & 0x01) as u8) != 0 {
                res = tmp
            }
        }
        if elem.eq(&0) {
            res = GF256::one()
        }
        res
    }
}

impl Field for GF256 {
    /// Returns the zero element of the field (additive identity)
    fn zero() -> Self {
        GF(0)
    }

    /// Returns the zero element of the field (multiplicative identity)
    fn one() -> Self {
        GF(1)
    }

    /// Returns true if this element is the additive identity
    fn is_zero(&self) -> bool {
        self.0.eq(&0).into()
    }

    /// Squares the element
    fn square(&mut self) {
        self.mul_assign(*self);
    }

    /// Returns multiplicative inverse (self^254)
    fn inverse(&self) -> Option<Self> {
        let mut res = *self;

        for _ in 0..6 {
            res.square();
            res.mul_assign(*self);
        }

        res.square();
        if self.0.eq(&0x00) {
            res = GF256::zero();
        }

        Some(res)
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
