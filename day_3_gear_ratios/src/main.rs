use std::{env, fs::File, io::Read};

fn main() {
    println!("{}", get_numbers_sum());
}

fn get_numbers_sum() -> u32 {
    let dir = env::current_dir().unwrap();
    let mut res = File::open(dir.join("input.txt")).ok().unwrap();
    let mut text = String::new();
    let mut sum = 0;
    let _ = res.read_to_string(&mut text);
    let text_vec = text
        .split('\n')
        .map(|line| line.chars().filter(|&c| c != '\r').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for (line_index, line) in text_vec.iter().enumerate() {
        let mut number = 0;
        let mut num_length = 0_usize;
        for (item_index, &item) in line.iter().enumerate() {
            if let Some(val) = item.to_digit(10) {
                number = number * 10 + val;
                num_length += 1;
                if item_index == line.len() - 1
                    && check_is_adjacent_symbol(&text_vec, line_index, item_index, num_length - 1)
                {
                    sum += number;
                }
            } else {
                if number > 0
                    && check_is_adjacent_symbol(&text_vec, line_index, item_index, num_length)
                {
                    sum += number;
                }
                number = 0;
                num_length = 0;
            }
        }
    }
    sum
}

fn check_is_adjacent_symbol(
    data: &Vec<Vec<char>>,
    line_index: usize,
    item_index: usize,
    num_length: usize,
) -> bool {
    let mut start_index = (item_index as i32 - 1) - num_length as i32;
    if start_index < 0 {
        start_index = 0;
    }
    // Check is adjacent above
    if line_index > 0 {
        for item in data[line_index - 1][(start_index as usize)..=item_index].iter() {
            if check_is_symbol(item) {
                return true;
            }
        }
    }
    // Check is adjacent below
    if line_index < data.len() - 1 {
        for item in data[line_index + 1][(start_index as usize)..=item_index].iter() {
            if check_is_symbol(item) {
                return true;
            }
        }
    }
    // Check is adjacent horizontally
    if item_index > 0 {
        if check_is_symbol(&data[line_index][start_index as usize]) {
            return true;
        }
        if check_is_symbol(&data[line_index][item_index]) {
            return true;
        }
    }
    false
}

fn check_is_symbol(c: &char) -> bool {
    !c.is_ascii_digit() && c != &'.'
}
