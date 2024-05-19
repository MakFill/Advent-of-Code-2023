use std::{collections::HashMap, env, fs::File, io::Read};

fn main() {
    println!("{}", get_total_winnings());
}

fn get_total_winnings() -> u32 {
    let dir = env::current_dir().unwrap();
    let mut res = File::open(dir.join("input.txt")).ok().unwrap();
    let mut text = String::new();
    let _ = res.read_to_string(&mut text);

    let mut hands_vec = text
        .split('\n')
        .map(|i| i.split_ascii_whitespace().collect::<Vec<_>>())
        .map(|i| (i[0], i[1].parse::<u32>().unwrap()))
        .collect::<Vec<_>>();

    hands_vec.sort_unstable_by(|(hand1, _), (hand2, _)| {
        let priority1 = get_hand_priority(hand1);
        let priority2 = get_hand_priority(hand2);

        if priority1 == priority2 {
            for (a, b) in hand1.chars().zip(hand2.chars()) {
                if a != b {
                    return get_char_priority(a).cmp(&get_char_priority(b));
                }
            }
        }

        priority1.cmp(&priority2)
    });

    hands_vec
        .iter()
        .enumerate()
        .fold(0, |acc, (index, (_, val))| acc + (index as u32 + 1) * val)
}

fn get_hand_priority(input: &str) -> u32 {
    let mut map = HashMap::<char, u32>::new();

    for c in input.chars() {
        map.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut freq_vec = vec![];
    for &frequency in map.values() {
        freq_vec.push(frequency);
    }

    freq_vec.sort_unstable_by(|a, b| b.cmp(a));

    let freq_str = freq_vec
        .iter_mut()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .concat();

    match freq_str.as_str() {
        "5" => 7,
        "41" => 6,
        "32" => 5,
        "311" => 4,
        "221" => 3,
        "2111" => 2,
        _ => 1,
    }
}

fn get_char_priority(c: char) -> u32 {
    match c {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        _ => 1,
    }
}
