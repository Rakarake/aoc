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
use crate::utils::Lel;
use rayon::prelude::*;

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
    i.transpose()
        .into_par_iter()
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

fn total_load(i: Vec<Vec<Tile>>) -> u32 {
    i
        .into_par_iter()
        .rev()
        .enumerate()
        .map(|(y, l)|
            (l.into_par_iter().filter(|t| *t == Round).collect::<Vec<Tile>>().len() * (y + 1)) as u32
        ).sum()
}

fn full_rotation(mut i: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    for _ in 0..4 {
        i = i.rotate_counter_clockwise();
        i = roll(i);
    }
    i
}

pub fn part1() -> u32 {
    total_load(roll(parse(INPUT)))
}

pub fn part2() -> u32 {
    let mut tiles = parse(INPUT);
    let mut prev: Vec<Vec<Vec<Tile>>> = Vec::new();
    for i in 0..1000000000_u64 {
        println!("{:?}", i);
        tiles = full_rotation(tiles);
        // See if we have looped
        for p in prev.iter().rev().take(10) {
            if *p == tiles {
                break;
            }
        }
        prev.push(tiles.clone());
    }
    total_load(tiles)
}
