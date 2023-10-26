#![allow(unused_macros)]

use log::debug;
use monkey::{Monkey, WorryLevelUnsigned};
use nom::Finish;

use crate::monkey::{parse_monkeys, PrettyMonkeysEvalCount, PrettyMonkeysItems};

mod monkey;

pub struct Settings {
    rounds: u32,
    divide_by_3: bool,
}

macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        type_name_of(f)
            .rsplit("::")
            .find(|&part| part != "f" && part != "{{closure}}")
            .expect("Short function name")
    }};
}

pub fn get_monkey_business(monkeys: &[Monkey]) -> WorryLevelUnsigned {
    let mut monkeys = Vec::from(monkeys);
    monkeys.sort_by_key(|m| m.evaluations_count());
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.evaluations_count())
        .product()
}

pub fn play(monkeys: &mut [Monkey], settings: Settings) {
    for round in 1..=settings.rounds {
        debug!("-------------------------------Round {round}------------------------------");
        for monkey_idx in 0..monkeys.len() {
            let mut monkey = monkeys.get(monkey_idx).cloned().unwrap();

            debug!("Monkey {}:", monkey.id());
            while let Some(mut item) = monkey.items_mut().pop_front() {
                debug!("  Monkey inspects an item with a worry level of {}", item);

                let worry_level = monkey.operation().evaluate(item);
                *monkey.evaluations_count_mut() += 1;
                debug!(
                    "    Worry level is {} to {}",
                    monkey.operation(),
                    worry_level
                );
                let mut worry_level = worry_level;
                if settings.divide_by_3 {
                    worry_level /= 3;
                    debug!(
                        "    Monkey gets bored with item. Worry level is divided by 3 to {}.",
                        worry_level
                    );
                }

                item.set(worry_level as WorryLevelUnsigned);
                let throw_to = monkey.test().apply(worry_level);

                debug!("    Item with worry level {item} is thrown to monkey {throw_to}");

                monkeys
                    .get_mut(throw_to)
                    .unwrap()
                    .items_mut()
                    .push_back(item);
            }
            // place monkey copy(with modified items list) back to Vec
            *monkeys.get_mut(monkey_idx).unwrap() = monkey;
        }
        debug!("{:#?}", PrettyMonkeysItems(monkeys));
    }
    debug!("{:#?}", PrettyMonkeysItems(monkeys));
}

pub fn part1() -> WorryLevelUnsigned {
    let mut monkeys = parse_monkeys(include_str!("../input.txt"))
        .finish()
        .unwrap()
        .1;
    play(
        &mut monkeys,
        Settings {
            rounds: 20,
            divide_by_3: true,
        },
    );
    println!("{:#?}", PrettyMonkeysItems(&monkeys));
    println!("{:#?}", PrettyMonkeysEvalCount(&monkeys));
    get_monkey_business(&monkeys)
}

pub fn part2() -> WorryLevelUnsigned {
    let mut monkeys = parse_monkeys(include_str!("../input.txt"))
        .finish()
        .unwrap()
        .1;
    play(
        &mut monkeys,
        Settings {
            rounds: 10000,
            divide_by_3: false,
        },
    );
    println!("{:#?}", PrettyMonkeysItems(&monkeys));
    println!("{:#?}", PrettyMonkeysEvalCount(&monkeys));
    get_monkey_business(&monkeys)
}

#[cfg(test)]
mod test {
    use crate::monkey::{parse_monkeys, PrettyMonkeysEvalCount};

    use super::*;
    use env_logger::Env;
    use log::warn;
    use nom::Finish;

    fn init_log() {
        use std::io::Write;
        // WARN or above if RUST_LOG was not set
        let log_init_res = env_logger::Builder::from_env(Env::default().default_filter_or("warn"))
            .is_test(false) // pass logs to the test framework
            .format_timestamp(None)
            .format(|buf, record| {
                writeln!(
                    buf,
                    "{}",
                    // format_args!("{0}", function!()),
                    // record.level(),
                    record.args()
                )
            })
            .try_init();
        if let Err(e) = log_init_res {
            warn!("{}", e.to_string());
        }
    }

    #[test]
    fn play_test_input() {
        init_log();
        let mut monkeys = parse_monkeys(constants::MONKEY_INPUT).finish().unwrap().1;
        play(
            &mut monkeys,
            Settings {
                rounds: 20,
                divide_by_3: true,
            },
        );
        println!("{:#?}", PrettyMonkeysItems(&monkeys));
        println!("{:#?}", PrettyMonkeysEvalCount(&monkeys));
        assert_eq!(get_monkey_business(&monkeys), 10605);
    }

    #[test]
    fn play_part1() {
        init_log();
        assert_eq!(part1(), 99852);
    }

    #[test]
    fn play_part2() {
        init_log();
        assert_eq!(part2(), 99852);
    }

    mod constants {
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
