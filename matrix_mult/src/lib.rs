use std::{
    fmt::Debug,
    ops::{Add, Mul},
};

use lalgebra_scalar::Scalar;

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar + Copy> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        let mut vec: Vec<T> = vec![];
        for t in self.0[n].clone() {
            vec.push(t);
        }
        vec
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut vec: Vec<T> = vec![];
        for v in &self.0 {
            vec.push(v[n]);
        }
        vec
    }
}

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

impl<T: Scalar<Item = T> + Copy + Mul<Output = T> + Add<Output = T> + Debug> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut matrix = Vec::new();
        if self.0.len() == self.0.len() {
            for index in 0..self.0.len() {
                if self.0[index].len() != rhs.0[index].len() {
                    return None;
                }
                let mut vec3 = Vec::new();
                for k in 0..self.0.len() {
					let mut a = T::zero();
                    for i in 0..self.0[index].len() {
						a = a  + (self.0[index][i] * rhs.0[i][k])
                    }
					vec3.push(a);
                }
                matrix.push(vec3);
            }
            Some(Matrix(matrix))
        } else {
            None
        }
    }
}
