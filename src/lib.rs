pub mod template;

// Use this file to add helper functions and additional modules.
#[derive(Debug, Clone)]
pub struct Matrix {
    matrix: Box<[u8]>,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix {
    pub fn from(input: &str) -> Self {
        let cols = input.find('\n').unwrap();
        let rows = (input.trim().len() + 1) / (cols + 1);
        let matrix = input
            .trim()
            .lines()
            .flat_map(|line| line.bytes())
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Matrix { matrix, rows, cols }
    }

    fn serialize(&self, i: i32, j: i32) -> usize {
        self.cols * i as usize + j as usize
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
        Some(((index / self.cols) as i32, (index % self.cols) as i32))
    }
}

impl std::ops::Index<(i32, i32)> for Matrix {
    type Output = u8;

    fn index(&self, (i, j): (i32, i32)) -> &Self::Output {
        &self.matrix[self.serialize(i, j)]
    }
}

impl std::ops::IndexMut<(i32, i32)> for Matrix {
    fn index_mut(&mut self, (i, j): (i32, i32)) -> &mut Self::Output {
        &mut self.matrix[self.serialize(i, j)]
    }
}
