const TEST_INPUT: &'static str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))\
";

const INPUT: &'static str = include_str!("day3.input");

enum Segment {
    Section(&'static str, u8),
    Num,
}

pub fn part1() -> u32 {
    let mut state: u8 = 0;
    const NUM_LENGTH: u8 = 3;
    use Segment::*;
    let segments = [
        Section("mul(", 4),
        Num,
        Section(",", 1),
        Num,
        Section(")", 1),
    ];
    TEST_INPUT.chars().for_each(|c| {
        
    });
}
pub fn part2() -> u32 {
    0
}
