#[derive(Debug)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

pub trait Scalar {
    type Item;
    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

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

impl<T: Scalar<Item = T> + Clone> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::one()]]) // 1x1 matrix with one element = 1
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut mat = vec![vec![T::zero(); n]; n];
        for i in 0..n {
            mat[i][i] = T::one();
        }
        Matrix(mat)
    }
}
