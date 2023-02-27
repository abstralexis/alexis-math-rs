use std::fmt::Display;

pub use crate::vectors::Vec2;
pub use Vec;

pub trait Matrix2Like {
    fn as_matrix2(&self) -> Matrix2;
}

#[derive(Debug)]
pub struct Matrix2 {
    matrix: Vec<Vec<f32>>,
}

impl Matrix2 {
    pub fn transpose(&self) -> Matrix2 {
        let mut matrix_t: Vec<Vec<f32>> = Vec::new();
        (0..(self.matrix[0].len())).for_each(|i| {
            let mut row_t: Vec<f32> = Vec::new();
            for row in &self.matrix {
                row_t.push(row[i]);
            }
            matrix_t.push(row_t);
        });
        Matrix2 { matrix: matrix_t }
    }
}

impl Display for Matrix2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display: String = String::new();
        for row in &self.matrix {
            display.push_str(
                (row.iter().map(|x| format!("{} ", x)).collect::<String>() + "\n").as_str(),
            )
        }
        display.pop();
        write!(f, "{}", display)
    }
}

impl Matrix2Like for Matrix2 {
    fn as_matrix2(&self) -> Matrix2 {
        Matrix2 {
            matrix: self.matrix.clone(),
        }
    }
}

impl Matrix2 {
    pub fn new_2d_zeroes(width: &usize, height: &usize) -> Self {
        Matrix2 {
            matrix: vec![vec![0.0; *width]; *height],
        }
    }
}

impl Matrix2Like for Vec2 {
    fn as_matrix2(&self) -> Matrix2 {
        Matrix2 {
            matrix: vec![vec![self.x], vec![self.y]],
        }
    }
}

impl Matrix2Like for Vec<f32> {
    fn as_matrix2(&self) -> Matrix2 {
        Matrix2 {
            matrix: vec![vec![self[0]], vec![self[1]]],
        }
    }
}
