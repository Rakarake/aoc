// Put all the days here

pub mod day_test;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

use aoc_table::table_gen::TableGen;

pub fn run_all() {
    TableGen::new("Morbin' AOC 2023 Table 🎅 hoho")
        .add(0, day_test::part1, day_test::part2)
        .add_next(day1::part1, day1::part2)
        .add_next(day2::part1, day2::part2)
        .add_next(day3::part1, day3::part2)
        .add_next(day4::part1, day4::part2)
        .run();
}
