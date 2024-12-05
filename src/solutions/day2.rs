pub fn solve() {
    // Part 1 solution
    println!("Day 2 - Part 1 solution");
    let part1_result = solve_part1("./input/day2.txt");
    println!("Part 1 Result: {}", part1_result);

    // // Part 2 solution
    // println!("Day 2 - Part 2 solution");
    // let part2_result = solve_part2("./input/day2.txt");
    // println!("Part 2 Result: {}", part2_result);
}

use std::fs;

fn get_arrays(inputfile: &str) -> Vec<Vec<i32>> {
    let input = fs::read_to_string(inputfile).expect("Should have been able to read the file");

    let lines = input.split("\n");
    let mut array: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        array.push(numbers);
    }

    array
}

fn solve_part1(inputfile: &str) -> i32 {
    let array = get_arrays(inputfile);

    let mut num_safe = 0;
    for i in 0..array.len() {
        let mut is_safe = true;
        let mut last_num = array[i][0];
        let check_num = array[i][1];
        let is_increasing = check_num - last_num > 0;
        for j in 1..array[i].len() {
            let this_num = array[i][j];
            if is_increasing {
                if this_num < last_num {
                    is_safe = false;
                    break;
                }
            } else {
                if this_num > last_num {
                    is_safe = false;
                    break;
                }
            }
            if (this_num - last_num).abs() > 3 || this_num == last_num {
                is_safe = false;
                break;
            }
            last_num = this_num;
        }
        if is_safe {
            num_safe += 1;
        }
    }

    num_safe
}

// fn solve_part2(inputfile: &str) -> i32 {
//     // TODO implement
//     4
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        assert_eq!(solve_part1("./input/day2_sample.txt"), 2); // Replace with actual expected value
    }

    // #[test]
    // fn test_part2_sample() {
    //     assert_eq!(solve_part2("./input/day2_sample.txt"), 4); // Replace with actual expected value
    // }
}
