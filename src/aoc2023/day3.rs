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
.664.598..
";

const INPUT: &'static str = include_str!("day3.input");

#[derive(PartialEq, Clone)]
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

fn is_tile_number(m: &Vec<Vec<Tile>>, pos: (i32, i32)) -> bool {
    if let Some(t) = get_tile(m, pos) { match t { Tile::Number(_) => true, _ => false } }
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

// Takes the position of the first digit of the number
// Fails if not the first
//                                           x      y
fn is_num_part_num(m: &Vec<Vec<Tile>>, pos: (usize, usize)) -> bool {
    let (x, y) = (pos.0 as i32, pos.1 as i32);
    is_tile_number(m, (x -1, y)) && (is_digit_part_digit(m, pos) || is_num_part_num(m, (pos.0 + 1, pos.1)))
}

fn all_part_numbers(m: &Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
    let acc: Vec<(usize, usize)> = Vec::new();
    m
        .iter()
        .enumerate()
        .fold(acc, |ps, (y, line)| {
            line
                .iter()
                .enumerate()
                .fold(ps, |mut psp, (x, tile)| {
                    if is_num_part_num(m, (x, y)) { psp.push((x, y)); psp } else {psp}
                })
        })
}

fn get_num_value(m: &Vec<Vec<Tile>>, pos: (usize, usize)) -> u32 {
    let x = &(m[pos.1])[pos.1 .. m.first().unwrap().len()];
    let y: Vec<&Tile> = x.iter().take_while(|t| match t { Tile::Number(_) => true, _ => false }).collect();
    let z: Vec<u32> = y.iter().map(|t| match t { Tile::Number(n) => n.clone(), _ => panic!("nOOOOOOO") }).collect();
    z.iter().rev().enumerate().fold(0, |n, (pow, digit)| n + digit * 10^(pow as u32))
}

pub fn part1() -> u32 {
    let m = parse(TEST_INPUT);
    let part_number_locations = all_part_numbers(&m);
    part_number_locations.iter().map(|pos| get_num_value(&m, pos.clone())).sum()
    //m
    //    .iter()
    //    .enumerate()
    //    .map(|(y, row)| {
    //        row
    //            .iter()
    //            .enumerate()
    //            .fold((0, Vec::new()), |(sum, ds), (x, t)| {
    //                if is_num_part_num(m, (x, y)) { sum +  }
    //            })
    //    })
    //    .sum()
    //let parsed = parse(TEST_INPUT);
    //let height = parsed.len();
    //let width = parsed.first().unwrap().len();
    //let mut is_part_number = false;
    //let mut digits = Vec::new();
    //for (y, row) in parsed.iter().enumerate() {
    //    for (x, tile) in row.iter().enumerate() {
    //        // If tile is number
    //        match tile {
    //            Tile::Empty => {
    //                is_part_number = false; 
    //                // If digits is not empty => process whole number
    //            },
    //            Tile::Number(n) => {
    //                // If not already a part number, check in all directions if part number
    //                if y != 0 {
    //                    if parsed[y-1][x] == Tile::Symbol { is_part_number = true; }
    //                }
    //                if y != height-1 {
    //                    if parsed[y+1][x] == Tile::Symbol { is_part_number = true; }
    //                }
    //                digits.push(n);
    //                if x + 1 == width {
    //                    // Process whole number
    //                }
    //            },
    //            Tile::Symbol => {
    //                // If digits is not empty => process whole number
    //            },
    //        }
    //    }
    //}
    //0
}


pub fn part2() -> u32 {
    0
}
