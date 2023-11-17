pub mod day_1;

use aoc_table::table_gen::TableGen;

pub fn run_all() {
    TableGen::new("Hoho, solutions to problems?")
        .add_next(day_1::part1, || 0)
        .run();
}
