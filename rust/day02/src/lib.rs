fn is_invalid_1(n: i64) -> bool {
    let string_n = n.to_string();

    let half_length = string_n.len() / 2;

    let first_half = &string_n[..half_length];
    let second_half = &string_n[half_length..];

    first_half == second_half
}

fn is_invalid_2(n: i64) -> bool {
    let string_n = n.to_string();

    for l in 1..string_n.len() {
        let parts = string_n.split(&string_n[..l]).collect::<Vec<&str>>();
        if parts.iter().all(|x| *x == "") {
            return true;
        }
    }

    false
}

pub fn part1(input: &str) -> String {
    input
        .split(',')
        .fold(0, |mut counter, range_string| {
            let ids = range_string
                .split('-')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i64>>();

            let (start, end) = (ids[0], ids[1]);

            for n in start..=end {
                if is_invalid_1(n) {
                    counter += n
                }
            }

            counter
        })
        .to_string()
}

pub fn part2(input: &str) -> String {
    input
        .split(',')
        .fold(0, |mut counter, range_string| {
            let ids = range_string
                .split('-')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i64>>();

            let (start, end) = (ids[0], ids[1]);

            for n in start..=end {
                if is_invalid_2(n) {
                    counter += n
                }
            }

            counter
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test_input.txt");
        assert_eq!(part1(input), "1227775554");
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../test_input.txt");
        assert_eq!(part2(input), "4174379265");
    }
}
