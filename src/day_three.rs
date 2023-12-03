use std::collections::HashMap;

#[allow(dead_code)]
pub fn part_one() {
    let now = std::time::Instant::now();

    let file_path = "resources/input-day-three.txt";
    let file_content = std::fs::read_to_string(file_path).unwrap();

    let input: Vec<Vec<char>> = file_content_string_to_engine_representation(&file_content);

    let mut my_map: HashMap<(usize, usize), i32> = HashMap::new();
    for (x, row) in input.iter().enumerate() {
        for (y, char) in row.iter().enumerate() {
            // for every symbol found check each surronding field and if there is a digit, then seek the beginning of the digit and put it into the map iwht its indices
        }
    }

    let elapsed_time = now.elapsed();
    println!("{input:?}, in {elapsed_time:?}");
}

fn file_content_string_to_engine_representation(file_content: &str) -> Vec<Vec<char>> {
    file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}
