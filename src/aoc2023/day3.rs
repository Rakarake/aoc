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

#[derive(PartialEq)]
enum Tile {
    Empty,
    Number(u32),
    Symbol,
}

enum TileP {
    Empty,
    Number(usize),
    Symbol,
}

fn parse(i: &str) -> Vec<Vec<Tile>> {
    i
        .lines()
        .map(|line|
            line.chars()
            .map(|c|{
                if c == 'c' { Tile::Empty } else {
                    if let Some(x) = c.to_digit(10) {Tile::Number(x)} else { Tile::Symbol }
                }
            }).collect()
        ).collect()
}

pub fn part1() -> u32 {
    let parsed = parse(TEST_INPUT);
    let height = parsed.len();
    let width = parsed.first().unwrap().len();
    let mut is_part_number = false;
    let mut digits = Vec::new();
    for (y, row) in parsed.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            // If tile is number
            match tile {
                Tile::Empty => {
                    is_part_number = false; 
                    // If digits is not empty => process whole number
                },
                Tile::Number(n) => {
                    // If not already a part number, check in all directions if part number
                    if y != 0 {
                        if parsed[y-1][x] == Tile::Symbol { is_part_number = true; }
                    }
                    if y != height-1 {
                        if parsed[y+1][x] == Tile::Symbol { is_part_number = true; }
                    }
                    digits.push(n);
                    if x + 1 == width {
                        // Process whole number
                    }
                },
                Tile::Symbol => {
                    // If digits is not empty => process whole number
                },
            }
        }
    }
    0
}


pub fn part2() -> u32 {
    0
}
