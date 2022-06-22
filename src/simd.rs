use core::{
    ops::{Add, Div, Mul, Sub},
    simd::{LaneCount, Simd, SupportedLaneCount},
};

use crate::GF;

pub type GF256Simd<const LANES: usize> = GF<Simd<u8, LANES>>;

impl<const LANES: usize> From<[u8; LANES]> for GF256Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    fn from(v: [u8; LANES]) -> Self {
        GF(Simd::from(v))
    }
}

impl<const LANES: usize> From<GF256Simd<LANES>> for [u8; LANES]
where
    LaneCount<LANES>: SupportedLaneCount,
{
    fn from(v: GF256Simd<LANES>) -> Self {
        v.0.into()
    }
}

impl<const LANES: usize> GF256Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    #[inline]
    pub fn splat(v: u8) -> Self {
        GF(Simd::splat(v))
    }

    #[inline]
    pub fn inv(self) -> Self {
        let i = Simd::splat(255)
            - (Simd::gather_or_default(LOGTABLE, self.0.cast()) % Simd::splat(255));

        Self(Simd::gather_or_default(ALOGTABLE, i))
    }

    #[inline]
    pub fn pow(self, exp: Simd<usize, LANES>) -> Self {
        let enable = self.0.cast::<usize>().lanes_ne(Simd::splat(0)) | exp.lanes_eq(Simd::splat(0));

        let i = Simd::gather_or_default(LOGTABLE, self.0.cast()) * exp;

        Self(Simd::gather_select(
            ALOGTABLE,
            enable,
            i % Simd::splat(255),
            Simd::splat(0),
        ))
    }

    #[inline]
    pub fn sum_lanes(self) -> u8 {
        self.0.reduce_xor()
    }
}

impl<const LANES: usize> Add for GF256Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl<const LANES: usize> Sub for GF256Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}

impl<const LANES: usize> Mul for GF256Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        let i = Simd::gather_or_default(LOGTABLE, self.0.cast())
            + Simd::gather_or_default(LOGTABLE, rhs.0.cast());

        Self(Simd::gather_or_default(ALOGTABLE, i))
    }
}

impl<const LANES: usize> Div for GF256Simd<LANES>
where
    LaneCount<LANES>: SupportedLaneCount,
{
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self {
        assert!(
            other.0.lanes_eq(Simd::splat(0)).any(),
            "attempt to divide by zero"
        );

        let i = Simd::gather_or_default(LOGTABLE, self.0.cast()) + Simd::splat(255)
            - Simd::gather_or_default(LOGTABLE, other.0.cast());

        Self(Simd::gather_or_default(ALOGTABLE, i))
    }
}

const LOGTABLE: &'static [usize; 256] = &crate::TABLES_U8.0;
const ALOGTABLE: &'static [u8; 1025] = &crate::TABLES_U8.1;

#[cfg(test)]
mod tests {
    use crate::GF;
    use core::simd::Simd;
    #[test]
    fn add_sub() {
        assert_eq!(
            (GF(Simd::<_, 64>::splat(5u8)) + GF(Simd::splat(60))) - GF(Simd::splat(5)),
            GF(Simd::splat(60))
        )
    }
}
