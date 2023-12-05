use std::collections::HashMap;
use regex::Regex;

fn parse_int(s: &str) -> u32 {
    let word_to_number: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    match word_to_number.get(s) {
        Some(num) => num.parse::<u32>().unwrap(),
        None => s.parse::<u32>().unwrap(),
    }
}

pub fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let line_nums: String = line.chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>();
        let first = line_nums.chars().next().unwrap();
        let last = line_nums.chars().last().unwrap();
        let num = first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap();
        sum += num;
    }
    sum
}

// fn part1_regex(input: &str) -> u32 {
//     let mut sum = 0;
//     let re = Regex::new(r"^.*?(\d)(.*(\d))?.*$").unwrap();
//     for line in input.lines() {
//         let captures = re.captures(line).unwrap();
//         let first = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
//         let num;
//         if captures.get(2).is_none() {  // only one number
//             num = first * 10 + first;
//         } else {
//             let last = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();
//             num = first * 10 + last;
//         }
//         sum += num;
//     }
//     sum
// }

pub fn part2(input: &str) -> u32 {
    let mut sum = 0;
    let re = Regex::new(r"(?x)
        ^.*?
        (\d|one|two|three|four|five|six|seven|eight|nine)
        (.*
        (\d|one|two|three|four|five|six|seven|eight|nine)
        )?
        .*$
    ").unwrap();

    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let first = parse_int(captures.get(1).unwrap().as_str());
        if captures.get(2).is_none() {  // only one number
            sum += first * 10 + first;
            continue;
        }
        let last = parse_int(captures.get(3).unwrap().as_str());
        sum += first * 10 + last;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part1() {
        let input = r#"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"#;
        assert_eq!(part1(input), 142);
    }

    #[test]
    fn test_day1_part2() {
        let input = r#"two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"#;
        assert_eq!(part2(input), 281);
    }
}