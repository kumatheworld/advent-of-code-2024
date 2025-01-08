pub mod template;

// Use this file to add helper functions and additional modules.

use itertools::{iproduct, Product};
use std::fmt;

pub const DIJ: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Clone)]
pub struct Matrix<T> {
    pub array: Box<[T]>,
    pub rows: usize,
    pub cols: usize,
}

trait AsSymbol {
    fn as_symbol(&self) -> char;
}

impl AsSymbol for u8 {
    fn as_symbol(&self) -> char {
        *self as char
    }
}

impl AsSymbol for bool {
    fn as_symbol(&self) -> char {
        if *self {
            '*'
        } else {
            ' '
        }
    }
}

impl<T: AsSymbol> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let index = row * self.cols + col;
                write!(f, "{}", self.array[index].as_symbol())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Clone> Matrix<T> {
    pub fn uniform(rows: usize, cols: usize, value: T) -> Self {
        let array = vec![value; rows * cols].into_boxed_slice();
        Matrix { array, rows, cols }
    }
}

impl Matrix<u8> {
    pub fn from(input: &str) -> Self {
        let cols = input.find('\n').unwrap();
        let rows = (input.trim_end().len() + 1) / (cols + 1);
        let array = input
            .trim_end()
            .lines()
            .flat_map(|line| line.bytes())
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Matrix { array, rows, cols }
    }
}

impl<T> Matrix<T> {
    fn serialize(&self, (i, j): (i32, i32)) -> usize {
        self.cols * i as usize + j as usize
    }

    pub fn indices(&self) -> Product<std::ops::Range<i32>, std::ops::Range<i32>> {
        iproduct!(0..self.rows as i32, 0..self.cols as i32)
    }

    pub fn inner_indices(&self) -> Product<std::ops::Range<i32>, std::ops::Range<i32>> {
        iproduct!(1..(self.rows - 1) as i32, 1..(self.cols - 1) as i32)
    }
    pub fn swap(&mut self, ij0: (i32, i32), ij1: (i32, i32)) {
        let index1 = self.serialize(ij0);
        let index2 = self.serialize(ij1);
        self.array.swap(index1, index2);
    }

    pub fn new_uniform<U: Clone>(&self, value: U) -> Matrix<U> {
        Matrix::uniform(self.rows, self.cols, value)
    }
}

impl<T: Copy> Matrix<T> {
    pub fn get(&self, (i, j): (i32, i32)) -> Option<T> {
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

    fn index(&self, ij: (i32, i32)) -> &Self::Output {
        &self.array[self.serialize(ij)]
    }
}

impl<T> std::ops::IndexMut<(i32, i32)> for Matrix<T> {
    fn index_mut(&mut self, ij: (i32, i32)) -> &mut Self::Output {
        &mut self.array[self.serialize(ij)]
    }
}
