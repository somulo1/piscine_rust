use std::ops::Mul;

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

pub trait Scalar {
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

// Implement Scalar for basic types
impl Scalar for u32 {
    type Item = u32;
    fn zero() -> Self::Item { 0 }
    fn one() -> Self::Item { 1 }
}

impl<T: Clone> Matrix<T> {
    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|row| row[n].clone()).collect()
    }
}

// Implement matrix multiplication
impl<T> Mul for Matrix<T>
where
    T: Scalar<Item = T> + Clone + Mul<Output = T> + std::ops::Add<Output = T>,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        let rows_a = self.number_of_rows();
        let cols_a = self.number_of_cols();
        let rows_b = rhs.number_of_rows();
        let cols_b = rhs.number_of_cols();

        if cols_a != rows_b {
            return None;
        }

        let mut result = vec![vec![T::zero(); cols_b]; rows_a];

        for i in 0..rows_a {
            for j in 0..cols_b {
                let mut sum = T::zero();
                for k in 0..cols_a {
                    sum = sum + self.0[i][k].clone() * rhs.0[k][j].clone();
                }
                result[i][j] = sum;
            }
        }

        Some(Matrix(result))
    }
}
