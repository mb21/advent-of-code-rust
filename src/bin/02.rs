advent_of_code::solution!(2);

pub mod util;
use util::str_to_u32_or_panic;
use std::cmp::max;

pub fn part_one(input: &str) -> Option<u32> {
    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;

    let sum = input.lines().filter_map(|line| {
        let (game_str, draws) = line.split_once(": ").expect(": should be there");
        let is_valid_game = draws.split("; ").all(|draw| {
            draw.split(", ").all(|cube| {
                if cube.ends_with("red") {
                    let r = &cube[0..cube.len()-4];
                    str_to_u32_or_panic(r) <= red_cubes
                } else if cube.ends_with("green") {
                    let g = &cube[0..cube.len()-6];
                    str_to_u32_or_panic(g) <= green_cubes
                } else if cube.ends_with("blue") {
                    let b = &cube[0..cube.len()-5];
                    str_to_u32_or_panic(b) <= blue_cubes
                } else {
                  panic!("cubes_str was {}", cube);
                }
            })
        });
        if is_valid_game {
            let game_id = str_to_u32_or_panic(&game_str[5..]);
            Some(game_id)
        } else {
            None
        }
    }).sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum = input.lines().map(|line| {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let (_, draws) = line.split_once(": ").expect(": should be there");

        for draw in draws.split("; ") {
            for cube in draw.split(", ") {
                if cube.ends_with("red") {
                    let nr_r = &cube[0..cube.len()-4];
                    r = max(r, str_to_u32_or_panic(nr_r));
                } else if cube.ends_with("green") {
                    let nr_g = &cube[0..cube.len()-6];
                    g = max(g, str_to_u32_or_panic(nr_g));
                } else if cube.ends_with("blue") {
                    let nr_b = &cube[0..cube.len()-5];
                    b = max(b, str_to_u32_or_panic(nr_b));
                } else {
                  panic!("cubes_str was {}", cube);
                }
            }
        }
        r * g * b
    }).sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
