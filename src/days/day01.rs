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
        let mut first_idx: usize = usize::MAX;
        let mut last_idx: usize = 0;
        let mut last: u32 = 0;
        let mut first: u32 = 0;

        for opt in OPTIONS {
            let idx = line.find(opt).unwrap_or_else(|| usize::MAX);
            if idx <= first_idx {
                if NUMS.contains(&opt) {
                    first = NUMS.iter().position(|p| p == &opt).unwrap() as u32;
                } else {
                    first = opt.chars().next().unwrap().to_digit(10).unwrap();
                }
                
                first_idx = idx;
            }
            
            let idx = line.rfind(opt).unwrap_or_else(|| usize::MAX);
            if idx >= last_idx && idx != usize::MAX {
                if NUMS.contains(&opt) {
                    last = NUMS.iter().position(|p| p == &opt).unwrap() as u32;
                } else {
                    last = opt.chars().next().unwrap().to_digit(10).unwrap();
                }

                last_idx = idx;
            }

        }

        (first * 10) + last
    }).sum();

    (Solution::from(sol1), Solution::from(sol2))
}
