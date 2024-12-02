const TEST_INPUT: &'static str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9\
";

const INPUT: &'static str = include_str!("day2.input");

fn parse(i: &str) -> Vec<Vec<i32>> {
    i.lines().map(|l|
        l
            .split_whitespace()
            .map(|w| w.parse::<i32>().unwrap())
            .collect()
    ).collect()
}

fn follows_rules(i: &Vec<i32>) -> Vec<bool> {
    let mut iter = i.iter();
    let first = iter.next().unwrap();
    let second = iter.next().unwrap();
    let initial_diff = second - first;
    let initial_holds = initial_diff.abs() <= 3 && initial_diff.abs() >= 1;
    let (acc, _, _) = iter.fold((vec![initial_holds], *second, initial_diff),
        |(mut acc, prev, prev_diff): (Vec<bool>, i32, i32), n: &i32| {
            let diff: i32 = n - prev;
            acc.push(
                diff.abs() <= 3 &&
                diff.abs() >= 1 &&
                ((diff > 0 && prev_diff > 0) || (diff < 0 && prev_diff < 0))
            );
            (acc, *n, diff)
        });
    acc
}

fn first_unsafe(i: &Vec<bool>) -> Option<usize> {
    i.iter().position(|x| !*x)
}

pub fn part1() -> u32 {
    let input = parse(INPUT);
    input.iter().map(|l| if follows_rules(l).iter().all(|x| *x) { 1 } else { 0 }).sum()
}

pub fn part2() -> u32 {
    let mut input = parse(INPUT);
    input.iter_mut().map(|l| {
        let mut rules = follows_rules(l);
        if let Some(i) = first_unsafe(&rules) {
            l.remove(i);
            // Recalculate if the level is fine
            rules = follows_rules(l);
        }
        if rules.iter().all(|x| *x) { 1 } else { 0 }
    }).sum()
}

