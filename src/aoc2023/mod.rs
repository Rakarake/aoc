// Put all the days here

pub mod day1;
pub mod day14;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day_test;

use aoc_table::table_gen::TableGen;

pub fn run_all() {
    TableGen::new("Morbin' AOC 2023 Table 🎅 hoho")
        .add(0, day_test::part1, day_test::part2)
        .add(1, day1::part1, day1::part2)
        .add(2, day2::part1, day2::part2)
        .add(3, day3::part1, day3::part2)
        .add(4, day4::part1, day4::part2)
        .add(5, day5::part1, day5::part2)
        .add(6, day6::part1, day6::part2)
        .add(14, day14::part1, day14::part2)
        .run();
}
