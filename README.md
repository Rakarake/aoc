# Advent of Code 🎅 hoho

Ok, to run a specific part, add that to to a `src/main.rs`. The file is
in the .gitignore, so it is for working on solutions.
Run with `cargo run --bin aoc`.

A `main.rs` could look like this:
```rust
use aoc::aoc2023::day_test;

fn main() {
    println!("Part 1: {:?}", day_test::part1());
    println!("Part 2: {:?}", day_test::part2());
}
```

When solving a day:
* Write the solution to `src/<year>/day_<day>.rs`
* Write the input to `src/<year>/day_<day>_input.txt`
* Add the solution to that year's table: `src/<year>/mod.rs`

