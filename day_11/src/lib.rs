#![allow(unused)]

mod monkey;

#[cfg(test)]
mod tests {
    use super::*;
    use constants::*;
    use pretty_assertions::assert_eq;

    mod constants {
        pub const GENERIC_MONKEY: &str = "Monkey 0:
  Starting items: 1, 2, 3, 4, 5
  Operation: new = old * 2
  Test: divisible by 2
    If true: throw to monkey 1
    If false: throw to monkey 2";
        pub const MONKEY_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
    }
}
