pub mod template;

use std::ops::Index;

// Use this file to add helper functions and additional modules.
#[derive(Debug)]
pub struct Matrix<'a> {
    matrix: &'a [u8],
    pub rows: usize,
    pub cols: usize,
}

impl<'a> Matrix<'a> {
    pub fn from(input: &'a str) -> Self {
        let rows = input.lines().collect::<Vec<_>>();
        Matrix {
            matrix: input.as_bytes(),
            rows: rows.len(),
            cols: rows[0].len(),
        }
    }
}

impl Index<(i32, i32)> for Matrix<'_> {
    type Output = u8;

    fn index(&self, (i, j): (i32, i32)) -> &Self::Output {
        &self.matrix[(self.cols + 1) * i as usize + j as usize]
    }
}
