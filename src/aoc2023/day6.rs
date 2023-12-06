const INPUT: &'static str = include_str!("day6.input");
const TEST_INPUT: &'static str = "\
Time:      7  15   30
Distance:  9  40  200\
";

const REAL_INPUT_FR: [(u32, u32); 4] = [(53, 250), (91, 1330), (67, 1081), (68, 1025)];

//                                                        higher
fn get_bounds(tr: f64, d: f64) -> f64 {
    (
        ((tr/2f64) + ((tr/2f64).powi(2) - d).sqrt())
            -
        ((tr/2f64) - ((tr/2f64).powi(2) - d).sqrt())
    ).abs()
}

pub fn part1() -> f64 {
    REAL_INPUT_FR.into_iter().map(|(t, d)| get_bounds(t as f64, d as f64)).fold(1.0, |o, v| o * v)
}

pub fn part2() -> u32 {
    0
}

