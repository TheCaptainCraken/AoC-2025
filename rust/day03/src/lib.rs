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

const K: usize = 12;

fn maximum_joltage_v2(bank: &str) -> u128 {
    let bank = bank
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u128)
        .rev()
        .collect::<Vec<u128>>();

    let mut stack = vec![0; K];

    for i in 0..K {
        stack[K - 1 - i] = bank[i];
    }

    for i in K..bank.len() {
        let mut holded = bank[i];

        let mut stack_i = 0;

        while stack_i < K && holded >= stack[stack_i] {
            let tmp = stack[stack_i];
            stack[stack_i] = holded;
            holded = tmp;
            stack_i += 1;
        }
    }

    stack
        .iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse::<u128>()
        .unwrap()
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
        .fold(0, |acc, bank| acc + maximum_joltage_v2(bank))
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
        assert_eq!(part2(input), "3121910778619");
    }

    #[test]
    fn test_example1() {
        let input = "987654321111111";
        assert_eq!(maximum_joltage_v2(input), 987654321111);
    }

    #[test]
    fn test_example2() {
        let input = "811111111111119";
        assert_eq!(maximum_joltage_v2(input), 811111111119);
    }
    #[test]
    fn test_example3() {
        let input = "234234234234278";
        assert_eq!(maximum_joltage_v2(input), 434234234278);
    }
    #[test]
    fn test_example4() {
        let input = "818181911112111";
        assert_eq!(maximum_joltage_v2(input), 888911112111);
    }
}
