use supply_stacks::CrateStack;

fn main() {
    let input_lines = include_str!("../input_test.txt").lines();

    let stacks_str = input_lines
        // Hope this is just an iterator clone
        .clone()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();
    dbg!(&stacks_str);

    //Determine number of stacks and create a vec of stacks
    let stacks: Vec<CrateStack> = Vec::with_capacity(
        stacks_str
            .last()
            .expect("Expected to have the enumeration of stacks")
            .len()
            / 3,
    );
    let len = stacks_str.len() + 1;

    for stacks_line in stacks_str.into_iter().rev().skip(1) {
        dbg!(stacks_line);
        let r: Vec<_> = stacks_line
            .split_ascii_whitespace()
            .enumerate()
            .map(|(idx, crt)| match crt.len() {
                3 => Some(crt),
                _ => None,
            })
            .collect();
        dbg!(r);
    }

    dbg!(stacks);

    let commands = input_lines.skip(len).collect::<Vec<_>>();
    dbg!(commands);
}
