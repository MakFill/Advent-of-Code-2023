fn main() {
    println!("{}", get_total_load());
}

fn get_total_load() -> usize {
    let input = include_str!("../input.txt");
    let mut input_vec = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

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
                        if allow_height_change {
                            height_to_adjust -= 1;
                        }
                    }
                    ('O', 'O') => {
                        for ind in (0..height_to_adjust).rev() {
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
            if (0..height_to_adjust).all(|i| input_vec[i][horizontal_index] != '.') {
                break;
            }
        }
    }

    input_vec.iter().enumerate().fold(0, |acc, (index, line)| {
        let load = line.iter().filter(|&i| i == &'O').count() * (input_vec.len() - index);
        acc + load
    })
}
