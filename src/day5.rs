use crate::utils;
use std::{io::BufRead, ops::Range};

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

fn map_seeds(seeds: &mut Vec<i128>, maps: &Vec<Vec<SeedMapRow>>) {
    for map in maps.iter() {
        for seed in &mut *seeds {
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
}

fn get_ranges() -> Vec<Range<i128>> {
    let mut ranges: Vec<Range<i128>> = get_seeds().chunks(2).map(|c| c[0]..c[0] + c[1]).collect();

    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    ranges
}

fn day5_part1() {
    let mut seeds = get_seeds();
    let maps = get_maps();

    map_seeds(&mut seeds, &maps);

    println!("Lowest location = {:?}", seeds.iter().min().unwrap());
}

fn day5_part2() {
    let maps = get_maps();
    let mut ranges = get_ranges();
    let mut range: Range<i128>;

    for map in maps {
        let mut new_ranges: Vec<Range<i128>> = Vec::new();
        'next_range: while !ranges.is_empty() {
            range = ranges.pop().unwrap();
            for row in &map {
                let overlap_start = std::cmp::max(range.start, row.source_start);
                let overlap_end = std::cmp::min(range.end, row.source_end);

                if overlap_start < overlap_end {
                    // We have some overlap, so let's map what we need to and
                    // toss anything that's left over back into the ranges Vec
                    // to try processing again
                    new_ranges.push((overlap_start + row.delta)..(overlap_end + row.delta));

                    if range.start < overlap_start {
                        // Everything in the seed range before this mapping row gets pushed back in for further processing
                        ranges.push(range.start..overlap_start);
                    }

                    if range.end > overlap_end {
                        // Everything in the seed range after this mapping row gets pushed back in for further processing
                        ranges.push(overlap_end..range.end);
                    }
                    
                    continue 'next_range;
                }
            }
            // None of the mapping rows applied to this seed range, toss it in the new pile
            new_ranges.push(range.start..range.end);
            continue;
        }

        ranges = new_ranges;
    }

    ranges.sort_by(|a,b| a.start.cmp(&b.start));

    println!("Lowest location = {:?}", ranges.first().unwrap().start);
}

pub fn day5() {
    day5_part1();
    day5_part2();
}
