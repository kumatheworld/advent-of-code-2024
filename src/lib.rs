pub mod template;

// Use this file to add helper functions and additional modules.
#[derive(Debug)]
pub struct Matrix<'a> {
    matrix: &'a [u8],
    pub rows: usize,
    pub cols: usize,
}
