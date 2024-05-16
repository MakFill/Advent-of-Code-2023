use std::{collections::HashSet, env, fs::File, io::Read};

fn main() {
    println!("{}", get_points_total());
}

fn get_points_total() -> u32 {
    let dir = env::current_dir().unwrap();
    let mut res = File::open(dir.join("input.txt")).ok().unwrap();
    let mut text = String::new();
    let mut sum = 0;
    let _ = res.read_to_string(&mut text);
    text.lines()
        .map(|line| line.split(':').collect::<Vec<_>>()[1])
        .for_each(|line| {
            let (owned_nums, winning_nums) = line.split_once('|').unwrap();
            let owned_set = owned_nums
                .split(' ')
                .filter(|el| !el.is_empty())
                .collect::<HashSet<_>>();
            let winning_set = winning_nums
                .split(' ')
                .filter(|el| !el.is_empty())
                .collect::<HashSet<_>>();
            let count = owned_set.intersection(&winning_set).count() as u32;
            if count > 0 {
                sum += 2_u32.pow(count - 1);
            }
        });

    sum
}
