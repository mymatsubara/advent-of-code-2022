#![allow(unstable_name_collisions)]
use std::{fs, time::Instant};

use itertools::Either;
use matrix::Matrix;

fn part_one(input: &[String]) -> String {
    let trees = parse_trees(input);
    let blocked = get_blocked_trees(&trees);

    blocked.iter().filter(|b| !**b).count().to_string()
}

fn part_two(input: &[String]) -> String {
    let trees = parse_trees(input);
    let scenic_scores = get_scenic_scores(&trees);

    scenic_scores
        .iter()
        .max()
        .expect("should contain max value")
        .to_string()
}

fn get_blocked_trees(trees: &Matrix<u8>) -> Matrix<bool> {
    let mut blocked = Matrix::new(trees.rows, trees.cols, true);

    scan_blocked(trees, &mut blocked, ScanDirection::TopLeft);
    scan_blocked(trees, &mut blocked, ScanDirection::BottomRight);

    blocked
}

enum ScanDirection {
    TopLeft,
    BottomRight,
}

fn scan_blocked(trees: &Matrix<u8>, blocked: &mut Matrix<bool>, direction: ScanDirection) {
    let rows: Either<_, _> = match direction {
        ScanDirection::TopLeft => Either::Left(0..trees.rows),
        ScanDirection::BottomRight => Either::Right((0..trees.rows).rev()),
    };
    let cols: Either<_, _> = match direction {
        ScanDirection::TopLeft => Either::Left(0..trees.cols),
        ScanDirection::BottomRight => Either::Right((0..trees.rows).rev()),
    };
    let rows_bound = match direction {
        ScanDirection::TopLeft => 0,
        ScanDirection::BottomRight => trees.rows - 1,
    };
    let cols_bound = match direction {
        ScanDirection::TopLeft => 0,
        ScanDirection::BottomRight => trees.cols - 1,
    };

    let mut top = vec![0; trees.cols];
    let mut left = vec![0; trees.rows];

    for i in rows {
        for j in cols.clone() {
            let blocking_top = top.get_mut(j).unwrap();
            let blocking_left = left.get_mut(i).unwrap();

            let cur_height = trees.get(i, j).unwrap();
            let blocked = blocked.get_mut(i, j).unwrap();

            if cur_height > blocking_top || i == rows_bound {
                *blocked = false;
                *blocking_top = *cur_height;
            }

            if cur_height > blocking_left || j == cols_bound {
                *blocked = false;
                *blocking_left = *cur_height;
            }
        }
    }
}

fn get_scenic_scores(trees: &Matrix<u8>) -> Matrix<usize> {
    let mut scenic_scores = Matrix::new(trees.rows, trees.cols, 0);

    for i in 1..trees.rows - 1 {
        for j in 1..trees.cols - 1 {
            let cur_height = trees.get(i, j).unwrap();

            let mut top_scenic = 0;
            for i1 in (0..i).rev() {
                let top_height = trees.get(i1, j).unwrap();
                top_scenic += 1;
                if cur_height <= top_height {
                    break;
                }
            }

            let mut left_scenic = 0;
            for j1 in (0..j).rev() {
                let left_height = trees.get(i, j1).unwrap();
                left_scenic += 1;
                if cur_height <= left_height {
                    break;
                }
            }

            let mut bottom_scenic = 0;
            for i1 in (i + 1)..trees.rows {
                let bottom_height = trees.get(i1, j).unwrap();
                bottom_scenic += 1;
                if cur_height <= bottom_height {
                    break;
                }
            }

            let mut right_scenic = 0;
            for j1 in (j + 1)..trees.cols {
                let right_height = trees.get(i, j1).unwrap();
                right_scenic += 1;
                if cur_height <= right_height {
                    break;
                }
            }

            let scenic_score = scenic_scores.get_mut(i, j).unwrap();
            *scenic_score = top_scenic * left_scenic * bottom_scenic * right_scenic;
        }
    }

    scenic_scores
}

fn parse_trees(input: &[String]) -> Matrix<u8> {
    let first_line = input.first().expect("input should have one line");
    let cols = first_line.as_bytes().len();
    let rows = input.len();

    let mut matrix = Matrix::new(rows, cols, 0);

    for (i, line) in input.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let element = matrix.get_mut(i, j).expect("should not pass matrix bounds");
            *element = c.to_digit(10).expect("input should only contain digits") as u8;
        }
    }

    matrix
}

mod matrix {
    use itertools::Itertools;
    use std::fmt::{Debug, Display, Error, Formatter};

    pub struct Matrix<T: Clone> {
        pub rows: usize,
        pub cols: usize,
        elements: Vec<T>,
    }

    impl<T: Clone> Matrix<T> {
        pub fn new(rows: usize, cols: usize, default: T) -> Self {
            Matrix {
                rows,
                cols,
                elements: vec![default; rows * cols],
            }
        }

        pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
            self.elements.get_mut(i * self.cols + j)
        }

        pub fn get(&self, i: usize, j: usize) -> Option<&T> {
            self.elements.get(i * self.cols + j)
        }

        pub fn iter(&self) -> impl Iterator<Item = &T> {
            self.elements.iter()
        }
    }

    impl<T: Clone + Display> Debug for Matrix<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            for row in self.elements.chunks(self.cols) {
                writeln!(
                    f,
                    "{}",
                    row.iter()
                        .map(|e| e.to_string())
                        .intersperse(", ".to_string())
                        .collect::<String>()
                )?;
            }
            Ok(())
        }
    }
}

// --- TESTS ---

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = parse_input(true);
        let result = part_one(&input);
        assert_eq!(result, "21");
    }

    #[test]
    fn test_part_two() {
        let input = parse_input(true);
        let result = part_two(&input);
        assert_eq!(result, "8");
    }
}

// --- Lines bellow do not need to be modified ---

fn main() {
    let input = parse_input(false);

    let start_one = Instant::now();
    let result_one = part_one(&input);
    let elapsed_one = start_one.elapsed();

    let start_two = Instant::now();
    let result_two = part_two(&input);
    let elapsed_two = start_two.elapsed();

    println!("Part one result: {result_one} [time: {:.2?}]", elapsed_one);
    println!("Part two result: {result_two} [time: {:.2?}]", elapsed_two);
}

fn parse_input(test: bool) -> Vec<String> {
    let file = if test { "input.test.txt" } else { "input.txt" };

    fs::read_to_string(file)
        .unwrap_or_else(|_| panic!("'{file}' not found"))
        .lines()
        .map(|line| line.trim().to_owned())
        .collect()
}
