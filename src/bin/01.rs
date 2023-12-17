use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .filter_map(|line| line.parse::<u32>().ok())
                .sum()
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut calories = input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .lines()
                .filter_map(|line| line.parse::<u32>().ok())
                .sum()
        })
        .collect_vec();
    calories.sort();

    Some(calories.iter().rev().take(3).sum::<u32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24000));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45000));
    }
}
