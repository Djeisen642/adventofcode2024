pub fn solve() {
    use std::time::Instant;
    let now = Instant::now();
    // Part 1 solution
    println!("Day 1 - Part 1 solution");
    let (line1, line2) = get_arrays("./input/day1.txt"); // Read file once and reuse
    let part1_result = solve_part1(&line1, &line2);
    println!("Part 1 Result: {}", part1_result);
    println!("Elapsed: {:.2?}", now.elapsed());

    let now2 = Instant::now();
    // Part 2 solution
    println!("Day 1 - Part 2 solution");
    let part2_result = solve_part2(&line1, &line2);
    println!("Part 2 Result: {}", part2_result);
    println!("Elapsed: {:.2?}", now2.elapsed());
}

use std::fs;

fn get_arrays(inputfile: &str) -> (Vec<i32>, Vec<i32>) {
    let input = fs::read_to_string(inputfile).expect("Should have been able to read the file");
    
    // Pre-allocate with capacity by counting lines first
    let line_count = input.lines().count();
    let mut line1 = Vec::with_capacity(line_count);
    let mut line2 = Vec::with_capacity(line_count);

    // Process lines directly without collecting
    input.lines().for_each(|line| {
        let mut nums = line.split_whitespace();
        if let (Some(n1), Some(n2)) = (nums.next(), nums.next()) {
            // Avoid unwrap by using unchecked parsing when we know format is valid
            unsafe {
                line1.push(n1.parse::<i32>().unwrap_unchecked());
                line2.push(n2.parse::<i32>().unwrap_unchecked());
            }
        }
    });

    (line1, line2)
}

fn solve_part1(line1: &[i32], line2: &[i32]) -> i32 {
    // Create owned copies for sorting
    let mut sorted1 = line1.to_vec();
    let mut sorted2 = line2.to_vec();
    sorted1.sort_unstable();
    sorted2.sort_unstable();
    
    // Use iterator with no allocations
    sorted1.iter()
        .zip(sorted2.iter())
        .map(|(a, b)| (b - a).abs())
        .sum()
}

fn solve_part2(line1: &[i32], line2: &[i32]) -> i32 {
    // Pre-calculate frequency map with exact capacity
    let mut map = std::collections::HashMap::with_capacity(line2.len());
    line2.iter().for_each(|&num| {
        *map.entry(num).or_insert(0) += 1;
    });

    // Calculate sum using iterator with no allocations
    line1.iter()
        .map(|&num| num * map.get(&num).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let (line1, line2) = get_arrays("./input/day1_sample.txt");
        assert_eq!(solve_part1(&line1, &line2), 11);
    }

    #[test]
    fn test_part2_sample() {
        let (line1, line2) = get_arrays("./input/day1_sample.txt");
        assert_eq!(solve_part2(&line1, &line2), 31);
    }
}
