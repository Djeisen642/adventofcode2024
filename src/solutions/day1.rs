pub fn solve() {
    // Part 1 solution
    println!("Day 1 - Part 1 solution");
    let part1_result = solve_part1("./input/day1.txt");
    println!("Part 1 Result: {}", part1_result);

    // Part 2 solution
    println!("Day 1 - Part 2 solution");
    let part2_result = solve_part2("./input/day1.txt");
    println!("Part 2 Result: {}", part2_result);
}

use std::fs;

fn get_arrays(inputfile: &str) -> (Vec<i32>, Vec<i32>) {
    let input = fs::read_to_string(inputfile).expect("Should have been able to read the file");

    let lines = input.split("\n");
    let count = lines.clone().count();
    let mut line1: Vec<i32> = vec![0; count];
    let mut line2: Vec<i32> = vec![0; count];
    for (i, line) in lines.enumerate() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        line1[i] = numbers[0];
        line2[i] = numbers[1];
    }

    (line1, line2)
}

fn solve_part1(inputfile: &str) -> i32 {
    let (mut line1, mut line2) = get_arrays(inputfile);
    line1.sort();
    line2.sort();
    let mut sum = 0;
    for i in 0..line1.len() {
        sum += (line2[i] - line1[i]).abs();
    }

    sum
}

fn solve_part2(inputfile: &str) -> i32 {
    let (line1, line2) = get_arrays(inputfile);
    let mut sum = 0;

    let mut map = std::collections::HashMap::new();
    for i in 0..line2.len() {
        let entry = map.entry(line2[i]).or_insert(0);
        *entry += 1;
    }

    for i in 0..line1.len() {
        let entry = map.entry(line1[i]).or_insert(0);
        sum += *entry * line1[i];
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        assert_eq!(solve_part1("./input/day1_sample.txt"), 11); // Replace with actual expected value
    }

    #[test]
    fn test_part2_sample() {
        assert_eq!(solve_part2("./input/day1_sample.txt"), 31); // Replace with actual expected value
    }
}
