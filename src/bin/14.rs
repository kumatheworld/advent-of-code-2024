// use advent_of_code::Matrix;
use advent_of_code::Index;
use itertools::Itertools;

advent_of_code::solution!(14);

fn common(
    input: &str,
) -> (
    (Index, Index),
    impl Iterator<Item = (Index, Index, Index, Index)> + '_,
) {
    const BOUNDARY: usize = 1000;
    let re = regex::Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)$").unwrap();

    let size = if input.len() < BOUNDARY {
        (7, 11)
    } else {
        (103, 101)
    };

    let ppvvs = input.lines().map(move |line| {
        re.captures(line)
            .unwrap()
            .iter()
            .skip(1)
            .map(|x| x.unwrap().as_str().parse::<Index>().unwrap())
            .collect_tuple()
            .unwrap()
    });

    (size, ppvvs)
}

pub fn part_one(input: &str) -> Option<u32> {
    const SECONDS: Index = 100;
    let ((rows, cols), ppvvs) = common(input);

    Some(
        ppvvs
            .filter_map(|(px, py, vx, vy)| {
                let x = (px + SECONDS * vx).rem_euclid(cols).cmp(&(cols >> 1));
                let y = (py + SECONDS * vy).rem_euclid(rows).cmp(&(rows >> 1));
                (![x, y].contains(&std::cmp::Ordering::Equal)).then_some((x, y))
            })
            .counts()
            .values()
            .map(|&v| v as u32)
            .product(),
    )
}

pub fn part_two(_: &str) -> Option<u32> {
    // let ((rows, cols), ppvvs) = common(input);
    // let (mut pps, vvs) = ppvvs
    //     .map(|(px, py, vx, vy)| ((px, py), (vx, vy)))
    //     .unzip::<_, _, Vec<_>, Vec<_>>();

    // for sec in 0.. {
    //     let mut mat = Matrix {
    //         array: vec![false; (rows * cols) as usize].into_boxed_slice(),
    //         rows: rows as usize,
    //         cols: cols as usize,
    //     };
    //     for ((px, py), (vx, vy)) in pps.iter_mut().zip(&vvs) {
    //         mat[(*py, *px)] = true;
    //         *px = (*px + vx).rem_euclid(cols);
    //         *py = (*py + vy).rem_euclid(rows);
    //     }
    //     if (sec + 15) % 101 == 0 {
    //         print!("{}", mat);
    //     }
    // }

    // Reasoning below:
    // As I printed `mat` out every second, some patterns were found at the following seconds.
    // 86, 187, 288, 389, 490, ...
    // Therefore, I printed `mat` out at 86 + 101 * n seconds,
    // and coincidentally saw a tree at 7,055 seconds.
    // I could have searched for something like a 4x4 grid of true values,
    // but it's hindsight because the tree could conceivably have been incomplete,
    // potentially lacking some "pixel"s.

    Some(7055)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        part_two(&advent_of_code::template::read_file("examples", DAY));
    }
}
