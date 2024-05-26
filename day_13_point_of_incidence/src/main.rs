use std::{env, fs::File, io::Read};

fn main() {
    println!("{}", get_notes_sum());
}

fn get_notes_sum() -> usize {
    let dir = env::current_dir().unwrap();
    let mut res = File::open(dir.join("input.txt")).ok().unwrap();
    let mut text = String::new();
    let _ = res.read_to_string(&mut text);
    let mut patterns_vec = vec![];

    let mut temp_vec = vec![];
    for line in text.lines() {
        if line.is_empty() {
            patterns_vec.push(temp_vec.clone());
            temp_vec.clear();
            continue;
        }
        temp_vec.push(line.chars().collect::<Vec<_>>());
    }
    patterns_vec.push(temp_vec);

    let mut res = 0;
    for pattern in patterns_vec {
        if let Some(val) = get_horizontal_rows(&pattern) {
            res += val * 100;
        } else {
            res += get_vertical_columns(pattern).unwrap_or(0);
        }
    }

    res
}

fn get_horizontal_rows(pattern: &[Vec<char>]) -> Option<usize> {
    for (index, row) in pattern.windows(2).enumerate() {
        if row[0] == row[1] {
            for i in 1..pattern.len() {
                let index_prev = index as i32 - i as i32;
                let index_next = index + 1 + i;
                if index_prev < 0 || index_next == pattern.len() {
                    return Some(index + 1);
                } else if pattern[index_prev as usize] != pattern[index_next] {
                    break;
                }
            }
        }
    }
    None
}

fn get_vertical_columns(pattern: Vec<Vec<char>>) -> Option<usize> {
    let rows = pattern.len();
    let cols = pattern[0].len();
    let mut rotated = vec![vec!['.'; rows]; cols];

    for i in 0..rows {
        for k in 0..cols {
            rotated[k][rows - 1 - i] = pattern[i][k];
        }
    }

    get_horizontal_rows(&rotated)
}
