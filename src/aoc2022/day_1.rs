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

const INPUT: &'static str = include_str!("day_1.input");

pub fn part1() -> u32 {
    let a: Vec<Vec<u32>> = Vec::new();
    let nums = INPUT.lines().map(|line| line.parse::<u32>());
    let num_groups = nums.fold(a, |mut acc, maybe_n| {
        match maybe_n {
            Ok(n) => {
                match acc.last_mut() {
                    Some(last) => {
                        last.push(n)
                    }
                    None => {
                        let new_group: Vec<u32> = vec![n];
                        acc.push(new_group);
                    }
                }
            },
            Err(_e) => {
                acc.push(Vec::new())
            }
        }
        acc
    });
    let num_sums = num_groups.iter().map(|g| g.iter().fold(0, |sum, x| sum + x));
    let mut max = 0;
    num_sums.for_each(|sum| if sum > max { max = sum });
    max
}

pub fn part2() -> u32 {
    let a: Vec<Vec<u32>> = Vec::new();
    let nums = INPUT.lines().map(|line| line.parse::<u32>());
    let num_groups = nums.fold(a, |mut acc, maybe_n| {
        match maybe_n {
            Ok(n) => {
                match acc.last_mut() {
                    Some(last) => {
                        last.push(n)
                    }
                    None => {
                        let new_group: Vec<u32> = vec![n];
                        acc.push(new_group);
                    }
                }
            },
            Err(_e) => {
                acc.push(Vec::new())
            }
        }
        acc
    });
    let num_sums = num_groups.iter().map(|g| g.iter().fold(0, |sum, x| sum + x));
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

