use supply_stacks::{parse_into_crates, Stack};

fn main() {
    let input_lines = include_str!("../input_test.txt").lines();

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

    println!("{stacks:?}");

    // Using the original iterator shifted by Crates lines + one empty \n line
    let commands = input_lines.skip(len).collect::<Vec<_>>();
    dbg!(commands);
}
