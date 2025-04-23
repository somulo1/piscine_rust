use std::ops::{Add, Sub};

#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

pub trait Scalar {
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

// Implement Scalar for basic types
impl Scalar for i32 {
    type Item = i32;
    fn zero() -> Self::Item { 0 }
    fn one() -> Self::Item { 1 }
}

impl Scalar for u32 {
    type Item = u32;
    fn zero() -> Self::Item { 0 }
    fn one() -> Self::Item { 1 }
}

impl Scalar for f64 {
    type Item = f64;
    fn zero() -> Self::Item { 0.0 }
    fn one() -> Self::Item { 1.0 }
}

// Implement Add for Matrix<T>
impl<T> Add for Matrix<T>
where
    T: Scalar<Item = T> + Add<Output = T> + Clone,
{
    type Output = Option<Matrix<T>>;

    fn add(self, rhs: Matrix<T>) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0.first().map_or(0, |r| r.len()) != rhs.0.first().map_or(0, |r| r.len()) {
            return None;
        }

        let result = self.0
            .into_iter()
            .zip(rhs.0.into_iter())
            .map(|(row1, row2)| {
                row1.into_iter()
                    .zip(row2.into_iter())
                    .map(|(a, b)| a + b)
                    .collect::<Vec<T>>()
            })
            .collect();

        Some(Matrix(result))
    }
}

// Implement Sub for Matrix<T>
impl<T> Sub for Matrix<T>
where
    T: Scalar<Item = T> + Sub<Output = T> + Clone,
{
    type Output = Option<Matrix<T>>;

    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0.first().map_or(0, |r| r.len()) != rhs.0.first().map_or(0, |r| r.len()) {
            return None;
        }

        let result = self.0
            .into_iter()
            .zip(rhs.0.into_iter())
            .map(|(row1, row2)| {
                row1.into_iter()
                    .zip(row2.into_iter())
                    .map(|(a, b)| a - b)
                    .collect::<Vec<T>>()
            })
            .collect();

        Some(Matrix(result))
    }
}
