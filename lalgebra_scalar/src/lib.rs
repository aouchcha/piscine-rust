use std::ops::{Add, Sub, Mul, Div};

pub trait Scalar: Add + Sub + Mul + Div + std::marker::Sized + Clone {
	type Item;
	fn zero() -> Self::Item;
	fn one() -> Self::Item;
}

impl Scalar for u32 {
    type Item = u32;
    fn zero() -> Self::Item {
        0 as u32
    }
	fn one() -> Self::Item {
        1 as u32
    }
}

impl Scalar for u64 {
    type Item = u64;
    fn zero() -> Self::Item {
        0 as u64
    }
	fn one() -> Self::Item {
        1 as u64
    }
}

impl Scalar for i32 {
    type Item = i32;
    fn zero() -> Self::Item {
        0 as i32
    }
	fn one() -> Self::Item {
        1 as i32
    }
}

impl Scalar for i64 {
    type Item = i64;
    fn zero() -> Self::Item {
        0 as i64
    }
	fn one() -> Self::Item {
        1 as i64
    }
}

impl Scalar for f32 {
    type Item = f32;
    fn zero() -> Self::Item {
        0 as f32
    }
	fn one() -> Self::Item {
        1 as f32
    }
}

impl Scalar for f64 {
    type Item = f64;
    fn zero() -> Self::Item {
        0 as f64
    }
	fn one() -> Self::Item {
        1 as f64
    }
}