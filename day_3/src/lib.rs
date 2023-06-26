use std::collections::HashSet;
pub use std::str::FromStr;

pub trait DuplicateItems {
    fn duplicate_items(&self) -> HashSet<char>;
    fn priority(&self) -> usize {
        self.duplicate_items()
            .iter()
            .map(|c| Self::item_value(*c))
            .sum()
    }

    fn item_value(c: char) -> usize {
        match c {
            'a'..='z' => c as usize - 96, // 1..16
            'A'..='Z' => c as usize - 38, // 27..52
            _ => 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Rucksack {
    compartment: [Vec<char>; 2],
}
impl Rucksack {
    pub fn new(first_comp: &str, second_comp: &str) -> Self {
        let mut rucksack = Self {
            compartment: [first_comp.chars().collect(), second_comp.chars().collect()],
        };
        rucksack.compartment[0].sort();
        rucksack.compartment[1].sort();
        rucksack
    }
    pub fn unique_items(&self) -> HashSet<char> {
        let it = self.compartment.iter().flatten().cloned();
        HashSet::from_iter(it)
    }
}
impl DuplicateItems for Rucksack {
    fn duplicate_items(&self) -> HashSet<char> {
        self.compartment[0]
            .iter()
            .filter(|c| self.compartment[1].binary_search(c).is_ok())
            .cloned()
            .collect()
    }
}
impl FromStr for Rucksack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 2 != 0 {
            return Err("There must be equal amount of items in two compartments".to_string());
        }
        let (first_comp, second_comp) = s.split_at(s.len() / 2);
        Ok(Rucksack::new(first_comp, second_comp))
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RacksackGroup([Rucksack; 3]);

impl RacksackGroup {
    pub fn new(group: [Rucksack; 3]) -> Self {
        Self(group)
    }
}

impl DuplicateItems for RacksackGroup {
    fn duplicate_items(&self) -> HashSet<char> {
        let first = self.0[0].unique_items();
        let second = self.0[1].unique_items();
        let third = self.0[2].unique_items();

        first
            .into_iter()
            .filter(|c| second.contains(c) && third.contains(c))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_example() {
        let content = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let racksacks: Vec<Rucksack> = content
            .lines()
            .map(|l| Rucksack::from_str(l).unwrap())
            .collect();

        assert_eq!(racksacks.len(), 6);

        let mut it = racksacks.iter();
        assert_eq!(
            *it.next().unwrap().duplicate_items().iter().next().unwrap(),
            'p'
        );
        assert_eq!(
            *it.next().unwrap().duplicate_items().iter().next().unwrap(),
            'L'
        );
        assert_eq!(
            *it.next().unwrap().duplicate_items().iter().next().unwrap(),
            'P'
        );
    }
}
