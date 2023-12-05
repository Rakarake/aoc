const INPUT: &'static str = include_str!("day4.input");
const TEST_INPUT: &'static str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4\
";

//                    ranges                     seeds
fn parse(i: &str) -> (Vec<Vec<(u32, u32, u32)>>, Vec<u32>) {
    let mut sections = i.split("\n\n");
    let seeds: Vec<u32> = sections
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let ranges: Vec<Vec<(u32, u32, u32)>> = sections.map(|s| {
        s.split('\n').skip(1).map(|l| {
            let nums: Vec<u32> = l.split(' ').map(|n| n.parse::<u32>().unwrap()).collect();
            (nums[0], nums[1], nums[2])
        }).collect()
    }).collect();
    (ranges, seeds)
}

// Convets number of one group to the next
fn convert(ranges: &Vec<Vec<(u32, u32, u32)>>, group_id: usize, num: u32) -> u32 {
    ranges[group_id].iter().find_map(|(dst_s, src_s, range)| {
        if num >= *src_s && num < src_s + range { Some(dst_s + (num - src_s)) } else { None }
    }).unwrap_or(num)
}

pub fn part1() -> u32 {
    let (ranges, seeds) = parse(INPUT);
    seeds.iter().map(|seed| {
        (0..7).into_iter().fold(*seed, |category_num, category_idx| {
            convert(&ranges, category_idx, category_num)
        })
    }).min().unwrap()
}

pub fn part2() -> u32 {
    0
}

