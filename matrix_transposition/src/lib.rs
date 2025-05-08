pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug, PartialEq, Eq)]
pub struct Matrix(pub (i32, i32), pub (i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
    Matrix((m.0.0, m.1.0), (m.0.1, m.1.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matrix = Matrix((1, 3), (4, 5));
        println!("Original matrix {:?}", matrix);
        println!("Transpose matrix {:?}", transpose(matrix));
    }
}
