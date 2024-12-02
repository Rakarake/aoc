pub mod day1;
pub mod day2;

use aoc_table::table_gen::TableGen;

pub fn run_all() {
    TableGen::new("Among us funny")
        .add(1, day1::part1, day1::part2)
        .add(2, day2::part1, day2::part2)
        .run();
}
