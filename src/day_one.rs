pub fn day_one() {
    let now = std::time::Instant::now();

    let file_path = "resources/input-day-one.txt";
    let file_content = std::fs::read_to_string(file_path).unwrap();

    let sum: u32 = file_content
        .lines()
        .map(|x| {
            let l = x
                .chars()
                .find(|c| c.is_digit(10))
                .unwrap()
                .to_digit(10)
                .unwrap();
            let r = x
                .chars()
                .rev()
                .find(|c| c.is_digit(10))
                .unwrap()
                .to_digit(10)
                .unwrap();
            // println!("{l:?} {r:?}");
            l * 10 + r
        })
        .sum();

    let elapsed_time = now.elapsed();
    println!("{sum:?}, in {elapsed_time:?}");
}
