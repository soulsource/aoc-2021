use aoc_runner_derive::*;
use std::str::FromStr;

#[aoc_generator(day1)]
pub fn input_generator<'c>(input : &'c str) -> Vec<u32>{ 
    input.lines().map(u32::from_str).map(Result::unwrap).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input : &Vec<u32>) -> u32 {
    struct Helper {
        count : u32,
        prev : u32,
    }
    input.iter().fold(Helper{count:0,prev:u32::MAX},|x ,curr| -> Helper {if curr > &x.prev {Helper{count : x.count + 1, prev : *curr}} else {Helper{count : x.count, prev : *curr}}}).count
}
