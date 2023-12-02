const TEST_INPUT: &'static str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\
";

const INPUT: &'static str = include_str!("day2.input");

use nom::IResult;
use nom::combinator::value;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::sequence::pair;
use nom::character::complete::u32;
use nom::multi::separated_list0;
use nom::bytes::complete::take_until;

#[derive(Clone, Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

fn parse_color(i: &str) -> IResult<&str, (u32, Color)> {
    pair(u32, alt((
                  value(Color::Red, tag(" red")),
                  value(Color::Green, tag(" green")),
                  value(Color::Blue, tag(" blue"))
              ))
    )(i)
}

fn parse_throw(i: &str) -> IResult<&str, Vec<(u32, Color)>> {
    separated_list0(tag(", "), parse_color)(i)
}

fn parse_line(i: &str) -> IResult<&str, Vec<Vec<(u32, Color)>>> {
    let (i, _) = take_until(": ")(i)?;
    let (i, _) = tag(": ")(i)?;
    separated_list0(tag("; "), parse_throw)(i)
}

fn line_highest_colors(i: &str) -> (u32, u32, u32) {
    let (_, throws) = parse_line(i).unwrap();
    let z = throws.concat();
    (z.iter().max_by_key(|c| if c.1 == Color::Red {c.0} else {0}).unwrap().0
    ,z.iter().max_by_key(|c| if c.1 == Color::Green {c.0} else {0}).unwrap().0
    ,z.iter().max_by_key(|c| if c.1 == Color::Blue {c.0} else {0}).unwrap().0)
}

pub fn part1() -> u32 {
    INPUT.lines().map(|l| {
        line_highest_colors(l)
    })
    .enumerate()
    .fold(0, |acc, (index, x)| {
        // Is the line possible?
        if x.0 > 12 || x.1 > 13 || x.2 > 14 {acc} else {acc + (index as u32) +1}
    })
}

pub fn part2() -> u32 {
    0
}

