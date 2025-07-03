use lalgebra_scalar::Scalar;

#[derive(Debug,PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let mut vec_raw = Vec::new();
        for _ in 0..row {
            let mut vec_col = Vec::new();
            for _ in 0..col {
                vec_col.push(T::zero());
            }
            vec_raw.push(vec_col);
        }
        Matrix(vec_raw)
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut vec_raw = Vec::new();
        for row in 0..n {
            let mut vec_col = Vec::new();
            for col in 0..n {
                if row != col {
                    vec_col.push(T::zero());
                } else {
                    vec_col.push(T::one());
                }
            }
            vec_raw.push(vec_col);
        }
        Matrix(vec_raw)
    }
}
