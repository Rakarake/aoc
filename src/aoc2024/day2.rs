const TEST_INPUT: &'static str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9\
";

fn safe() {
}

fn parse(i: &str) -> Vec<Vec<i32>> {
    i.lines().map(|l|
        l
            .split_whitespace()
            .map(|w| w.parse::<i32>().unwrap())
            .collect()
    ).collect()
}

fn decending(i: Vec<i32>) -> bool {
    let (de, _) = i.iter().fold((true, i32::MAX), |(de, prev), n| (de && *n < prev, *n));
    de
}
fn acending(i: Vec<i32>) -> bool {
    let (asc, _) = i.iter().fold((true, 0), |(asc, prev), n| (asc && *n > prev, *n));
    asc
}
fn follows_rules(i: &Vec<i32>) -> bool {
    let (de, _, _) = i.iter().fold((true, 0, 0),
        |(de, prev, sig), n|
            (de && (n - prev).abs() <= 3 && ((n - prev).signum() - sig).abs() <= 1, *n, (n-prev)));
    de
}

pub fn part1() -> u32 {
    let input = parse(TEST_INPUT);
    input.iter().map(|l| if follows_rules(l) { 1 } else { 0 }).sum()
}

pub fn part2() -> u32 {
    0
}

