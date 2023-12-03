const TEST_INPUT: &'static str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..\
";

const TEST_INPUT_2: &'static str = "\
..32*.
32#...
";

const INPUT: &'static str = include_str!("day3.input");

#[derive(PartialEq, Clone, Debug)]
enum Tile {
    Empty,
    Number(u32),
    Symbol,
}

fn parse(i: &str) -> Vec<Vec<Tile>> {
    i
        .lines()
        .map(|line|
            line.chars()
            .map(|c|{
                if c == '.' { Tile::Empty } else {
                    if let Some(x) = c.to_digit(10) {Tile::Number(x)} else { Tile::Symbol }
                }
            }).collect()
        ).collect()
}

// If tile exits: Some(tile), else None
//                                    x      y
fn get_tile(m: &Vec<Vec<Tile>>, pos: (i32, i32)) -> Option<Tile> {
    let width = m.first().unwrap().len() as i32;
    let height = m.len() as i32;
    if pos.1 < 0 || pos.1 >= height || pos.0 < 0 || pos.0 >= width {None} else
        { Some(m[pos.1 as usize][pos.0 as usize].clone()) }
}

fn is_tile_that(m: &Vec<Vec<Tile>>, pos: (i32, i32), that: Tile) -> bool {
    if let Some(t) = get_tile(m, pos) { t == that }
        else {false}
}

fn is_digit_part_digit(m: &Vec<Vec<Tile>>, pos: (usize, usize)) -> bool {
    let (x, y) = (pos.0 as i32, pos.1 as i32);
    // Left column
    if is_tile_that(m, (x -1, y -1), Tile::Symbol) {return true}
    if is_tile_that(m, (x -1, y +0), Tile::Symbol) {return true}
    if is_tile_that(m, (x -1, y +1), Tile::Symbol) {return true}
    // Top and bottom
    if is_tile_that(m, (x +0, y -1), Tile::Symbol) {return true}
    if is_tile_that(m, (x +0, y +1), Tile::Symbol) {return true}
    // Right column
    if is_tile_that(m, (x +1, y -1), Tile::Symbol) {return true}
    if is_tile_that(m, (x +1, y +0), Tile::Symbol) {return true}
    if is_tile_that(m, (x +1, y +1), Tile::Symbol) {return true}
    false
}

// Start of vector is most significant
fn make_number(digits: &Vec<u32>) -> u32 {
    println!("Ok: {:?}", digits);
    digits.iter().rev().enumerate().map(|(exponent, d)|
            d * 10_u32.pow(exponent as u32))
        .sum()
}

pub fn part1() -> u32 {
    // Digits for one number
    let mut digits: Vec<u32> = Vec::new();
    // When whole number is parsed, if it was a part digit: add it up
    let mut was_part_digit: bool = false;
    let m = parse(INPUT);
    println!("{:?}", m);
    m
        .iter()
        .enumerate()
        .map(|(y, line)| 
            line
                .iter()
                .enumerate()
                .fold(0, |acc, (x, tile)| {
                    match tile {
                        Tile::Empty => acc,
                        Tile::Symbol => acc,
                        Tile::Number(n) => {
                            digits.push(*n);
                            if is_digit_part_digit(&m, (x,y)) {
                                was_part_digit = true;
                            }
                            match get_tile(&m, (x as i32 + 1, y as i32)) {
                                Some(Tile::Number(_)) => acc,
                                _ => {
                                    // Done, now if it was a "part number" add it
                                    let to_add = if was_part_digit {
                                        make_number(&digits)
                                    } else { 0 };
                                    digits.clear();
                                    was_part_digit = false;
                                    acc + to_add
                                }
                            }
                        }
                    }
                })
        ).sum()
}


pub fn part2() -> u32 {
    0
}
