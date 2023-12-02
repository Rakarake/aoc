const TEST_INPUT: &'static str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000\
";

const INPUT: &'static str = include_str!("day1.input");

pub fn part1() -> u32 {
    elf_sums(INPUT).into_iter().max().unwrap()
}

fn elf_sums(input: &'static str) -> Vec<u32> {
    input
        .split("\n\n")
        .map(|section| {
            section
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect()
}

pub fn part2() -> u32 {
    let num_sums = elf_sums(INPUT);
    // Ascending order
    let mut top_three_tmp: [u32; 3] = [0; 3];
    for num in num_sums {
        // Integrate new num
        if num > top_three_tmp[0] {
            if num > top_three_tmp[1] {
                if num > top_three_tmp[2] {
                    top_three_tmp[0] = top_three_tmp[1];
                    top_three_tmp[1] = top_three_tmp[2];
                    top_three_tmp[2] = num;
                } else {
                    top_three_tmp[0] = top_three_tmp[1];
                    top_three_tmp[1] = num;
                }
            } else {
                top_three_tmp[0] = num;
            }
        }
    }
    top_three_tmp.iter().sum()
}
