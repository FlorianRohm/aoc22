use itertools::Itertools;
use std::ops::RangeInclusive;

trait InclusiveRangeExt {
    fn contains_range(&self, other: &Self) -> bool;
    fn overlaps_range(&self, other: &Self) -> bool;
    fn from_str(input: &str) -> Self;
}

impl InclusiveRangeExt for RangeInclusive<usize> {
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps_range(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }

    fn from_str(input: &str) -> Self {
        let (start, end) = input
            .split('-')
            .map(|a| a.parse().expect("should be able to parse"))
            .collect_tuple::<(usize, usize)>()
            .unwrap();

        start..=end
    }
}

fn main() {
    let input = include_str!("input.txt");
    let x = input
        .lines()
        .map(|line| {
            let (range1, range2) = line
                .split(',')
                .collect_tuple::<(&str, &str)>()
                .expect("exactly two");
            (
                RangeInclusive::from_str(range1),
                RangeInclusive::from_str(range2),
            )
        })
        .collect::<Vec<_>>();
    let number_of_inclusive_ranges = x
        .clone()
        .iter()
        .filter(|(range1, range2)| range1.contains_range(range2) || range2.contains_range(range1))
        .count();

    println!(
        "Number of ranges which are included in each other is {}",
        number_of_inclusive_ranges
    );
    let number_of_overlapping_ranges = x
        .clone()
        .iter()
        .filter(|(range1, range2)| range1.overlaps_range(range2) || range2.overlaps_range(range1))
        .count();

    println!(
        "Number of ranges which are overlapping each other is {}",
        number_of_overlapping_ranges
    );
}
