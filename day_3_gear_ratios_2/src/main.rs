use std::{env, fs::File, io::Read};

fn main() {
    println!("{}", get_ratios_sum());
}

fn get_ratios_sum() -> u32 {
    let dir = env::current_dir().unwrap();
    let mut res = File::open(dir.join("input.txt")).ok().unwrap();
    let mut text = String::new();
    let mut sum = 0;
    let _ = res.read_to_string(&mut text);
    let text_vec = text
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for (line_index, line) in text_vec.iter().enumerate() {
        for (item_index, item) in line.iter().enumerate() {
            if check_is_symbol(item) {
                sum += get_gear_ratio(&text_vec, line_index, item_index);
            }
        }
    }
    sum
}

fn get_gear_ratio(data: &Vec<Vec<char>>, line_index: usize, item_index: usize) -> u32 {
    let mut start_index = item_index as i32 - 1;
    if start_index < 0 {
        start_index = 0;
    }
    let mut end_index = item_index + 1;
    if end_index == data[0].len() {
        end_index = item_index;
    }
    let mut adjacent_values = Vec::<u32>::with_capacity(2);
    // Check is adjacent above
    if line_index > 0 {
        fill_adjacent_values(
            data,
            line_index - 1,
            start_index,
            end_index,
            &mut adjacent_values,
        );
        if adjacent_values.len() > 2 {
            return 0;
        }
    }
    // Check is adjacent below
    if line_index < data.len() - 1 {
        fill_adjacent_values(
            data,
            line_index + 1,
            start_index,
            end_index,
            &mut adjacent_values,
        );
        if adjacent_values.len() > 2 {
            return 0;
        }
    }
    // Check is adjacent horizontally
    fill_adjacent_values(
        data,
        line_index,
        start_index,
        end_index,
        &mut adjacent_values,
    );

    if adjacent_values.len() == 2 {
        return adjacent_values[0] * adjacent_values[1];
    }
    0
}

fn fill_adjacent_values(
    data: &[Vec<char>],
    line_index: usize,
    start_index: i32,
    end_index: usize,
    adjacent_values: &mut Vec<u32>,
) {
    let mut value = 0;
    let mut start_num_index = 0;
    for (index, item) in data[line_index][(start_index as usize)..=end_index]
        .iter()
        .enumerate()
    {
        if index + (start_index as usize) < start_num_index {
            continue;
        }
        if item.is_ascii_digit() {
            start_num_index = index + start_index as usize;
            while start_num_index > 0 {
                if data[line_index][start_num_index - 1].is_ascii_digit() {
                    start_num_index -= 1;
                } else {
                    break;
                }
            }

            while start_num_index < data[line_index].len() {
                if let Some(val) = data[line_index][start_num_index].to_digit(10) {
                    value = value * 10 + val;
                    if start_num_index == data[line_index].len() - 1 {
                        adjacent_values.push(value);
                        value = 0;
                    }
                    start_num_index += 1;
                } else {
                    adjacent_values.push(value);
                    value = 0;
                    break;
                }
            }
        }
    }
}

fn check_is_symbol(c: &char) -> bool {
    !c.is_ascii_digit() && c != &'.'
}
