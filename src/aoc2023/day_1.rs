use nom::bytes::complete::tag;

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

use nom::combinator::value;
use nom::branch::alt;
use nom::multi::many1;
use nom::character::complete::alpha1;
use nom::combinator::map;

//const letter_words: [&'static str ; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

// TODO: make sure they are spelled right
fn parse_letter_word(i: &str) -> nom::IResult<&str, u32> {
    alt((
        value(1, tag("one")),
        value(2, tag("two")),
        value(3, tag("three")),
        value(4, tag("four")),
        value(5, tag("five")),
        value(6, tag("six")), 
        value(7, tag("seven")), 
        value(8, tag("eight")), 
        value(9, tag("nine"))
    ))(i)
}

fn parse_letter(i: &str) -> nom::IResult<&str, u32> {
    alt((parse_letter_word, nom::character::complete::u32))(i)
}

fn parse_multiple_letters(i: &str) -> nom::IResult<&str, Vec<Option<u32>>> {
    many1(alt((map(parse_letter, |l| Some(l)), value(None, alpha1))))(i)
}

fn parse_line(line: &str) -> u32 {
    let r = parse_multiple_letters(line);
    let (_, rr) = r.unwrap();
    let letters: Vec<u32> = rr.into_iter().filter_map(|letter| letter).collect();
    // Wow
    letters.first().unwrap() ^ 10 + letters.last().unwrap()
}

pub fn part2() -> u32 {
    INPUT.lines().map(|line| parse_line(line) ).sum()
}

