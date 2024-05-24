use std::{env, fs::File, io::Read};

fn main() {
    println!("{}", get_counts_sum());
}

fn get_counts_sum() -> u32 {
    let dir = env::current_dir().unwrap();
    let mut res = File::open(dir.join("input.txt")).ok().unwrap();
    let mut text = String::new();
    let _ = res.read_to_string(&mut text);

    let input_vec = text
        .lines()
        .map(|line| {
            let (groups, values) = line.split_once(' ').unwrap();
            let mut question_signs_amount = 0_usize;
            let mut hash_signs_amount = 0_usize;
            let input = groups
                .chars()
                .inspect(|c| {
                    if c == &'?' {
                        question_signs_amount += 1;
                    } else if c == &'#' {
                        hash_signs_amount += 1;
                    }
                })
                .collect::<Vec<_>>();
            (
                input,
                values
                    .split(',')
                    .map(|val| val.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
                question_signs_amount,
                hash_signs_amount,
            )
        })
        .collect::<Vec<_>>();

    let mut res = 0_u32;
    for (input, values, question_signs_amount, hash_signs_amount) in input_vec {
        let hashes_to_add_amount = values.iter().sum::<usize>() - hash_signs_amount;
        let size = 2_u32.pow(question_signs_amount as u32);
        let mut arrangments_amount = 0;
        for i in 0..size {
            let binary = format!("{:0>width$b}", i, width = question_signs_amount);
            if binary.chars().filter(|c| c == &'1').count() != hashes_to_add_amount {
                continue;
            }
            let temp_input =
                get_group_without_question_marks(binary.chars().collect::<Vec<_>>(), &input);
            if check_is_right_group(temp_input, &values) {
                arrangments_amount += 1;
            }
        }
        res += arrangments_amount;
    }

    res
}

fn get_group_without_question_marks(binary_group: Vec<char>, input: &[char]) -> Vec<char> {
    let mut index = 0;
    input
        .iter()
        .map(|c| {
            if c == &'?' {
                let val = if binary_group[index] == '1' { '#' } else { '.' };
                index += 1;
                return val;
            }
            *c
        })
        .collect::<Vec<_>>()
}

fn check_is_right_group(group: Vec<char>, val: &Vec<usize>) -> bool {
    let mut res = vec![];
    let mut counter = 0;
    for c in group {
        match c {
            '#' => {
                counter += 1;
            }
            _ => {
                if counter > 0 {
                    res.push(counter);
                }
                counter = 0;
            }
        }
    }
    if counter > 0 {
        res.push(counter);
    }

    val == &res
}
