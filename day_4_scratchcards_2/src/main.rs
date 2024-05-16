use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::Read,
};

fn main() {
    println!("{}", get_total_scratchcards());
}

fn get_total_scratchcards() -> usize {
    let dir = env::current_dir().unwrap();
    let mut res = File::open(dir.join("input.txt")).ok().unwrap();
    let mut text = String::new();
    let mut sum = 0;
    let _ = res.read_to_string(&mut text);
    let lines_vec_iter = text
        .lines()
        .map(|line| line.split(':').collect::<Vec<_>>()[1]);
    let mut frequency_map = HashMap::<usize, usize>::new();
    for (index, line) in lines_vec_iter.enumerate() {
        let (owned_nums, winning_nums) = line.split_once('|').unwrap();
        let owned_set = owned_nums
            .split(' ')
            .filter(|el| !el.is_empty())
            .collect::<HashSet<_>>();
        let winning_set = winning_nums
            .split(' ')
            .filter(|el| !el.is_empty())
            .collect::<HashSet<_>>();
        let count = owned_set.intersection(&winning_set).count();
        let frequency = *frequency_map.get(&index).unwrap_or(&0);
        let card_res = frequency + 1;
        ((index + 1)..=(count + index)).for_each(|val| {
            frequency_map
                .entry(val)
                .and_modify(|val| *val += card_res)
                .or_insert(card_res);
        });
        sum += card_res;
    }

    sum
}
