use std::ops::{Add, Sub};

use lalgebra_scalar::Scalar;

#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T> + Add<Output = T> + Copy> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn add(self, rhs: Self) -> Self::Output {
        let mut matrix = Vec::new();
        if self.0.len() == self.0.len() {
            for index in 0..self.0.len() {
                if self.0[index].len() != rhs.0[index].len() {
                    return None;
                }
                let mut vec3 = Vec::new();
                for j in 0..self.0[index].len() {
                    vec3.push(self.0[index][j] + rhs.0[index][j])
                }
                matrix.push(vec3);
            }
            Some(Matrix(matrix))
        } else {
            None
        }
    }
}

impl<T: Scalar<Item = T> + Sub<Output = T> + Copy> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut matrix = Vec::new();
        if self.0.len() == self.0.len() {
            for index in 0..self.0.len() {
                if self.0[index].len() != rhs.0[index].len() {
                    return None;
                }
                let mut vec3 = Vec::new();
                for j in 0..self.0[index].len() {
                    vec3.push(self.0[index][j] - rhs.0[index][j])
                }
                matrix.push(vec3);
            }
            Some(Matrix(matrix))
        } else {
            None
        }
    }
}
