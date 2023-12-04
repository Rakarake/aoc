const TEST_INPUT: &'static str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\
";

const INPUT: &'static str = include_str!("day4.input");

fn parse_nums(i: &str) -> Vec<u32> {
    i.split(' ').filter_map(|word| word.parse::<u32>().ok()).collect()
}

fn parse(i: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    i.lines().map(|l| {
        let (s1, s2) = l.split_once('|').unwrap();
        (parse_nums(s1.split_once(": ").unwrap().1), parse_nums(s2))
    }).collect()
}

fn get_line_winning_cards((c1, c2): (Vec<u32>, Vec<u32>)) -> Vec<u32> {
    // get-num, win-num
    c2.iter().filter_map(|g_n| c1.iter().find(|w_n| *w_n == g_n)).copied().collect()
}

fn make_score_from_line(n_hits: u32) -> u32 {
    if n_hits == 0 {
        0
    }  else {
        2_u32.pow(n_hits -1)
    }
}

pub fn part1() -> u32 {
    parse(INPUT).iter().map(|line| make_score_from_line(get_line_winning_cards(line.clone()).len() as u32)).sum()
}

pub fn part2() -> u32 {
    0
}

