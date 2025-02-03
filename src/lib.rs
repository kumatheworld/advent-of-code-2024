pub mod template;

// Use this file to add helper functions and additional modules.

use itertools::{iproduct, Product};
use std::fmt;

type Index = i32;
pub type IJ = (Index, Index);

pub const DIJ: [IJ; 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

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
        let rows = (input.trim_end_matches('\n').len() + 1) / (cols + 1);
        let array = input
            .trim_end_matches('\n')
            .lines()
            .flat_map(|line| line.bytes())
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Matrix { array, rows, cols }
    }
}

impl<T> Matrix<T> {
    fn serialize(&self, (i, j): IJ) -> usize {
        self.cols * i as usize + j as usize
    }

    pub fn indices(&self) -> Product<std::ops::Range<Index>, std::ops::Range<Index>> {
        iproduct!(0..self.rows as Index, 0..self.cols as Index)
    }

    pub fn inner_indices(&self) -> Product<std::ops::Range<Index>, std::ops::Range<Index>> {
        iproduct!(1..(self.rows - 1) as Index, 1..(self.cols - 1) as Index)
    }
    pub fn swap(&mut self, ij0: IJ, ij1: IJ) {
        let index1 = self.serialize(ij0);
        let index2 = self.serialize(ij1);
        self.array.swap(index1, index2);
    }

    pub fn new_uniform<U: Clone>(&self, value: U) -> Matrix<U> {
        Matrix::uniform(self.rows, self.cols, value)
    }
}

impl<T: Copy> Matrix<T> {
    pub fn get(&self, (i, j): IJ) -> Option<T> {
        ((0..self.rows as Index).contains(&i) && (0..self.cols as Index).contains(&j))
            .then(|| self[(i, j)])
    }
}

impl<T: PartialEq> Matrix<T> {
    pub fn position(&self, b: T) -> Option<IJ> {
        let index = self.array.iter().position(|b_| *b_ == b)?;
        Some(((index / self.cols) as Index, (index % self.cols) as Index))
    }
}

impl<T> std::ops::Index<IJ> for Matrix<T> {
    type Output = T;

    fn index(&self, ij: IJ) -> &Self::Output {
        &self.array[self.serialize(ij)]
    }
}

impl<T> std::ops::IndexMut<IJ> for Matrix<T> {
    fn index_mut(&mut self, ij: IJ) -> &mut Self::Output {
        &mut self.array[self.serialize(ij)]
    }
}
