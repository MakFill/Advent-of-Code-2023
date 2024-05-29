fn main() {
    println!("{}", get_results_sum());
}

fn get_results_sum() -> usize {
    let input = include_str!("../input.txt");

    input.split(',').fold(0, |acc, curr| acc + get_hash(curr))
}

fn get_hash(input: &str) -> usize {
    input
        .trim()
        .chars()
        .fold(0, |acc, curr| (acc + (curr as usize)) * 17 % 256)
}
