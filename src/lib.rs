use std::ops::{Add, AddAssign, Sub, SubAssign};



#[derive(Debug, Clone, Copy)]
pub struct GF256(pub u8);



impl std::fmt::Display for GF256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
        *self = GF256(self.0 ^ other.0);
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
        *self = GF256(self.0 ^ other.0);
    }
}


#[cfg(test)]
mod tests {
}
