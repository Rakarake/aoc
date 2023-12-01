const TEST_INPUT: &'static str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet\
";

const INPUT: &'static str = include_str!("day_1.input");

pub fn part1() -> u32 {
    INPUT
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars()
                .filter(|c| c.is_digit(10))
                .collect();
            let (last, first) = (chars.first().unwrap(), chars.last().unwrap());
            String::from_iter([last, first].into_iter()).parse::<u32>().unwrap()
        })
        .sum()
}
