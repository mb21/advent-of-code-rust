pub mod util;
use util::str_to_u32_or_panic;
use std::cmp::min;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut numbers = Vec::<String>::new();
    let vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let part_number = |nr_str: &str, line: usize, start: usize, end: usize| -> u32 {
        for i in line.saturating_sub(1)..min(line+2, vec.len() - 1) {
            for j in start.saturating_sub(1)..min(end+1, vec[i].len() - 1) {
                if i != line || (j == start.saturating_sub(1) || j == end) {
                    let char = vec[i][j];
                    if char != '.' && !char.is_digit(10) {
                        return str_to_u32_or_panic(nr_str)
                    }
                }
            }
        }
        0
    };

    let mut sum = 0;
    for (i, line) in vec.iter().enumerate() {
        let mut parsing_nr = false;
        for (j, char) in line.iter().enumerate() {
            if char.is_digit(10) {
                if parsing_nr {
                    numbers.last_mut().expect("last element should be there")
                        .push_str(&char.to_string());
                } else {
                    numbers.push(char.to_string());
                    parsing_nr = true;
                }
            } else {
                if parsing_nr {
                    let nr_str = numbers.last().expect("number should be there");
                    sum += part_number(nr_str, i, j - nr_str.len(), j);
                }
                parsing_nr = false;
            }
        }
        if parsing_nr {
            let nr_str = numbers.last().expect("number should be there");
            sum += part_number(nr_str, i, line.len() - nr_str.len(), line.len());
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut numbers = Vec::<String>::new();
    let vec: Vec<Vec<Option<&String>>> = input.lines().map(|line| {
        let mut parsing_nr = false;
        line.chars().map(|char| {
            if char.is_digit(10) {
                if parsing_nr {
                    numbers.last_mut().expect("last element should be there")
                      .push_str(&char.to_string());
                    numbers.last()
                } else {
                    let s = char.to_string();
                    numbers.push(s);
                    parsing_nr = true;
                    Some(&s)
                }
            } else {
                parsing_nr = false;
                None
            }
        }).collect()
    }).collect();


    let gear_ratio = |line: usize, start: usize, end: usize| -> u32 {
        let mut nr_of_adjacent_numbers = 0;
        let mut product = 1;
        for i in line.saturating_sub(1)..min(line+2, vec.len() - 1) {
            for j in start.saturating_sub(1)..min(end+1, vec[i].len() - 1) {
                if i != line || (j == start.saturating_sub(1) || j == end) {
                    if let Some(s) = vec[i][j] {
                        nr_of_adjacent_numbers += 1;
                        product *= str_to_u32_or_panic(s);
                    }
                }
            }
        }
        if nr_of_adjacent_numbers == 2 {
            product
        } else {
            0
        }
    };

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
