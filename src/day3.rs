use std::collections::{HashMap, HashSet};
use regex::Regex;

fn find_numbers_by_symbol_idx(symbol_i: usize, symbol_j: usize, num_indexes: &HashMap<(usize, usize), i32>) -> HashSet<i32> {
    let pre_row = symbol_i - 1;
    let next_row = symbol_i + 1;
    let pre_col = symbol_j - 1;
    let next_col = symbol_j + 1;
    let mut numbers: HashSet<i32> = HashSet::new();
    for row in pre_row..=next_row {
        for col in pre_col..=next_col {
            let number = num_indexes.get(&(row, col));
            if let Some(n) = number {
                let _ = numbers.insert(*n);
            }
        }
    }
    numbers
}

fn parse_symbol_and_number(input: &str, symbol_re: Regex) -> (HashMap<(usize, usize), i32>, Vec<(usize, usize)>) {
    let number_re = Regex::new(r"\d+").unwrap();
    let mut num_indexes: HashMap<(usize, usize), i32> = HashMap::new();
    let mut symbols: Vec<(usize, usize)> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        // store the row index and the column index range of the number
        let numbers_matches: Vec<_> = number_re
            .find_iter(line)
            .filter(|m| !m.is_empty())
            .map(|m| (m.range(), m.as_str().parse::<i32>().unwrap()))
            .collect();
        for (range, number) in numbers_matches {
            for j in range {
                let _ = num_indexes.insert((i, j), number);
            }
        }
        let symbol_idxs: Vec<(usize, usize)> = symbol_re.find_iter(line)
            .filter(|m| !m.is_empty())
            .map(|m| (i, m.range().start)).collect();
        symbols.extend(&symbol_idxs);
    }
    (num_indexes, symbols)
}

pub fn part1(input: &str) -> i32 {
    let symbol_re = Regex::new(r"[^\d,^.]").unwrap();
    let mut part_numbers: Vec<i32> = Vec::new();
    let (num_indexes, symbols) = parse_symbol_and_number(input, symbol_re);
    for (i, j) in symbols {
        let nums = find_numbers_by_symbol_idx(i, j, &num_indexes);
        part_numbers.extend(nums);
    }
    part_numbers.iter().sum()
}

pub fn part2(input: &str) -> i32 {
    let symbol_re = Regex::new(r"[*]").unwrap();
    let mut part_numbers: Vec<i32> = Vec::new();
    let (num_indexes, symbols) = parse_symbol_and_number(input, symbol_re);
    for (i, j) in symbols {
        let nums = find_numbers_by_symbol_idx(i, j, &num_indexes);
        if nums.len() == 2 {
            let gear_ratio = nums.iter().product();
            part_numbers.push(gear_ratio);
        }
    }
    part_numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn day3_part1() {
        assert_eq!(part1(INPUT), 4361);
    }

    #[test]
    fn day3_part2() {
        assert_eq!(part2(INPUT), 467835);
    }
}
