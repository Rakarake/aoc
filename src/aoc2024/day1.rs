const TEST_INPUT: &'static str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

const INPUT: &'static str = include_str!("day1.input");

fn parse(i: &str) -> Vec<(i32, i32)> {
    i
        .lines()
        .map(|l| {
            let ns = l
                .split_once("   ")
                .unwrap();
            (ns.0.parse::<i32>().unwrap(), ns.1.parse::<i32>().unwrap())
        }
        ).collect()
}

fn the_lists(i: Vec<(i32, i32)>) -> (Vec<i32>, Vec<i32>) {
    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];
    for (left, right) in i {
        left_list.push(left);
        right_list.push(right);
    }
    (left_list, right_list)
}

pub fn part1() -> i32 {
    let i = parse(INPUT);
    let (mut left_list, mut right_list) = the_lists(i);
    left_list.sort();
    right_list.sort();
    use std::iter::zip;
    zip(left_list.iter(), right_list.iter()).fold(0, |acc, (l, r)| acc + (l - r).abs())
}

pub fn part2() -> i32 {
    let i = parse(INPUT);
    let (mut left_list, mut right_list) = the_lists(i);
    left_list.iter().fold(0, |acc, n1|
        acc + n1 * right_list.iter().filter(|n2| n1 == *n2).count() as i32)
}
