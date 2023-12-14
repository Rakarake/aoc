const INPUT: &'static str = include_str!("day6.input");
const TEST_INPUT: &'static str = "\
Time:      7  15   30
Distance:  9  40  200\
";

const REAL_INPUT_FR: [(u32, u32); 4] = [(53, 250), (91, 1330), (67, 1081), (68, 1025)];
const TEST_REAL_INPUT_FR: [(u32, u32); 3] = [(7, 9), (15, 40), (30, 200)];

//use bigdecimal::BigDecimal;
//use bigdecimal::rounding::RoundingMode;
//
////                                                        higher
//fn get_bounds(tr: BigDecimal, d: BigDecimal) -> u32 {
//    //println!("first: {:?}", (((tr/2.0) + ((tr/2.0).powi(2) - d).sqrt()).ceil() as i32));
//    let lil_div = tr / BigDecimal::from(2);
//    ((lil_div + (lil_div * lil_div - d).sqrt()).() as u32)
//        -
//    (((tr/2.0) - ((tr/2.0).powf(2.0) - d).sqrt()).ceil() as u32)
//}

pub fn part1() -> u32 {
    //TEST_REAL_INPUT_FR.into_iter().map(|(t, d)| get_bounds(t as BigDecimal, d as BigDecimal)).fold(1, |o, v| {println!("hi: {:?}", v);o * v})
    0
}

pub fn part2() -> u32 {
    0
}

