const TEST_INPUT: &'static str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))\
";

const INPUT: &'static str = include_str!("day3.input");

enum Segment {
    Section(&'static str),
    Num,
}

pub fn part1() -> u32 {
    const NUM_LENGTH: u8 = 3;
    let mut sum = 0u32;
    let mut digits_state: [char; 3];
    let mut segment_state = 0usize;
    use Segment::*;
    let mut segments = [
        (0u8, Section("mul(")),
        (0u8, Num),
        (0u8, Section(",")),
        (0u8, Num),
        (0u8, Section(")")),
    ];
    TEST_INPUT.chars().for_each(|c| {
        let (state, segment) = segments[segment_state];
        match segment {
            Section(str) => {
                if !c == str[state] {
                    state = 0;
                    // TODO fix state of all segments
                }
                if state == str.len() { segment_state += 1 }
            },
            Num => {
                if state == NUM_LENGTH { segment_state += 1 }
            },
        }
    });
}
pub fn part2() -> u32 {
    0
}
