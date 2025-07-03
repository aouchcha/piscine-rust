#[derive(Debug)]
pub struct Matrix<T>(pub Vec<Vec<T>>);
use lalgebra_scalar::*;

impl <T: Scalar<Item = T>> Matrix<T> {
	pub fn new() -> Matrix<T> {
        Matrix::<T>(vec![vec![T::zero()]])
	}

	pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let mut res : Vec<Vec<T>> = Vec::new();
        for _ in 0..row {
            let mut holder : Vec<T> = Vec::new();
            for _ in 0..col {
                holder.push(T::zero());
            }
            res.push(holder)
        }
        Matrix(res)
	}

	pub fn identity(n: usize) -> Matrix<T> {
        let mut res : Vec<Vec<T>> = Vec::new();
        for i in 0..n {
            let mut holder : Vec<T> = Vec::new();
            for j in 0..n {
                if i == j {
                    holder.push(T::one());
                }else {
                    holder.push(T::zero());
                }
            }
            res.push(holder)
        }
        Matrix(res)
	}
}