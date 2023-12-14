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
//fn roll(i: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
//    
//}

pub fn part1() -> u32 {
    //parse(TEST_INPUT).into_iter().
    0
}

pub fn part2() -> u32 {
    0
}

