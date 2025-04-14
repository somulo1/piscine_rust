#[derive(Debug, PartialEq, Eq)]
pub struct Matrix(pub (i32, i32), pub (i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
<<<<<<< HEAD
    // Transposing by swapping rows and columns
    Matrix((m.0 .0, m.1 .0), (m.0 .1, m.1 .1))
=======
    let Matrix((a, b), (c, d)) = m;
    Matrix((a, c), (b, d))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_test() {
        let m = Matrix((1, 2), (3, 4));
        let result = transpose(m);
        assert_eq!(result, Matrix((1, 3), (2, 4)));
    }
>>>>>>> 7114c937a7c0a8a7c95d4f2ac968a6ab5af99801
}
