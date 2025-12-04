fn maximum_joltage_v1(bank: &str) -> i32 {
    let bank = bank.chars().collect::<Vec<char>>();
    let mut max: i32 = 0;
    for i in 0..bank.len() {
        for j in (i + 1)..bank.len() {
            let n = bank[i].to_digit(10).unwrap() * 10 + bank[j].to_digit(10).unwrap();

            if n as i32 > max {
                max = n as i32;
            }
        }
    }

    max
}

fn gospers_hack(n: u64) -> u64 {
    let n = n as i64;
    let c = n & -n;
    let r = n + c;
    return ((((r ^ n) >> 2) / c) | r) as u64;
}

fn maximum_joltage_v2(bank: &str) -> u64 {
    let bank = bank
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>();

    todo!()
}

pub fn part1(input: &str) -> String {
    let banks = input.lines();

    banks
        .fold(0, |acc, bank| acc + maximum_joltage_v1(bank))
        .to_string()
}

pub fn part2(input: &str) -> String {
    let banks = input.lines();

    banks
        .fold(0, |acc, bank| acc + maximum_joltage_v1(bank))
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test_input.txt");
        assert_eq!(part1(input), "357");
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../test_input.txt");
        assert_eq!(part2(input), "");
    }
}
