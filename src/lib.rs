pub mod template;

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

    pub fn get(&self, i: i32, j: i32) -> Option<u8> {
        if (0..self.rows as i32).contains(&i) && (0..self.cols as i32).contains(&j) {
            Some(self[(i, j)])
        } else {
            None
        }
    }

    pub fn find(&self, b: u8) -> Option<(i32, i32)> {
        let index = self.matrix.iter().position(|&b_| b_ == b)?;
        Some((
            (index / (self.cols + 1)) as i32,
            (index % (self.cols + 1)) as i32,
        ))
    }
}

impl std::ops::Index<(i32, i32)> for Matrix<'_> {
    type Output = u8;

    fn index(&self, (i, j): (i32, i32)) -> &Self::Output {
        &self.matrix[(self.cols + 1) * i as usize + j as usize]
    }
}
