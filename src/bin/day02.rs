fn main() {
    let games = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    let set = Set {
        blue: 14,
        red: 12,
        green: 13,
    };

    let possible_games = possible(set, games);
    let sum: usize = possible_games.iter().sum();

    println!("possible games are: {:?}", possible_games);
    println!("sum of ids is: {}", sum);
}

#[derive(Debug)]
struct Set {
    blue: usize,
    red: usize,
    green: usize,
}

impl From<&str> for Set {
    fn from(value: &str) -> Self {
        let parts = value
            .split(",")
            .map(|s| s.trim().split(" ").collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;

        for part in parts {
            match part[1] {
                "blue" => blue = part[0].parse::<usize>().unwrap(),
                "red" => red = part[0].parse::<usize>().unwrap(),
                "green" => green = part[0].parse::<usize>().unwrap(),
                _ => unreachable!(),
            }
        }

        Set { blue, red, green }
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let parts = value.split(":").collect::<Vec<&str>>();

        let id = parts[0].split(" ").collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .unwrap();

        let sets = parts[1]
            .split(";")
            .map(|s| Set::from(s))
            .collect::<Vec<_>>();

        Game { id, sets }
    }
}

fn possible(set: Set, games: &'static str) -> Vec<usize> {
    let mut ids = Vec::new();

    for line in games.lines() {
        let game = Game::from(line);
        let mut is_possible = true;

        for game_set in game.sets {
            if game_set.blue > set.blue || game_set.red > set.red || game_set.green > set.green {
                is_possible = false;
            }
        }

        if is_possible {
            ids.push(game.id);
        }
    }

    ids
}
