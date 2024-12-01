pub mod day1;

use aoc_table::table_gen::TableGen;

pub fn run_all() {
    TableGen::new("Among us funny")
        .add(1, day1::part1, day1::part2)
        .run();
}
