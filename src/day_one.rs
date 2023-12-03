#[allow(dead_code)]
pub fn day_one_part_one() {
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

#[allow(dead_code)]
pub fn day_one_part_two() {
    let now = std::time::Instant::now();

    let file_path = "resources/input-day-one.txt";
    let file_content = std::fs::read_to_string(file_path).unwrap();

    let sum: u32 = file_content
        .lines()
        .map(|x| {
            let l = get_first_digit_or_word_digit(x);
            let r = get_last_digit_or_word_digit(x);
            // println!("{l:?} {r:?}");
            l * 10 + r
        })
        .sum();

    let elapsed_time = now.elapsed();
    println!("{sum:?}, in {elapsed_time:?}");
}

fn get_first_digit_or_word_digit(string: &str) -> u32 {
    let digits_written_out = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for i in 0..string.len() {
        let sub = &string[i..];
        if let Some(num) = sub.chars().next().unwrap().to_digit(10) {
            return num;
        }

        for (j, word_digit) in digits_written_out.iter().enumerate() {
            if sub.starts_with(word_digit) {
                return (j + 1) as u32;
            }
        }
    }
    panic!("should not heppen")
}

fn get_last_digit_or_word_digit(string: &str) -> u32 {
    let digits_written_out = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for i in 0..string.len() {
        let sub = &string[string.len() - (i + 1)..];
        if let Some(num) = sub.chars().next().unwrap().to_digit(10) {
            return num;
        }

        for (j, word_digit) in digits_written_out.iter().enumerate() {
            if sub.starts_with(word_digit) {
                return (j + 1) as u32;
            }
        }
    }
    panic!("should not heppen")
}

#[cfg(test)]
mod tests {
    use crate::day_one::{get_first_digit_or_word_digit, get_last_digit_or_word_digit};

    #[test]
    fn it_works() {
        let result = "two1nine";
        let l = get_first_digit_or_word_digit(result);
        let r = get_last_digit_or_word_digit(result);
        println!("{l:?} - {r:?}");
    }
}
