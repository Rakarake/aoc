const TEST_INPUT: &'static str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....\
";

const INPUT: &'static str = include_str!("day14.input");

#[derive(Clone, PartialEq, Debug)]
enum Tile {
    Empty,
    Round,
    Cube,
}
use Tile::*;

fn parse(i: &'static str) -> Vec<Vec<Tile>> {
    i.lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'O' => Round,
                    '#' => Cube,
                    '.' => Empty,
                    _ => panic!("Noooo!!!"),
                })
                .collect()
        })
        .collect()
}

// Roll upwards!
fn roll(i: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    use crate::utils::Lel;
    i.transpose()
        .into_iter()
        .map(|col| {
            match col
                .into_iter()
                .rev()
                .fold((vec![], vec![]), |(mut res, mut acc), t| {
                    if t == Round {
                        acc.push(t);
                    } else {
                        // If a cube or at border, the boulders stop
                        if t == Cube {
                            // Roll out all the round rocks!
                            res.append(&mut acc)
                        }
                        res.push(t);
                    }
                    (res, acc)
                }) {
                (mut r, mut a) => {
                    r.append(&mut a);
                    r.reverse();
                    r
                }
            }
        })
        .collect::<Vec<Vec<Tile>>>()
        .transpose()
}

pub fn part1() -> u32 {
    roll(parse(INPUT))
        .into_iter()
        .rev()
        .enumerate()
        .map(|(y, l)|
            (l.into_iter().filter(|t| *t == Round).collect::<Vec<Tile>>().len() * (y + 1)) as u32
        ).sum()
}

pub fn part2() -> u32 {
    0
}
