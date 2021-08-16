#![no_std]
//! # Galois Field
//!
//! finite field arithmetic
//!
//! ```
//! use gf::GF;
//!
//! let x = GF(123u8);
//! let y = GF(225u8);
//! println!("{}", x + y);
//! ```

use core::fmt;

mod gen_table;
mod impls;

const TABLES_U8: ([usize; 256], [u8; 1025]) = gen_table::gen_tables_u8();

/// # The Golias Field Type.
///
/// ```
/// use gf::GF;
///
/// let val = GF(4);
///
/// let typed_val1 = GF(5u8);
/// let typed_val2 = GF::<u8>(5);
/// assert_eq!(typed_val1, typed_val2)
/// ```
///
/// Supports all basic Mathemtaical Functions
#[derive(PartialEq, Eq, Clone, Copy, Default, Hash, Debug)]
#[repr(transparent)]
pub struct GF<T>(pub T);

pub type GF256 = GF<u8>;

impl<T> GF<T> {
    pub fn from_ref<'a>(u: &'a T) -> &'a Self {
        unsafe { &*(u as *const T as *const Self) }
    }

    pub fn from_mut<'a>(u: &'a mut T) -> &'a mut Self {
        unsafe { &mut *(u as *mut T as *mut Self) }
    }
}

impl<T: fmt::Display> fmt::Display for GF<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::Binary> fmt::Binary for GF<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::Octal> fmt::Octal for GF<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::LowerHex> fmt::LowerHex for GF<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: fmt::UpperHex> fmt::UpperHex for GF<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::GF;
    #[test]
    fn add_sub() {
        assert_eq!((GF(5u8) + GF(60)) - GF(5), GF(60))
    }

    #[test]
    fn mul_div() {
        assert_eq!((GF(5u8) * GF(60)) / GF(5), GF(60))
    }

    #[test]
    fn pow() {
        assert_eq!(GF(5u8).pow(0), GF(1))
    }
    #[test]
    fn pow1() {
        assert_eq!(GF(5u8).pow(1), GF(5))
    }
    #[test]
    fn pow2() {
        assert_eq!(GF(4u8).pow(2), GF(4) * GF(4))
    }

    #[test]
    fn conv() {
        assert_eq!(GF::from(34u8), GF(34u8));
        let x: u8 = GF(34u8).into();
        assert!(x == 34);
    }

    #[test]
    fn test_ref() {
        let mut a = 5u8;
        *a.as_mut() += GF(10);
        assert_eq!(a, (GF(5u8) + GF(10)).0)
    }
}
