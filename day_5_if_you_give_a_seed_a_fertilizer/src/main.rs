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
    let mut lowest_location: u64 = u64::MAX;
    let mut seeds: Vec<u64> = vec![];
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
                .for_each(|i| seeds.push(i.parse::<u64>().unwrap()));
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
        } else if starts_with_digit(line) {
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

    for seed in seeds {
        let soil = get_map_val(seed, &to_soil_vec);
        let fertilizer = get_map_val(soil, &to_fertilizer_vec);
        let water = get_map_val(fertilizer, &to_water_vec);
        let light = get_map_val(water, &to_light_vec);
        let temperature = get_map_val(light, &to_temperature_vec);
        let humidity = get_map_val(temperature, &to_humidity_vec);
        let location = get_map_val(humidity, &to_location_vec);
        lowest_location = lowest_location.min(location);
    }

    lowest_location
}

fn get_map_val(prev_type_val: u64, map_vec: &Vec<Vec<u64>>) -> u64 {
    let mut res = prev_type_val;
    for range_vec in map_vec {
        if (range_vec[1]..(range_vec[1] + range_vec[2])).contains(&prev_type_val) {
            res = prev_type_val + range_vec[0] - range_vec[1];
            break;
        }
    }
    res
}

fn starts_with_digit(s: &str) -> bool {
    if let Some(c) = s.chars().next() {
        return c.is_ascii_digit();
    }
    false
}
