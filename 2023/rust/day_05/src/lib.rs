use std::{
    ops::RangeInclusive,
    sync::{Arc, Mutex},
};

use crate::parser::{parse_input, ParseResult};

type Seed = u64;

mod parser;

#[derive(Clone)]
struct Map {
    source: String,
    destination: String,
    ranges: Vec<SeedRange>,
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}-to-{} map:", self.source, self.destination)?;
        self.ranges.iter().try_for_each(|r| writeln!(f, "{r:?}"))?;
        Ok(())
    }
}

impl Map {
    fn get_dest(&self, source_num: Seed) -> Seed {
        let res = self.ranges.iter().as_slice().binary_search_by(|r| {
            match r.source_range.contains(&source_num) {
                true => std::cmp::Ordering::Equal,
                false => r.source_range.start().cmp(&source_num),
            }
        });
        if let Ok(idx) = res {
            return self.ranges[idx]
                .get_dest(source_num)
                .expect("binary search should already find the correct position");
        }
        source_num
    }
}

#[derive(PartialEq, Eq, Clone)]
struct SeedRange {
    source_range: RangeInclusive<Seed>,
    dest_range: RangeInclusive<Seed>,
}

impl SeedRange {
    fn get_dest(&self, source_num: Seed) -> Option<Seed> {
        match self.source_range.contains(&source_num) {
            true => Some(self.dest_range.start() + (source_num - self.source_range.start())),
            false => None,
        }
    }
}

impl PartialOrd for SeedRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SeedRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.source_range.start().cmp(other.source_range.start())
    }
}

impl core::hash::Hash for SeedRange {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.source_range.start().hash(state);
    }
}

impl std::fmt::Debug for SeedRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "src: {:>11}..={:<11}, dst: {:>11}..={:<11}",
            self.source_range.start(),
            self.source_range.end(),
            self.dest_range.start(),
            self.dest_range.end()
        )?;
        Ok(())
    }
}

pub fn part_1(input: &str) -> Seed {
    let ParseResult { seeds, mut maps } = parse_input(input).unwrap();
    // dbg!(&maps);
    let mut numbers_vec = vec![];
    for seed in seeds {
        let mut current_number = seed;
        for map in &mut maps {
            current_number = map.get_dest(current_number);
        }
        numbers_vec.push(current_number);
    }
    numbers_vec.iter().copied().min().unwrap_or_default()
}

pub fn part_2_threaded(input: &str) -> Seed {
    let ParseResult { seeds, maps } = parse_input(input).unwrap();
    let seeds = seeds_vec_to_ranges(seeds);

    let mut handles = vec![];
    let min_location = Arc::new(Mutex::new(u64::MAX));

    for seed_range in seeds.into_iter() {
        let min_mut = Arc::clone(&min_location);
        let mut maps_copy = maps.clone();
        let handle = std::thread::spawn(move || {
            for seed in seed_range {
                let mut current_number = seed;
                for map in &mut maps_copy {
                    current_number = map.get_dest(current_number);
                }
                let mut min = min_mut.lock().unwrap();
                if current_number < *min {
                    *min = current_number;
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let min = *min_location.lock().unwrap();
    min
}

pub fn part_2_threaded_mpsc(input: &str) -> Seed {
    let ParseResult { seeds, maps } = parse_input(input).unwrap();
    let seeds = seeds_vec_to_ranges(seeds);

    let mut handles = vec![];
    let (tx, rx) = std::sync::mpsc::channel();
    let mut min_location = u64::MAX;

    for seed_range in seeds.into_iter() {
        let mut maps_copy = maps.clone();
        let tx_copy = tx.clone();
        let handle = std::thread::spawn(move || {
            for seed in seed_range {
                let mut current_number = seed;
                for map in &mut maps_copy {
                    current_number = map.get_dest(current_number);
                }
                tx_copy.send(current_number).unwrap();
            }
        });
        handles.push(handle);
    }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    drop(tx);
    while let Ok(r) = rx.recv() {
        if r < min_location {
            min_location = r;
        }
    }

    min_location
}

pub fn part_2_single(input: &str) -> Seed {
    let ParseResult { seeds, maps } = parse_input(input).unwrap();
    let seeds = seeds_vec_to_ranges(seeds);

    let mut min_location = u64::MAX;

    for seed_range in seeds.into_iter() {
        for seed in seed_range {
            let mut current_number = seed;
            for map in &maps {
                current_number = map.get_dest(current_number);
            }
            if current_number < min_location {
                min_location = current_number;
            }
        }
    }
    min_location
}

fn seeds_vec_to_ranges(seeds: Vec<Seed>) -> Vec<RangeInclusive<Seed>> {
    seeds
        .chunks_exact(2)
        .map(|r| r[0]..=r[0] + r[1] - 1)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 35);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2_threaded(INPUT), 46);
        assert_eq!(part_2_single(INPUT), 46);
        assert_eq!(part_2_threaded_mpsc(INPUT), 46);
    }
}
