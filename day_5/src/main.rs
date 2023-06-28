use std::str::FromStr;

use supply_stacks::{
    move_crate, move_crate_and_retain_order, parse_into_crates, CommandError, MoveCommand, Stack,
};

fn main() {
    let input_lines = include_str!("../input.txt").lines();

    let stacks_str = input_lines
        // Hope this is just an iterator clone
        .clone()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();
    dbg!(&stacks_str);

    //Determine number of stacks and create a vec of stacks
    let size = (stacks_str
        .last()
        .expect("Expected to have the enumeration of stacks")
        .len()
        + 1) // Strange way to make the proper number of stacks....
        / 4;
    // And create a vec of stacks with determined size
    let mut stacks = vec![Stack::new(); size];

    // To use with the second copy of iterator
    // to start with the commands line
    let len = stacks_str.len() + 1;

    for stacks_line in stacks_str.into_iter().rev().skip(1) {
        // println!("stacks_line: {stacks_line:?}");
        let crates = parse_into_crates(stacks_line);
        // println!("parsed: {crates:?}");
        crates
            .into_iter()
            .enumerate()
            // idx needed to chose the proper stack to push
            .filter(|(_idx, crt)| crt.is_some())
            .for_each(|(idx, crt)| {
                stacks.get_mut(idx).unwrap().push(crt.unwrap());
            });
    }

    println!("Before move commands:");
    println!("{stacks:?}");

    // Using the original iterator shifted by Crates lines + one empty \n line
    let input_lines = input_lines.skip(len);
    let commands = input_lines
        .map(MoveCommand::from_str)
        .collect::<Result<Vec<_>, CommandError>>()
        .expect("expected to parse all commands");
    // println!("{commands:?}");

    println!("-------------------PART 1-------------------");
    part_1(stacks.clone(), commands.clone());
    println!("-------------------PART 2-------------------");
    part_2(stacks, commands);
    println!("-------------------END----------------------");
}

fn part_1(mut stacks: Vec<Stack>, commands: Vec<MoveCommand>) {
    commands.into_iter().for_each(|cmd| {
        move_crate(&mut stacks, cmd);
    });
    println!("After move commands:");
    println!("{stacks:?}");

    println!("Part 1 answer:");
    let result_part1: String = stacks
        .into_iter()
        .filter_map(|stack| stack.top())
        .fold("".to_string(), |acc, crt| format!("{}{}", acc, *crt));
    println!("{result_part1}");
}

fn part_2(mut stacks: Vec<Stack>, commands: Vec<MoveCommand>) {
    commands.into_iter().for_each(|cmd| {
        move_crate_and_retain_order(&mut stacks, cmd);
    });
    println!("After move commands:");
    println!("{stacks:?}");

    println!("Part 1 answer:");
    let result_part1: String = stacks
        .into_iter()
        .filter_map(|stack| stack.top())
        .fold("".to_string(), |acc, crt| format!("{}{}", acc, *crt));
    println!("{result_part1}");
}
