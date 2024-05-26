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
    let mut is_smudge_detected = false;
    for (index, row) in pattern.windows(2).enumerate() {
        let (is_equal, smudged) = check_is_equal_rows(&row[0], &row[1], is_smudge_detected);
        if is_equal {
            is_smudge_detected = smudged;
            for i in 1..pattern.len() {
                let index_prev = index as i32 - i as i32;
                let index_next = index + 1 + i;
                if index_prev < 0 || index_next == pattern.len() {
                    if is_smudge_detected {
                        return Some(index + 1);
                    } else {
                        is_smudge_detected = false;
                        break;
                    }
                }
                let (is_equal, smudged) = check_is_equal_rows(
                    &pattern[index_prev as usize],
                    &pattern[index_next],
                    is_smudge_detected,
                );
                if is_equal {
                    is_smudge_detected = smudged;
                }
                if !is_equal {
                    is_smudge_detected = false;
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

fn check_is_equal_rows(row1: &[char], row2: &[char], mut with_smudge: bool) -> (bool, bool) {
    for (c1, c2) in row1.iter().zip(row2) {
        if c1 != c2 {
            if !with_smudge {
                with_smudge = true;
            } else {
                return (false, true);
            }
        }
    }

    (true, with_smudge)
}
