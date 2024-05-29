fn main() {
    println!("{}", get_results_sum());
}

const BOXES_LENGTH: usize = 256;

#[derive(Debug)]
enum Sign {
    Equal,
    Minus,
}

#[derive(Debug)]
struct Lens {
    value: Vec<String>,
    sign: Sign,
    hash: usize,
}

impl Lens {
    fn new(input: &str) -> Self {
        let input = input.trim();
        if input.ends_with('-') {
            let mut input = input.to_owned();
            input.pop();
            Self {
                value: vec![input.clone()],
                sign: Sign::Minus,
                hash: get_hash(&input),
            }
        } else {
            let (lens, focal_length) = input.split_once('=').unwrap();
            Self {
                value: vec![lens.to_owned(), focal_length.to_owned()],
                sign: Sign::Equal,
                hash: get_hash(lens),
            }
        }
    }
}

fn get_results_sum() -> usize {
    let input = include_str!("../input.txt");
    let lens_vec = input.split(',').map(Lens::new).collect::<Vec<_>>();

    let mut boxes_vec: Vec<Vec<Vec<String>>> = vec![vec![]; BOXES_LENGTH];

    for lens in lens_vec {
        let curr_box = &mut boxes_vec[lens.hash];
        match lens.sign {
            Sign::Equal => match curr_box.iter().position(|s| s[0] == lens.value[0]) {
                Some(index) => {
                    curr_box[index][1] = lens.value[1].clone();
                }
                None => {
                    curr_box.push(lens.value);
                }
            },
            Sign::Minus => curr_box.retain(|s| s[0] != lens.value[0]),
        }
    }

    boxes_vec.iter().enumerate().fold(0, |acc, (index, curr)| {
        let box_result = curr.iter().enumerate().fold(0, |acc, (index, curr)| {
            acc + ((index + 1) * curr[1].parse::<usize>().unwrap())
        });
        acc + ((index + 1) * box_result)
    })
}

fn get_hash(input: &str) -> usize {
    input
        .trim()
        .chars()
        .fold(0, |acc, curr| (acc + (curr as usize)) * 17 % BOXES_LENGTH)
}
