use std::cmp::max;

#[derive(Debug)]
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn new(id: u32, red: u32, green: u32, blue: u32) -> Self {
        Self {
            id,
            red,
            green,
            blue,
        }
    }
    fn impossible(&self) -> bool {
        self.red > 12 || self.green > 13 || self.blue > 14
    }
}

fn parse_lines(input: &str) -> Vec<Game> {
    let mut games = Vec::new();
    for line in input.lines() {
        let parts = line.split(":").collect::<Vec<&str>>();
        let id = parts[0].split(" ").collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let mut game = Game::new(id, 0, 0, 0);
        for cube_set in parts[1].split(";") {
            cube_set.trim().split(",").for_each(|cube| {
                let cube = cube.trim().split(" ").collect::<Vec<&str>>();
                let color = cube[1].trim();
                let count = cube[0].trim().parse::<u32>().unwrap();
                match color {
                    "red" => game.red = max(count, game.red),
                    "green" => game.green = max(count, game.green),
                    "blue" => game.blue = max(count, game.blue),
                    _ => panic!("Unknown color: {}", color),
                }
            });
        }
        games.push(game);
    }
    games
}

pub fn part1(input: &str) -> u32 {
    let mut sum = 0;
    let games = parse_lines(input);
    for game in games {
        if !game.impossible() {
            sum += game.id;
        }
    }
    sum
}

pub fn part2(input: &str) -> u32 {
    let mut sum = 0;
    let games = parse_lines(input);
    for game in games {
        let power = game.red * game.green * game.blue;
        sum += power;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT), 2286);
    }
}