use std::ops::RangeInclusive;

use color_eyre::{eyre, Report, Result};

trait RangeInclusiveExt {
    fn contains_range(&self, other: &Self) -> bool;
    fn overlaps_range(&self, other: &Self) -> bool;
    fn contains_or_is_contained(&self, other: &Self) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }
    fn overlaps_or_is_overlaped(&self, other: &Self) -> bool {
        self.overlaps_range(other) || other.overlaps_range(self)
    }
}

impl<T> RangeInclusiveExt for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }
    fn overlaps_range(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

fn main() -> Result<(), Report> {
    let mut contains: u32 = 0;
    let mut overlaps: u32 = 0;

    for line in include_str!("../input.txt").lines() {
        let ranges = line
            .split(',')
            .map(|range| {
                let mut range_it = range.split('-');
                let start = range_it
                    .next()
                    .ok_or_else(|| eyre::eyre!("Expected start of range"))?
                    .parse::<u32>()?;
                let end = range_it
                    .next()
                    .ok_or_else(|| eyre::eyre!("Expected end of range"))?
                    .parse::<u32>()?;
                Ok(start..=end)
            })
            .collect::<Result<Vec<_>>>()?;
        let (first, second) = collect_to_tuple(ranges)?;

        if first.contains_or_is_contained(&second) {
            contains += 1;
        }
        if first.overlaps_or_is_overlaped(&second) {
            // println!("overlaping pair: {first:?} - {second:?}");
            overlaps += 1;
        }
    }

    dbg!(contains);
    dbg!(overlaps);
    Ok(())
}

fn collect_to_tuple<T>(ranges_vec: Vec<T>) -> Result<(T, T)> {
    if ranges_vec.len() != 2 {
        Err(eyre::eyre!("Expected exactly two ranges"))?;
    }
    let mut ranges_it = ranges_vec.into_iter();
    let first = ranges_it
        .next()
        .ok_or_else(|| eyre::eyre!("Start range not found"))?;
    let second = ranges_it
        .next()
        .ok_or_else(|| eyre::eyre!("End range not found"))?;
    Ok((first, second))
}
