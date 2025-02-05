use advent_of_code::{Matrix, DIJ, IJ};

advent_of_code::solution!(6);

fn patrol(mat: &mut Matrix<u8>, ij0: IJ) -> Option<u32> {
    let mut d = 0;
    let mut ij = ij0;

    mat[ij] = b'X';
    let mut sum = 1;
    // mat.rows * mat.cols times should be enough to see if there's a loop
    for _ in 0..mat.rows * mat.cols {
        ij += DIJ[d];
        match mat.get(ij) {
            Some(b'.') => {
                sum += 1;
                mat[ij] = b'X';
            }
            Some(b'#') => {
                ij -= DIJ[d];
                d = (d + 1) % 4;
            }
            Some(b'X') => continue,
            Some(_) => unreachable!(),
            None => return Some(sum),
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut mat = Matrix::from(input);
    let ij0 = mat.position(b'^').unwrap();
    patrol(&mut mat, ij0)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mat0 = Matrix::from(input);
    let ij0 = mat0.position(b'^').unwrap();
    let mut mat1 = mat0.clone();
    patrol(&mut mat1, ij0);

    let IJ((i0, j0)) = ij0;
    Some(
        mat0.indices()
            .filter(|&ij| {
                let IJ((i, j)) = ij;
                mat1[ij] == b'X' && (i != i0 || j != j0) && {
                    let mut mat = mat0.clone();
                    mat[ij] = b'#';
                    patrol(&mut mat, ij0).is_none()
                }
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
