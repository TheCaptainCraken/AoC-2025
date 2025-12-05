const NEIGHBORS_OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn neighbors(row: usize, column: usize, matrix: &Vec<Vec<bool>>) -> impl Iterator<Item = bool> {
    let rows = matrix.len() as isize;
    let columns = matrix.get(0).map_or(0, |row| row.len()) as isize;

    NEIGHBORS_OFFSETS
        .iter()
        .filter_map(move |(row_offset, column_offset)| {
            let new_row = row as isize + row_offset;
            let new_column = column as isize + column_offset;

            if new_row >= 0 && new_row < rows && new_column >= 0 && new_column < columns {
                Some(matrix[new_row as usize][new_column as usize])
            } else {
                None
            }
        })
}

pub fn part1(input: &str) -> String {
    let diagram: Vec<Vec<bool>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '@' { true } else { false })
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();

    let mut answer = 0;
    for row in 0..diagram.len() {
        for col in 0..diagram[0].len() {
            if diagram[row][col] == true {
                let number_of_adj_rolls =
                    neighbors(row, col, &diagram).filter(|n| *n == true).count();

                if number_of_adj_rolls < 4 {
                    answer += 1;
                }
            }
        }
    }

    answer.to_string()
}

pub fn part2(input: &str) -> String {
    let mut diagram: Vec<Vec<bool>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '@' { true } else { false })
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();

    let mut cleanup = true;
    let mut answer = 0;
    while cleanup {
        cleanup = false;
        for row in 0..diagram.len() {
            for col in 0..diagram[0].len() {
                if diagram[row][col] == true {
                    let number_of_adj_rolls =
                        neighbors(row, col, &diagram).filter(|n| *n == true).count();

                    if number_of_adj_rolls < 4 {
                        answer += 1;
                        diagram[row][col] = false;
                        cleanup = true;
                    }
                }
            }
        }
    }

    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test_input.txt");
        assert_eq!(part1(input), "13");
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../test_input.txt");
        assert_eq!(part2(input), "43");
    }
}
