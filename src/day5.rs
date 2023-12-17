use crate::utils;
use std::io::BufRead;

#[derive(Debug)]
struct SeedMapRow {
    source_start: i128,
    source_end: i128,
    delta: i128,
}

fn get_seeds() -> Vec<i128> {
    let mut reader = utils::get_reader_for_day(5);

    let mut seed_line = String::new();
    reader
        .read_line(&mut seed_line)
        .expect("First line isn't seeds?!");

    seed_line
        .split(":")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<i128>().unwrap())
        .collect()
}

fn get_maps() -> Vec<Vec<SeedMapRow>> {
    let reader = utils::get_reader_for_day(5);

    let mut all_maps: Vec<Vec<SeedMapRow>> = Vec::new();
    let mut line;
    let mut curr_map: Vec<SeedMapRow> = Vec::new();

    for raw_line in reader.lines() {
        line = raw_line.unwrap();

        if line.is_empty() {
            if !curr_map.is_empty() {
                curr_map.sort_by(|a, b| a.source_start.cmp(&b.source_start));
                all_maps.push(curr_map);
            }
            curr_map = Vec::new();
            continue;
        }

        if line.contains("map") || line.contains("seeds") {
            // It's a header line, for now we don't care
            continue;
        }

        let line_parts: Vec<i128> = line
            .split_whitespace()
            .map(|n| n.parse::<i128>().unwrap())
            .collect();

        let dest = *line_parts.get(0).unwrap();
        let src = *line_parts.get(1).unwrap();
        let offset = *line_parts.get(2).unwrap();

        curr_map.push(SeedMapRow {
            source_start: src,
            source_end: src + offset,
            delta: dest - src,
        })
    }

    // Add the last one
    curr_map.sort_by(|a, b| a.source_start.cmp(&b.source_start));
    all_maps.push(curr_map);

    all_maps
}

fn day5_part1() {
    let mut seeds = get_seeds();

    let maps = get_maps();
    for map in maps.iter() {
        for seed in &mut seeds {
            // First check to see if the source is min/max out-of-bounds and thus doesn't change
            if *seed < map.first().unwrap().source_start || *seed > map.last().unwrap().source_end {
                continue;
            }

            for row in map {
                if *seed >= row.source_start && *seed < row.source_end {
                    // We have something to change!
                    *seed += row.delta;
                    break;
                }
            }
        }
    }
    println!("Lowest location = {:?}", seeds.iter().min().unwrap());
}

pub fn day5() {
    day5_part1();
    // day5_part2();
}
