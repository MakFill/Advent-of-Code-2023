use std::{env, fs::File, io::Read};

fn main() {
    println!("{}", get_ids_sum());
}

fn get_ids_sum() -> u32 {
    let dir = env::current_dir().unwrap();
    let file = File::open(dir.join("input.txt"));
    let mut text = String::new();
    let mut sum = 0;
    if let Ok(mut res) = file {
        let _ = res.read_to_string(&mut text);
        'main: for s in text.split('\n') {
            let str_vec = s.split_ascii_whitespace().collect::<Vec<_>>();
            let id = &str_vec[1][..(str_vec[1].len() - 1)].parse::<u32>().unwrap();
            for (index, &item) in str_vec.iter().enumerate() {
                if (item.starts_with("red") && str_vec[index - 1].parse::<u32>().unwrap() > 12)
                    || (item.starts_with("green")
                        && str_vec[index - 1].parse::<u32>().unwrap() > 13)
                    || (item.starts_with("blue") && str_vec[index - 1].parse::<u32>().unwrap() > 14)
                {
                    continue 'main;
                }
            }
            sum += id;
        }
    }
    sum
}
