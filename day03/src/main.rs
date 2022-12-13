use std::collections::hash_map::RandomState;
use std::collections::HashSet;

struct Rucksack(Vec<char>);

impl Rucksack {
    fn from_str(input: &str) -> Self {
        Self(input.chars().collect())
    }

    fn odd_one(&self) -> char {
        let (left, right) = self.0.split_at(self.0.len() / 2);

        let set1: HashSet<&char, RandomState> = HashSet::from_iter(left.iter());
        let set2: HashSet<&char, RandomState> = HashSet::from_iter(right.iter());
        let intersection = set1.intersection(&set2).collect::<Vec<&&char>>();
        let [odd_one] = intersection[..] else { panic!("there should only be one intersecion") };
        **odd_one
    }

    fn common_one(a: &Self, b: &Self, c: &Self) -> char {
        let set1: HashSet<char, RandomState> = HashSet::from_iter(a.0.clone().into_iter());
        let set2 = HashSet::from_iter(b.0.clone().into_iter());
        let set3 = HashSet::from_iter(c.0.iter());

        let intermediary_set = set1.intersection(&set2).collect::<HashSet<&char>>();
        let intersection = set3.intersection(&intermediary_set).collect::<Vec<_>>();

        let [same_one] = intersection[..] else { panic!("there should only be one intersecion") };
        **same_one
    }
}

fn char_to_number(input: char) -> u32 {
    match input {
        i @ 'a'..='z' => i as u32 - 96,
        i @ 'A'..='Z' => i as u32 - 38,
        _ => 0,
    }
}

fn main() {
    use itertools::Itertools;
    let input = include_str!("input.txt");
    let sum: u32 = input
        .lines()
        .map(|line| char_to_number(Rucksack::from_str(line).odd_one()))
        .sum();

    println!("The sum of wrong entries is {}", sum);
    let sum: u32 = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let mut chunk = chunk.into_iter();
            let a = Rucksack::from_str(chunk.next().unwrap());
            let b = Rucksack::from_str(chunk.next().unwrap());
            let c = Rucksack::from_str(chunk.next().unwrap());
            char_to_number(Rucksack::common_one(&a, &b, &c))
        })
        .sum();

    println!("The sum of common badged is {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_odd_ones() {
        assert_eq!(
            Rucksack::from_str("vJrwpWtwJgWrhcsFMMfFFhFp").odd_one(),
            'p'
        );
        assert_eq!(
            Rucksack::from_str("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").odd_one(),
            'L'
        );
        assert_eq!(Rucksack::from_str("PmmdzqPrVvPwwTWBwg").odd_one(), 'P');
    }

    #[test]
    fn should_convert_to_number() {
        assert_eq!(char_to_number('p'), 16);
        assert_eq!(char_to_number('L'), 38);
        assert_eq!(char_to_number('P'), 42);
    }
}
