use aoc_runner_derive::*;
use std::str::FromStr;

#[aoc_generator(day1)]
pub fn input_generator<'c>(input : &'c str) -> Vec<u32>{ 
    input.lines().map(u32::from_str).map(Result::unwrap).collect()
}

#[aoc(day1, part1, InternalState)]
pub fn solve_part1(input : &Vec<u32>) -> u32 {
    struct Helper {
        count : u32,
        prev : u32,
    }
    input.iter().fold(Helper{count:0,prev:u32::MAX},|x ,curr| -> Helper {if curr > &x.prev {Helper{count : x.count + 1, prev : *curr}} else {Helper{count : x.count, prev : *curr}}}).count
}

#[aoc(day1, part1, Iterators)]
pub fn solve_part1_iterators(input : &Vec<u32>) -> usize {
    input.iter().skip(1).zip(input.iter()).filter(|(new, old)| new > old).count()
}

#[aoc(day1, part2, InternalState)]
pub fn solve_part2(input : &Vec<u32>) -> usize {
    let floating_window :u32 = input.iter().take(3).sum();
    input.iter().skip(3).zip(input.iter()).scan(floating_window, |floating_window, (new, old)| {
        let prev_window = *floating_window;
        *floating_window = (*floating_window + *new) - *old;
        Some((*floating_window, prev_window))
    }).filter(|(new, old)| new > old).count()
}

#[aoc(day1,part2, DumbLoop)]
pub fn solve_part2_dumb_loop(input : &Vec<u32>) -> usize {
    if input.len() < 3 {
        0
    }
    else {
        let mut floating_window = input[0]+input[1]+input[2];
        let mut sum = 0;
        for index in 3..input.len() {
            let old = floating_window;
            floating_window = (floating_window + input[index]) - input[index -3];
            if floating_window > old {
                sum = sum + 1;
            }
        }
        sum
    }
}

#[aoc(day1, part2, Iterators)]
pub fn solve_part2_iterators(input : &Vec<u32>) -> usize {
    let sliding_window = input.iter().skip(2).zip(input.iter().skip(1)).zip(input.iter())
        .map(|((third, second), first)| third + second + first);
    let old_window = sliding_window.clone();
    let sliding_window = sliding_window.skip(1);
    sliding_window.zip(old_window).filter(|(new, old)| new > old).count()
}
