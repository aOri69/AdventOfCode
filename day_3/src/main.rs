use rucksack_reorganization::*;

fn main() {
    let _content = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    let content = include_str!("../input.txt");

    let racksacks: Vec<Rucksack> = content
        .lines()
        .map(|l| Rucksack::from_str(l).unwrap())
        .collect();

    let result: Vec<usize> = racksacks
        .iter()
        .map(|rack| {
            // dbg!(&rack.unique_items());
            rack.priority()
        })
        .collect::<Vec<_>>();
    dbg!(result.iter().sum::<usize>());

    if racksacks.len() % 3 != 0 {
        panic!("number of racksacks could not be divided into groups of three");
    }

    let groups = racksacks
        .chunks_exact(3)
        .map(|chunk| RacksackGroup::new([chunk[0].clone(), chunk[1].clone(), chunk[2].clone()]));

    // for gr in groups {
    //     let items = gr.priority();
    //     dbg!(items);
    // }

    let part_two_answer: usize = groups.map(|gr| gr.priority()).sum();
    dbg!(part_two_answer);
}
