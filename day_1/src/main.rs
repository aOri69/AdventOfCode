use calorie_counting::GroupSumIter;
use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = include_str!("../input_test.txt");

    let sum = calculate_max_calories_from_str(input);
    println!("-----------------------------------------");
    println!("{}", sum);
    println!("-----------------------------------------");
    println!("Top 3: {:?}", calculate_top_three(input));

    let input = include_str!("../input.txt");

    let sum = calculate_max_calories_from_str(input);
    println!("-----------------------------------------");
    println!("{}", sum);
    println!("-----------------------------------------");
    println!("Top 3: {:?}", calculate_top_three(input));

    println!("-----------------------------------------");
    println!("----------FasterThanLiMe way-------------");
    println!("-----------------------------------------");
    // FasterThanLiMe way
    {
        let lines = include_str!("../input.txt")
            .lines()
            .map(|v| v.parse::<u64>().ok())
            .collect::<Vec<_>>();
        // Just in case=)
        let _groups = lines
            .split(|line| line.is_none())
            .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
            .collect::<Vec<_>>();
        // println!("groups = {groups:?}");
        // Actual result of the task
        let elven_lead = lines
            .split(|line| line.is_none())
            .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
            .max();
        println!("{elven_lead:?}");
    }
    // Selfwritten iterator way
    {
        let lines = include_str!("../input.txt")
            .lines()
            .map(|v| v.parse::<u64>().ok());
        // here! 👋
        let elven_lead = GroupSumIter { inner: lines }.max();
        println!("{elven_lead:?}");
    }
    // Itertools batching way
    {
        let max = include_str!("../input.txt")
            .lines()
            .map(|v| v.parse::<u64>().ok())
            .batching(|it| {
                let mut sum = None;
                while let Some(Some(v)) = it.next() {
                    sum = Some(sum.unwrap_or(0) + v);
                }
                sum
            })
            .max();
        println!("{max:?}");
    }
    // Itertools coalesce way
    {
        let max = include_str!("../input.txt")
            .lines()
            .map(|v| v.parse::<u64>().ok())
            .coalesce(|a, b| match (a, b) {
                (None, None) => Ok(None),
                (None, Some(b)) => Ok(Some(b)),
                (Some(a), Some(b)) => Ok(Some(a + b)),
                (Some(a), None) => Err((Some(a), None)),
            })
            .max()
            .flatten()
            .unwrap_or_default();
        println!("{max:?}");
    }

    Ok(())
}

fn calculate_max_calories_from_str(input: &str) -> u32 {
    let splitter = match input.find('\r') {
        Some(_) => "\r\n\r\n",
        None => "\n\n",
    };

    // dbg!(splitter);
    let sum = input
        .split(splitter)
        .map(|group| {
            // dbg!(group);
            group
                .lines()
                .map(|value| {
                    // dbg!(value);
                    value.parse::<u32>().unwrap()
                })
                .sum::<u32>()
        })
        .max();

    sum.unwrap_or(0)
}

fn calculate_top_three(input: &str) -> u32 {
    let splitter = match input.find('\r') {
        Some(_) => "\r\n\r\n",
        None => "\n\n",
    };

    // dbg!(splitter);
    let mut sum: Vec<_> = input
        .split(splitter)
        .map(|group| {
            // dbg!(group);
            group
                .lines()
                .map(|value| {
                    // dbg!(value);
                    value.parse::<u32>().unwrap()
                })
                .sum::<u32>()
        })
        .collect();
    sum.sort_by(|a, b| b.cmp(a));
    sum.iter().take(3).sum()
}
