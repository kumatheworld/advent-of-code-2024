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
}
