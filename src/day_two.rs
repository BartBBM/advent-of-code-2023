#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<(i32, i32, i32)>,
}

#[allow(dead_code)]
pub fn part_one() {
    let now = std::time::Instant::now();

    let file_path = "resources/input-day-two.txt";
    let file_content = std::fs::read_to_string(file_path).unwrap();

    let games: Vec<Game> = file_content_string_to_games(&file_content);

    let max_possible = (12, 13, 14);
    let sum_of_ids = games
        .iter()
        .filter(|game| {
            game.sets.iter().all(|set| {
                set.0 <= max_possible.0 && set.1 <= max_possible.1 && set.2 <= max_possible.2
            })
        })
        .fold(0, |acc, game| acc + game.id);

    let elapsed_time = now.elapsed();
    println!("{sum_of_ids:?}, in {elapsed_time:?}");
}

#[allow(dead_code)]
pub fn part_two() {
    let now = std::time::Instant::now();

    let file_path = "resources/input-day-two.txt";
    let file_content = std::fs::read_to_string(file_path).unwrap();

    let games: Vec<Game> = file_content_string_to_games(&file_content);

    let power_of_min_sets = games
        .iter()
        .map(|game| {
            game.sets.iter().fold((0, 0, 0), |acc, set| {
                (acc.0.max(set.0), acc.1.max(set.1), acc.2.max(set.2))
            })
        })
        .fold(0, |acc, set| acc + (set.0 * set.1 * set.2));

    let elapsed_time = now.elapsed();
    println!("{power_of_min_sets:?}, in {elapsed_time:?}");
}

fn file_content_string_to_games(file_content: &str) -> Vec<Game> {
    file_content
        .lines()
        .map(|x| {
            // "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            let (id_substr, sets_substr) = x.split_once(':').unwrap();
            let id: i32 = id_substr.split_once(' ').unwrap().1.parse().unwrap();
            let sets = sets_substr
                .split(';')
                .map(|y| {
                    // " 1 red, 2 green, 6 blue"
                    let mut set = (0, 0, 0);
                    y.split(',').for_each(|z| {
                        // " 1 red"
                        let (amount, color) = z.trim().split_once(' ').unwrap();
                        let amount = amount.parse().unwrap();
                        match color {
                            "red" => set.0 = amount,
                            "green" => set.1 = amount,
                            "blue" => set.2 = amount,
                            _ => panic!(),
                        }
                    });
                    set
                })
                .collect();
            println!("{id:?} {sets:?}");
            Game { id, sets }
        })
        .collect()
}
