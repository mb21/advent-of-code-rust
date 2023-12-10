advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input.lines().map(|line| {
        let mut digits = line.chars().filter(|c| c.is_digit(10));
        let first = digits.next().expect("first should be there");
        let last = digits.next_back().unwrap_or_else(|| first);
        let str = format!("{}{}", first, last);
        str.parse::<u32>().unwrap_or_else(|x| panic!("{}", x))
    }).sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum = input.lines().map(|line| {
        let fst = get_nr(line.chars().collect(), NAMES);
        let lst = get_nr(line.chars().rev().collect(), NAMES_REV);
        let str = format!("{}{}", fst, lst);
        str.parse::<u32>().unwrap_or_else(|x| panic!("{}", x))
    }).sum();
    Some(sum)
}

const NAMES: &[&str] = &[
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

const NAMES_REV: &[&str] = &[
    "eno",
    "owt",
    "eerht",
    "ruof",
    "evif",
    "xis",
    "neves",
    "thgie",
    "enin",
];

fn get_nr(chars: Vec<char>, names: &[&str]) -> u32 {
    for i in 0..chars.len() {
        match names.iter().position(|&name|
            chars[0..i].iter().collect::<String>().contains(name)
        ) {
            Some(i) => {
                return i as u32 + 1;
            }
            None => {
                match chars[i].to_digit(10) {
                    Some(d) => { return d }
                    None => {}
                }
            }
        }
    }
    return 0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
