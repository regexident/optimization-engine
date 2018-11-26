//! # matrix_operations
//!
//! Matrix operations used by the optimization algorithm.
//!
//! # Examples
//!
//! ```
//! fn main() {
//! }
//! ```
//!
//! # Errors
//!
//!
//! # Panics
//!
//!

use num::{Float, Zero};
use std::iter::Sum;
use std::ops::{Add, Mul};

/// Calculate the inner product of two vectors
#[inline(always)]
pub fn inner_product<T>(a: &[T], b: &[T]) -> T
where
    T: Float + Sum<T> + Mul<T, Output = T>,
{
    assert!(a.len() == b.len());

    a.iter().zip(b.iter()).map(|(x, y)| (*x) * (*y)).sum()
}

/// Calculate the 2-norm of a vector
#[inline(always)]
pub fn norm1<T>(a: &[T]) -> T
where
    T: Float + Sum<T> + Add<T, Output = T>,
{
    a.iter().map(|x| x.abs()).sum()
}

/// Calculate the 2-norm of a vector
#[inline(always)]
pub fn norm2<T>(a: &[T]) -> T
where
    T: Float + Sum<T> + Mul<T, Output = T>,
{
    let norm: T = a.iter().map(|x| (*x) * (*x)).sum();
    norm.sqrt()
}

/// Calculate the infinity-norm of a vector
#[inline(always)]
pub fn norm_inf<T>(a: &[T]) -> T
where
    T: Float + Zero,
{
    a.iter()
        .fold(T::zero(), |current_max, x| x.abs().max(current_max))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn inner_product_test() {
        assert_eq!(
            matrix_operations::inner_product(&vec![1.0, 2.0, 3.0], &vec![1.0, 2.0, 3.0]),
            14.0
        );
    }

    #[test]
    #[should_panic]
    fn inner_product_test_panic() {
        matrix_operations::inner_product(&vec![2.0, 3.0], &vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn norm1_test() {
        assert_eq!(matrix_operations::norm1(&vec![1.0, -2.0, -3.0]), 6.0);
    }

    #[test]
    fn norm2_test() {
        assert_eq!(matrix_operations::norm2(&vec![3.0, 4.0]), 5.0);
    }

    #[test]
    fn norm_inf_test() {
        assert_eq!(matrix_operations::norm_inf(&vec![1.0, -2.0, -3.0]), 3.0);
        assert_eq!(
            matrix_operations::norm_inf(&vec![1.0, -8.0, -3.0, 0.0]),
            8.0
        );
    }
}