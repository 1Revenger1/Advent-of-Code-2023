use crate::{Solution, SolutionPair};

///////////////////////////////////////////////////////////////////////////////

const NUMS: [&'static str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
const OPTIONS: [&'static str; 19] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

pub fn solve(str: String) -> SolutionPair {
    let sol1: u32 = str.lines().map(|line| {
        let digits:Vec<u32> = line.chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        if digits.len() == 0 {
            return 0;
        }
        (digits.first().unwrap() * 10) + digits.last().unwrap()
    }).sum();

    let sol2: u32 = str.lines().map(|line| {
        let mut last: u32 = 0;
        let mut first: u32 = 0;

        'outer: for i in 0..line.len() {
            for opt in OPTIONS {   
                if i + opt.len() > line.len() {
                    continue;
                }

                if &line[i..(i + opt.len())] == opt {
                    if NUMS.contains(&opt) {
                        first = NUMS.iter().position(|p| p == &opt).unwrap() as u32;
                    } else {
                        first = opt.chars().next().unwrap().to_digit(10).unwrap();
                    }

                    break 'outer;
                }
            }
        }

        'outer: for i in (1..=line.len()).rev() {
            for opt in OPTIONS {
                if i < opt.len() {
                    continue;
                }

                if &line[(i - opt.len())..(i)] == opt {
                    if NUMS.contains(&opt) {
                        last = NUMS.iter().position(|p| p == &opt).unwrap() as u32;
                    } else {
                        last = opt.chars().next().unwrap().to_digit(10).unwrap();
                    }

                    break 'outer;
                }
            }
        }

        (first * 10) + last
    }).sum();

    (Solution::from(sol1), Solution::from(sol2))
}
