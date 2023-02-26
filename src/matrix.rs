pub use Vec;
pub use crate::vectors::Vec2;

pub trait Matrix2Like {
    fn as_matrix2(&self) -> Matrix2;
}

#[derive(Debug)]
pub struct Matrix2 {
    matrix: Vec<Vec<f32>>
}

impl Matrix2Like for Matrix2 {
    fn as_matrix2(&self) -> Matrix2 {
        Matrix2 {
            matrix: self.matrix.clone()
        }
    }
}

impl Matrix2 {
    pub fn new_2d_zeroes(width: &usize, height: &usize) -> Self {
        Matrix2 {
            matrix: vec![vec![0.0; *width]; *height]
        }
    }
}

impl Matrix2Like for Vec2 {
    fn as_matrix2(&self) -> Matrix2 {
        Matrix2 {
            matrix: vec![
                vec![self.x],
                vec![self.y]
            ]
        }
    }
}
