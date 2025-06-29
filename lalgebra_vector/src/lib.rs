use lalgebra_scalar::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

use std::ops::{Add, Mul};

impl<T: Scalar + Add<Output = T> + Copy> Add for Vector<T> {
    type Output = Option<Vector<T>>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            None
        } else {
            let mut vec = Vector(Vec::new());
            for (index, _) in self.0.iter().enumerate() {
                vec.0.push(self.0[index] + rhs.0[index]);
            }
            Some(vec)
        }
    }
}

impl<T: Scalar<Item = T> + Mul<Output = T> + Copy + Add<Output = T>> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            None
        } else {
            let mut nbr: T = T::zero();
            for (index, _) in self.0.iter().enumerate() {
                nbr = nbr + (self.0[index] * other.0[index]);
            }
            Some(nbr)
        }
    }
}
