const TEST_INPUT: &'static str = "\
OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....\
";

const INPUT: &'static str = include_str!("day14.input");

#[derive(Clone)]
enum Tile {
    Empty,
    Round,
    Cube,
}

fn parse(i: &'static str) -> Vec<Vec<Tile>> {
    use Tile::*;
    i.lines().map(|l| {
        l.chars().map(|c| match c {
            'O' => Round,
            '#' => Cube,
            '.' => Empty,
            _ => panic!("Noooo!!!"),
        }).collect()
    }).collect()
}

// Roll upwards!
fn roll(i: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    use crate::utils::Lel;
    i.transpose().into_iter().map(|col| {
        col
            .into_iter()
            .enumerate()
            .rev()
            .fold((vec![], vec![]), |(mut res, acc), (y, t)| {
                // If has stones in acc and next is a cube/end: empty acc into res
                if !acc.is_empty() {
                }
                res.push(t);
                (res, acc)
            }).0
    }).collect()
}

pub fn part1() -> u32 {
    //parse(TEST_INPUT).into_iter().
    0
}

pub fn part2() -> u32 {
    0
}

