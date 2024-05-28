use std::collections::HashMap;

fn main() {
    println!("{}", get_total_load());
}

const CYCLE_AMOUNT: u32 = 1_000_000_000;

fn get_total_load() -> usize {
    let input = include_str!("../input.txt");
    let mut input_vec = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut map = HashMap::new();

    map.insert(input_vec.clone(), 0);
    let mut res_index = 0;
    let mut i = 0;
    loop {
        for _ in 0..4 {
            tilt_north(&mut input_vec);
            rotate_lever(&mut input_vec);
        }
        i += 1;
        let input = input_vec.clone();
        if let Some(index) = map.get(&input) {
            let diff = i - index;
            res_index = index + (CYCLE_AMOUNT - index) % diff;
            break;
        } else {
            map.insert(input, i);
        }
    }

    for (key, val) in map.iter() {
        if res_index == *val {
            return key.iter().enumerate().fold(0, |acc, (index, line)| {
                let load = line.iter().filter(|&i| i == &'O').count() * (input_vec.len() - index);
                acc + load
            });
        }
    }

    0
}

fn tilt_north(input_vec: &mut Vec<Vec<char>>) {
    for horizontal_index in 0..input_vec[0].len() {
        let mut height_to_adjust = input_vec.len();

        while height_to_adjust > 1 {
            let mut allow_height_change = true;
            for vertical_index in (1..height_to_adjust).rev() {
                let curr_item = input_vec[vertical_index][horizontal_index];
                let prev_item = input_vec[vertical_index - 1][horizontal_index];
                match (curr_item, prev_item) {
                    ('O', '.') => {
                        input_vec[vertical_index][horizontal_index] = '.';
                        input_vec[vertical_index - 1][horizontal_index] = 'O';
                        for ind in (1..height_to_adjust).rev() {
                            let i = input_vec[ind][horizontal_index];
                            if i == '.' {
                                allow_height_change = false;
                                break;
                            } else if i == '#' {
                                height_to_adjust -= 1;
                                break;
                            }
                        }
                    }
                    ('O', 'O') => {
                        for ind in (1..height_to_adjust).rev() {
                            let i = input_vec[ind][horizontal_index];
                            if i == '.' {
                                allow_height_change = false;
                                break;
                            } else if i == '#' {
                                height_to_adjust -= 1;
                                break;
                            }
                        }
                    }
                    _ => {
                        if allow_height_change {
                            height_to_adjust -= 1;
                        }
                    }
                };
            }
            if (1..height_to_adjust)
                .rev()
                .all(|i| input_vec[i][horizontal_index] != '.')
            {
                break;
            }
        }
    }
}

fn rotate_lever(lever: &mut Vec<Vec<char>>) {
    for i in 0..lever.len() {
        for k in i + 1..lever[i].len() {
            let temp = lever[i][k];
            lever[i][k] = lever[k][i];
            lever[k][i] = temp;
        }
    }

    for row in lever.iter_mut() {
        row.reverse();
    }
}
