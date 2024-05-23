use std::{env, fs::File, io::Read};

fn main() {
    println!("{}", get_lengths_sum());
}

fn get_lengths_sum() -> usize {
    let dir = env::current_dir().unwrap();
    let mut res = File::open(dir.join("input.txt")).ok().unwrap();
    let mut text = String::new();
    let _ = res.read_to_string(&mut text);

    let mut locations_vec = vec![];
    let mut image_height = 0;
    let mut image_width = 0;
    text.lines().enumerate().for_each(|(line_index, line)| {
        image_height += 1;
        if image_width == 0 {
            image_width = line.len();
        }
        line.chars().enumerate().for_each(|(char_index, c)| {
            if c == '#' {
                locations_vec.push((line_index, char_index));
            }
        });
    });

    let mut horizontal_offset_indexes = vec![];
    let mut vertical_offset_indexes = vec![];
    (0..image_height.max(image_width)).for_each(|index| {
        if (index < image_height) && !locations_vec.iter().any(|i| i.0 == index) {
            horizontal_offset_indexes.push(index);
        }
        if (index < image_width) && !locations_vec.iter().any(|i| i.1 == index) {
            vertical_offset_indexes.push(index);
        }
    });

    let gaped_locations_vec = locations_vec
        .iter()
        .map(|location| {
            let mut horizontal_location = location.0;
            for &horizontal_offset_index in &horizontal_offset_indexes {
                if location.0 > horizontal_offset_index {
                    horizontal_location += 1;
                } else {
                    break;
                }
            }

            let mut vertical_location = location.1;
            for &vertical_offset_index in &vertical_offset_indexes {
                if location.1 > vertical_offset_index {
                    vertical_location += 1;
                } else {
                    break;
                }
            }

            (horizontal_location, vertical_location)
        })
        .collect::<Vec<_>>();

    gaped_locations_vec
        .iter()
        .enumerate()
        .map(|(index, location)| {
            let rest_locations = &gaped_locations_vec[(index + 1)..];
            rest_locations
                .iter()
                .map(|loc| loc.0.abs_diff(location.0) + loc.1.abs_diff(location.1))
                .sum::<usize>()
        })
        .sum::<usize>()
}
