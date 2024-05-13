use std::{env, fs::File, io::Read};

fn main() {
    println!("{}", get_sum())
}

fn get_sum() -> u32 {
    let dir = env::current_dir().unwrap();
    let file = File::open(dir.join("src").join("input.txt"));
    let mut text = String::new();
    let mut sum = 0;
    if let Ok(mut res) = file {
        let _ = res.read_to_string(&mut text);
        text.split('\n').for_each(|s| {
            let mut first = 10;
            let mut last = 0;
            let mut str_digit = String::new();
            for c in s.chars() {
                let digit: u32;
                match c.to_digit(10) {
                    Some(num) => {
                        digit = num;
                        str_digit.clear();
                    }
                    None => {
                        str_digit.push(c);
                        digit = get_digit_from_str(&str_digit);
                        if digit == 10 {
                            continue;
                        }
                    }
                }
                if first == 10 {
                    first = digit;
                }
                last = digit;
            }
            if first < 10 {
                sum += first * 10 + last;
            }
        });
    }
    sum
}

fn get_digit_from_str(input: &str) -> u32 {
    let mut res = 10;

    if input.ends_with("one") {
        res = 1;
    } else if input.ends_with("two") {
        res = 2;
    } else if input.ends_with("three") {
        res = 3;
    } else if input.ends_with("four") {
        res = 4;
    } else if input.ends_with("five") {
        res = 5;
    } else if input.ends_with("six") {
        res = 6;
    } else if input.ends_with("seven") {
        res = 7;
    } else if input.ends_with("eight") {
        res = 8;
    } else if input.ends_with("nine") {
        res = 9;
    }

    res
}
