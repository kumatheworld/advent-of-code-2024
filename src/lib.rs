pub mod template;

// Use this file to add helper functions and additional modules.

use itertools::iproduct;

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    array: Box<[T]>,
    pub rows: usize,
    pub cols: usize,
}

impl Matrix<u8> {
    pub fn from(input: &str) -> Self {
        let cols = input.find('\n').unwrap();
        let rows = (input.trim().len() + 1) / (cols + 1);
        let array = input
            .trim()
            .lines()
            .flat_map(|line| line.bytes())
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Matrix { array, rows, cols }
    }
}

impl<T> Matrix<T> {
    fn serialize(&self, i: i32, j: i32) -> usize {
        self.cols * i as usize + j as usize
    }

    pub fn indices(&self) -> impl Iterator<Item = (i32, i32)> {
        iproduct!(0..self.rows as i32, 0..self.cols as i32)
    }

    pub fn new_uniform<U: Clone>(&self, u: U) -> Matrix<U> {
        let rows = self.rows;
        let cols = self.cols;
        let array = vec![u; rows * cols].into_boxed_slice();
        Matrix { array, rows, cols }
    }
}

impl<T: Copy> Matrix<T> {
    pub fn get(&self, i: i32, j: i32) -> Option<T> {
        ((0..self.rows as i32).contains(&i) && (0..self.cols as i32).contains(&j))
            .then(|| self[(i, j)])
    }
}

impl<T: PartialEq> Matrix<T> {
    pub fn position(&self, b: T) -> Option<(i32, i32)> {
        let index = self.array.iter().position(|b_| *b_ == b)?;
        Some(((index / self.cols) as i32, (index % self.cols) as i32))
    }
}

impl<T> std::ops::Index<(i32, i32)> for Matrix<T> {
    type Output = T;

    fn index(&self, (i, j): (i32, i32)) -> &Self::Output {
        &self.array[self.serialize(i, j)]
    }
}

impl<T> std::ops::IndexMut<(i32, i32)> for Matrix<T> {
    fn index_mut(&mut self, (i, j): (i32, i32)) -> &mut Self::Output {
        &mut self.array[self.serialize(i, j)]
    }
}
