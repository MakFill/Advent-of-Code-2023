use std::collections::{HashSet, VecDeque};

fn main() {
    println!("{}", get_result());
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Beam {
    position: (usize, usize),
    direction: Direction,
    layout_size: (usize, usize),
}

impl Beam {
    fn new(position: (usize, usize), direction: Direction, layout_size: (usize, usize)) -> Self {
        Self {
            position,
            direction,
            layout_size,
        }
    }

    fn next(&self, input: &[Vec<char>]) -> Option<Vec<Beam>> {
        match self.direction {
            Direction::Top => {
                if self.position.0 == 0 {
                    return None;
                }
                let position = (self.position.0 - 1, self.position.1);
                match input[position.0][position.1] {
                    '-' => Some(vec![
                        Beam::new(position, Direction::Left, self.layout_size),
                        Beam::new(position, Direction::Right, self.layout_size),
                    ]),
                    '\\' => Some(vec![Beam::new(position, Direction::Left, self.layout_size)]),
                    '/' => Some(vec![Beam::new(
                        position,
                        Direction::Right,
                        self.layout_size,
                    )]),
                    _ => Some(vec![Beam::new(position, self.direction, self.layout_size)]),
                }
            }
            Direction::Right => {
                if self.position.1 == self.layout_size.1 - 1 {
                    return None;
                }
                let position = (self.position.0, self.position.1 + 1);
                match input[position.0][position.1] {
                    '|' => Some(vec![
                        Beam::new(position, Direction::Top, self.layout_size),
                        Beam::new(position, Direction::Bottom, self.layout_size),
                    ]),
                    '\\' => Some(vec![Beam::new(
                        position,
                        Direction::Bottom,
                        self.layout_size,
                    )]),
                    '/' => Some(vec![Beam::new(position, Direction::Top, self.layout_size)]),
                    _ => Some(vec![Beam::new(position, self.direction, self.layout_size)]),
                }
            }
            Direction::Bottom => {
                if self.position.0 == self.layout_size.0 - 1 {
                    return None;
                }
                let position = (self.position.0 + 1, self.position.1);
                match input[position.0][position.1] {
                    '-' => Some(vec![
                        Beam::new(position, Direction::Left, self.layout_size),
                        Beam::new(position, Direction::Right, self.layout_size),
                    ]),
                    '\\' => Some(vec![Beam::new(
                        position,
                        Direction::Right,
                        self.layout_size,
                    )]),
                    '/' => Some(vec![Beam::new(position, Direction::Left, self.layout_size)]),
                    _ => Some(vec![Beam::new(position, self.direction, self.layout_size)]),
                }
            }
            Direction::Left => {
                if self.position.1 == 0 {
                    return None;
                }
                let position = (self.position.0, self.position.1 - 1);
                match input[position.0][position.1] {
                    '|' => Some(vec![
                        Beam::new(position, Direction::Top, self.layout_size),
                        Beam::new(position, Direction::Bottom, self.layout_size),
                    ]),
                    '\\' => Some(vec![Beam::new(position, Direction::Top, self.layout_size)]),
                    '/' => Some(vec![Beam::new(
                        position,
                        Direction::Bottom,
                        self.layout_size,
                    )]),
                    _ => Some(vec![Beam::new(position, self.direction, self.layout_size)]),
                }
            }
        }
    }
}

fn get_result() -> usize {
    let input = include_str!("../input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let layout_size = (input.len(), input[0].len());
    let mut res = 0;
    for index in 0..layout_size.0 {
        // From the left
        res = res.max(get_res_by_start_position(
            Beam::new((index, 0), Direction::Right, layout_size),
            &input,
        ));
        // From the right
        res = res.max(get_res_by_start_position(
            Beam::new((index, layout_size.1 - 1), Direction::Left, layout_size),
            &input,
        ));
    }

    for index in 0..layout_size.1 {
        // From the top
        res = res.max(get_res_by_start_position(
            Beam::new((0, index), Direction::Bottom, layout_size),
            &input,
        ));
        // From the bottom
        res = res.max(get_res_by_start_position(
            Beam::new((layout_size.0 - 1, index), Direction::Top, layout_size),
            &input,
        ));
    }

    res
}

fn get_res_by_start_position(enter_beam: Beam, input: &[Vec<char>]) -> usize {
    let mut visited_set = HashSet::new();
    let mut res_set = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(enter_beam);
    visited_set.insert(enter_beam);
    res_set.insert(enter_beam.position);
    while !queue.is_empty() {
        let beam = queue.pop_front().unwrap();
        if let Some(beam_vec) = beam.next(input) {
            for next_beam in beam_vec {
                if visited_set.insert(next_beam) {
                    queue.push_back(next_beam);
                    res_set.insert(next_beam.position);
                }
            }
        }
    }

    res_set.len()
}
