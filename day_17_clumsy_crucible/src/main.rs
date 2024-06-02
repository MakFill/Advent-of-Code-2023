use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

fn main() {
    println!("{}", get_result());
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    cost: usize,
    row: usize,
    col: usize,
    direction_with_moves: ((isize, isize), usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(
    grid: &Vec<Vec<usize>>,
    start_row: usize,
    start_col: usize,
) -> HashMap<(usize, usize), Vec<State>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut map: HashMap<(usize, usize), Vec<State>> = HashMap::new();
    let mut heap = BinaryHeap::new();

    heap.push(State {
        cost: 0,
        row: start_row,
        col: start_col,
        direction_with_moves: ((0, 0), 0),
    });

    while let Some(State {
        cost,
        row,
        col,
        direction_with_moves,
    }) = heap.pop()
    {
        let mut moves = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        moves.retain(|i| i != &(-direction_with_moves.0 .0, -direction_with_moves.0 .1));
        if direction_with_moves.1 == 3 {
            moves.retain(|i| i != &direction_with_moves.0);
        }
        for &single_move in &moves {
            let (dr, dc) = single_move;
            let next_row = row as isize + dr;
            let next_col = col as isize + dc;

            if next_row >= 0
                && next_row < rows as isize
                && next_col >= 0
                && next_col < cols as isize
            {
                let next_row = next_row as usize;
                let next_col = next_col as usize;
                let next_cost = cost + grid[next_row][next_col];
                let moves_amount = if direction_with_moves.0 == single_move {
                    direction_with_moves.1 + 1
                } else {
                    1
                };
                let next_state = State {
                    cost: next_cost,
                    row: next_row,
                    col: next_col,
                    direction_with_moves: (single_move, moves_amount),
                };

                if let Some(cost_vec) = map.get_mut(&(next_row, next_col)) {
                    if cost_vec.iter().any(|i| {
                        i.col == next_state.col
                            && i.row == next_state.row
                            && i.direction_with_moves == next_state.direction_with_moves
                    }) {
                        continue;
                    }
                    cost_vec.push(next_state);
                } else {
                    map.insert((next_row, next_col), vec![next_state]);
                }
                heap.push(next_state);
            }
        }
    }

    map
}

fn get_result() -> usize {
    let input = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|i| i.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    dijkstra(&input, 0, 0)
        .get(&(input.len() - 1, input[0].len() - 1))
        .unwrap()
        .iter()
        .max() // We rewrited compare functionality above so max == min and vice versa
        .unwrap()
        .cost
}
