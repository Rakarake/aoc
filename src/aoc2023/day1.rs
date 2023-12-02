use nom::bytes::complete::tag;

const TEST_INPUT: &'static str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet\
";
const TEST_INPUT_2: &'static str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen\
";

const INPUT: &'static str = include_str!("day1.input");

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

const DIGIT_WORDS: [&'static str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

// Ah, there is no match_indices_map function, ah
fn parse_digit(s: &str) -> u32 {
    if let Some(x) = s.chars().next().unwrap().to_digit(10) { x }
    else {
        DIGIT_WORDS.iter().position(|digit_word| digit_word.contains(s)).unwrap() as u32 + 1 
    }
}

pub fn part2() -> u32 {
    INPUT
        .lines()
        .map(|line| {
            // All digit words and their indices in the line ("one", "two" etc)
            let mut x: Vec<Vec<(usize, &str)>> = DIGIT_WORDS.iter().map(|digit_word| {
                line.match_indices(digit_word).collect()
            }).collect();
            // All digits (represented by Arabic numerals) and their indices
            let y: Vec<(usize, &str)> = line.match_indices(|c: char| c.is_digit(10)).collect();
            // Combine these
            x.push(y);
            // We are interested which occurrences comes first and last, so every group
            // needs to be in the same structure to be compared.
            let z = x.concat();
            let first = z.iter().min_by_key(|v| v.0).unwrap();
            let last = z.iter().max_by_key(|v| v.0).unwrap();
            parse_digit(first.1) * 10 + parse_digit(last.1)
         })
        .sum()
}

