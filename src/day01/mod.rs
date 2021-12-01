pub fn solve() {
    let input = include_str!("input.txt");

    let depths = parse_input(input);

    println!("Part 1: {}", solve_part_1(&depths));
    println!("Part 2: {}", solve_part_2(&depths));
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

fn solve_part_1(depths: &Vec<u32>) -> u32 {
    count_increases(depths)
}

fn solve_part_2(depths: &Vec<u32>) -> u32 {
    let windows = depths.windows(3).map(|w| w.iter().sum()).collect();

    count_increases(&windows)
}

fn count_increases(data: &Vec<u32>) -> u32 {
    let mut increases = 0;
    let mut last = data[0];

    for value in data {
        if value > &last {
            increases += 1;
        }
        last = *value;
    }

    increases
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_with_sample() {
        let sample = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(solve_part_1(&sample), 7);
    }

    #[test]
    fn test_part_2_with_sample() {
        let sample = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(solve_part_2(&sample), 5);
    }

    #[test]
    fn test_part_1_with_input() {
        assert_eq!(solve_part_1(&parse_input(include_str!("input.txt"))), 1121);
    }

    #[test]
    fn test_part_2_with_input() {
        assert_eq!(solve_part_2(&parse_input(include_str!("input.txt"))), 1065);
    }
}
