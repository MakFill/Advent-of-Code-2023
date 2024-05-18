use std::{env, fs::File, io::Read};

fn main() {
    println!("{}", get_lowest_location());
}

enum Map {
    ToSoil,
    ToFertilizer,
    ToWater,
    ToLight,
    ToTemperature,
    ToHumidity,
    ToLocation,
    Null,
}

fn get_lowest_location() -> u64 {
    let dir = env::current_dir().unwrap();
    let mut res = File::open(dir.join("input.txt")).ok().unwrap();
    let mut text = String::new();
    let _ = res.read_to_string(&mut text);
    let mut seeds: Vec<[u64; 2]> = vec![];
    let mut to_soil_vec: Vec<Vec<u64>> = vec![];
    let mut to_fertilizer_vec: Vec<Vec<u64>> = vec![];
    let mut to_water_vec: Vec<Vec<u64>> = vec![];
    let mut to_light_vec: Vec<Vec<u64>> = vec![];
    let mut to_temperature_vec: Vec<Vec<u64>> = vec![];
    let mut to_humidity_vec: Vec<Vec<u64>> = vec![];
    let mut to_location_vec: Vec<Vec<u64>> = vec![];
    let mut map_flag: Map = Map::Null;
    for line in text.lines() {
        if line.is_empty() {
            map_flag = Map::Null;
            continue;
        }
        if line.starts_with("seeds") {
            line.split_once(':')
                .unwrap()
                .1
                .trim()
                .split_ascii_whitespace()
                .map(|i| i.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
                .chunks_exact(2)
                .for_each(|chunk| seeds.push([chunk[0], chunk[0] + chunk[1] - 1]));
        } else if line.starts_with("seed-to-soil") {
            map_flag = Map::ToSoil;
        } else if line.starts_with("soil-to-fertilizer") {
            map_flag = Map::ToFertilizer;
        } else if line.starts_with("fertilizer-to-water") {
            map_flag = Map::ToWater;
        } else if line.starts_with("water-to-light") {
            map_flag = Map::ToLight;
        } else if line.starts_with("light-to-temperature") {
            map_flag = Map::ToTemperature;
        } else if line.starts_with("temperature-to-humidity") {
            map_flag = Map::ToHumidity;
        } else if line.starts_with("humidity-to-location") {
            map_flag = Map::ToLocation;
        } else if is_starts_with_digit(line) {
            let val = line
                .split_ascii_whitespace()
                .map(|i| i.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            match map_flag {
                Map::ToSoil => to_soil_vec.push(val),
                Map::ToFertilizer => to_fertilizer_vec.push(val),
                Map::ToWater => to_water_vec.push(val),
                Map::ToLight => to_light_vec.push(val),
                Map::ToTemperature => to_temperature_vec.push(val),
                Map::ToHumidity => to_humidity_vec.push(val),
                Map::ToLocation => to_location_vec.push(val),
                Map::Null => (),
            }
        }
    }

    let soil = get_map_ranges(seeds, &to_soil_vec);
    let fertilizer = get_map_ranges(soil, &to_fertilizer_vec);
    let water = get_map_ranges(fertilizer, &to_water_vec);
    let light = get_map_ranges(water, &to_light_vec);
    let temperature = get_map_ranges(light, &to_temperature_vec);
    let humidity = get_map_ranges(temperature, &to_humidity_vec);
    let location = get_map_ranges(humidity, &to_location_vec);

    location.iter().min_by(|a, b| a[0].cmp(&b[0])).unwrap()[0]
}

fn get_map_ranges(prev_range: Vec<[u64; 2]>, map_vec: &[Vec<u64>]) -> Vec<[u64; 2]> {
    let map_range = map_vec
        .iter()
        .map(|range_vec| {
            let start = range_vec[1] as i64;
            let end = (range_vec[1] + range_vec[2] - 1) as i64;
            let diff = (range_vec[0] as i64) - (range_vec[1] as i64);
            [start, end, diff]
        })
        .collect::<Vec<_>>();
    let mut prev_range = prev_range;
    let mut res_range: Vec<[u64; 2]> = vec![];
    for map_r in &map_range {
        let mut temp_prev_rng = vec![];
        for prev_r in prev_range {
            let (intersection_rng, ranges_vec) = get_splited_ranges(&prev_r, map_r);
            if let Some(intersection) = intersection_rng {
                res_range.push(intersection);
            }
            for v in ranges_vec {
                temp_prev_rng.push(v);
            }
        }
        prev_range = temp_prev_rng;
    }
    res_range.append(&mut prev_range);
    res_range
}

fn get_splited_ranges(
    target_range: &[u64; 2],
    map_range: &[i64; 3],
) -> (Option<[u64; 2]>, Vec<[u64; 2]>) {
    let mut res = vec![];
    let start = target_range[0].max(map_range[0] as u64);
    let end = target_range[1].min(map_range[1] as u64);
    if start <= end {
        let intersection_rng = [
            ((start as i64) + map_range[2]) as u64,
            ((end as i64) + map_range[2]) as u64,
        ];
        if target_range[0] < start {
            res.push([target_range[0], start - 1]);
        }
        if target_range[1] > end {
            res.push([end + 1, target_range[1]]);
        }
        (Some(intersection_rng), res)
    } else {
        res.push(*target_range);
        (None, res)
    }
}

fn is_starts_with_digit(s: &str) -> bool {
    if let Some(c) = s.chars().next() {
        return c.is_ascii_digit();
    }
    false
}
