const INPUT: &'static str = include_str!("day5.input");
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
fn parse(i: &str) -> (Vec<Vec<(u64, u64, u64)>>, Vec<u64>) {
    let mut sections = i.split("\n\n");
    let seeds: Vec<u64> = sections
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let ranges: Vec<Vec<(u64, u64, u64)>> = sections.map(|s| {
        s.split('\n').skip(1).filter_map(|l| {
            let nums: Vec<u64> = l.split(' ').filter_map(|n| n.parse::<u64>().ok()).collect();
            if nums.len() != 3 { None } else { Some((nums[0], nums[1], nums[2])) }
        }).collect()
    }).collect();
    (ranges, seeds)
}

// Convets number of one group to the next
fn convert(ranges: &Vec<Vec<(u64, u64, u64)>>, group_id: usize, num: u64) -> u64 {
    ranges[group_id].iter().find_map(|(dst_s, src_s, range)| {
        if num >= *src_s && num < src_s + range { Some(dst_s + (num - src_s)) } else { None }
    }).unwrap_or(num)
}

pub fn part1() -> u64 {
    let (ranges, seeds) = parse(INPUT);
    seeds.iter().map(|seed| {
        (0..7).into_iter().fold(*seed, |category_num, category_idx| {
            convert(&ranges, category_idx, category_num)
        })
    }).min().unwrap()
}

pub fn part2() -> u64 {
    let (ranges, seed_nums) = parse(INPUT);
    let seed_ranges: Vec<(u64, u64)> = seed_nums
        .iter()
        .copied()
        .enumerate()
        .collect::<Vec<(usize, u64)>>()
        .split_inclusive(|(i, v)| i % 2 == 1)
        .map(|v| (v[0].1, v[1].1)).collect();
    seed_ranges.iter()
        .map(|(start, len)| (*start..(*start + *len)).into_iter().map(|seed| {
            (0..7).into_iter().fold(seed, |category_num, category_idx| {
                convert(&ranges, category_idx, category_num)
            })
        }).min().unwrap()).min().unwrap()
}

