#![allow(unused_imports, unused_macros)]

use log::{debug, error, trace, warn};
use monkey::parse_monkeys;
use nom::Finish;

mod monkey;

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

pub fn play(s: &str) {
    let mut monkeys = parse_monkeys(s).finish().unwrap().1;

    for round in 1..=20 {
        debug!("Round {round}:");
        for monkey in &mut monkeys {
            debug!("Monkey {}:", monkey.id());
            while let Some(item) = monkey.items_mut().pop_front() {
                debug!("{}", item);
                let worry_level = monkey.operation().evaluate(item);
            }
        }
    }
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;
    use env_logger::Env;
    use log::{debug, error, info, log_enabled, Level};

    fn init_log() {
        use std::io::Write;
        // WARN or above if RUST_LOG was not set
        let log_init_res = env_logger::Builder::from_env(Env::default().default_filter_or("warn"))
            .is_test(true) // pass logs to the test framework
            .format_timestamp(None)
            .format(|buf, record| {
                writeln!(
                    buf,
                    "{1} [{0}]: {2}",
                    format_args!("{0}", function!()),
                    record.level(),
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
        play(constants::MONKEY_INPUT);
    }

    #[test]
    fn play_part1() {
        init_log();
        play(include_str!("../input.txt"));
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
