const DIAL_START_POSITION: i32 = 50;
const MODULO_DIAL: i32 = 100;

pub fn part1(input: &str) -> String {
    input
        .lines()
        .fold((0, DIAL_START_POSITION), |(zeroes, dial_position), line| {
            let number: i32 = line.chars().skip(1).collect::<String>().parse().unwrap();
            let mut new_dial_position = dial_position;
            if line.chars().nth(0).unwrap() == 'L' {
                new_dial_position -= number;
            } else {
                new_dial_position += number;
            }

            new_dial_position %= MODULO_DIAL;

            if new_dial_position == 0 {
                (zeroes + 1, new_dial_position)
            } else {
                (zeroes, new_dial_position)
            }
        })
        .0
        .to_string()
}

pub fn part2(input: &str) -> String {
    input
        .lines()
        .fold(
            (0, DIAL_START_POSITION),
            |(zeros, current_dial_position), line| {
                let spin_amount: i32 = line.chars().skip(1).collect::<String>().parse().unwrap();

                let full_spins = spin_amount / MODULO_DIAL;

                if line.chars().nth(0).unwrap() == 'L' {
                    let new_dial_position =
                        (current_dial_position - spin_amount).rem_euclid(MODULO_DIAL);

                    if current_dial_position != 0
                        && (new_dial_position > current_dial_position || new_dial_position == 0)
                    {
                        (zeros + full_spins + 1, new_dial_position)
                    } else {
                        (zeros + full_spins, new_dial_position)
                    }
                } else {
                    let new_dial_position =
                        (current_dial_position + spin_amount).rem_euclid(MODULO_DIAL);

                    if new_dial_position < current_dial_position {
                        (zeros + full_spins + 1, new_dial_position)
                    } else {
                        (zeros + full_spins, new_dial_position)
                    }
                }
            },
        )
        .0
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../test_input.txt");
        assert_eq!(part1(input), "3");
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../test_input.txt");
        assert_eq!(part2(input), "6");
    }
}
