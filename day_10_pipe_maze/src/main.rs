use std::{env, fs::File, io::Read};

fn main() {
    println!("{}", get_steps_amount());
}

fn get_steps_amount() -> u32 {
    let dir = env::current_dir().unwrap();
    let mut res = File::open(dir.join("input.txt")).ok().unwrap();
    let mut text = String::new();
    let _ = res.read_to_string(&mut text);
    let mut source_vec = vec![];
    let mut start = (0, 0);

    text.lines().enumerate().for_each(|(line_index, line)| {
        let line_vec = line
            .chars()
            .enumerate()
            .map(|(item_index, item)| {
                if item == 'S' {
                    start.0 = line_index;
                    start.1 = item_index;
                }
                item
            })
            .collect::<Vec<_>>();
        source_vec.push(line_vec);
    });

    let positions = get_start_directions(&source_vec, start);
    let mut position_1 = positions[0];
    let mut position_2 = positions[1];
    let mut prev_position_1 = start;
    let mut prev_position_2 = start;
    let mut step = 1;
    loop {
        step += 1;
        let temp1 = position_1;
        position_1 = get_next_position(
            source_vec[prev_position_1.0][prev_position_1.1],
            source_vec[position_1.0][position_1.1],
            position_1,
            prev_position_1,
        );
        prev_position_1 = temp1;
        if position_1 == position_2 {
            break;
        }

        let temp2 = position_2;
        position_2 = get_next_position(
            source_vec[prev_position_2.0][prev_position_2.1],
            source_vec[position_2.0][position_2.1],
            position_2,
            prev_position_2,
        );
        prev_position_2 = temp2;
        if position_1 == position_2 {
            break;
        }
    }

    step
}

fn get_start_directions(source_vec: &Vec<Vec<char>>, start: (usize, usize)) -> Vec<(usize, usize)> {
    let mut res = vec![];

    if start.0 > 0 {
        match source_vec[start.0 - 1][start.1] {
            '|' | 'F' | '7' => res.push((start.0 - 1, start.1)),
            _ => (),
        };
    }
    if start.0 < source_vec.len() - 1 {
        match source_vec[start.0 + 1][start.1] {
            '|' | 'L' | 'J' => res.push((start.0 + 1, start.1)),
            _ => (),
        };
    }
    if start.1 > 0 && res.len() != 2 {
        match source_vec[start.0][start.1 - 1] {
            '-' | 'F' | 'L' => res.push((start.0, start.1 - 1)),
            _ => (),
        };
    }
    if start.1 < source_vec[0].len() - 1 && res.len() != 2 {
        match source_vec[start.0][start.1 + 1] {
            '7' | '-' | 'J' => res.push((start.0, start.1 + 1)),
            _ => (),
        };
    }

    res
}

fn get_next_position(
    prev_c: char,
    c: char,
    mut position: (usize, usize),
    prev_position: (usize, usize),
) -> (usize, usize) {
    match (c, prev_c) {
        ('-', 'F' | 'L') => {
            position.1 += 1;
        }
        ('-', '7' | 'J') => {
            position.1 -= 1;
        }
        ('-', '-' | 'S') => {
            if prev_position.1 < position.1 {
                position.1 += 1;
            } else {
                position.1 -= 1;
            }
        }
        ('|', 'F' | '7') => {
            position.0 += 1;
        }
        ('|', 'L' | 'J') => {
            position.0 -= 1;
        }
        ('|', '|' | 'S') => {
            if prev_position.0 < position.0 {
                position.0 += 1;
            } else {
                position.0 -= 1;
            }
        }
        ('F', '-' | '7') => {
            position.0 += 1;
        }
        ('F', '|' | 'L') => {
            position.1 += 1;
        }
        ('F', 'J' | 'S') => {
            if prev_position.0 == position.0 {
                position.0 += 1;
            } else {
                position.1 += 1;
            }
        }
        ('L', '-' | 'J') => {
            position.0 -= 1;
        }
        ('L', '|' | 'F') => {
            position.1 += 1;
        }
        ('L', '7' | 'S') => {
            if prev_position.0 < position.0 {
                position.1 += 1;
            } else {
                position.0 -= 1;
            }
        }
        ('J', 'L' | '-') => {
            position.0 -= 1;
        }
        ('J', '7' | '|') => {
            position.1 -= 1;
        }
        ('J', 'F' | 'S') => {
            if prev_position.0 < position.0 {
                position.1 -= 1;
            } else {
                position.0 -= 1;
            }
        }
        ('7', '-' | 'F') => {
            position.0 += 1;
        }
        ('7', '|' | 'J') => {
            position.1 -= 1;
        }
        ('7', 'L' | 'S') => {
            if prev_position.0 == position.0 {
                position.0 += 1;
            } else {
                position.1 -= 1;
            }
        }
        _ => (),
    };
    position
}
