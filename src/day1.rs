use aoc_runner_derive::*;
use std::str::FromStr;
use std::ops::Add;

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

enum FloatingWindow {
    Prewarming{
        value : u32,
        previous : [u32;2],
        index : usize,
    },
    Ready{
        value : u32,
        previous : [u32;3],
        index : usize,
    },
}

impl FloatingWindow {
    fn try_get(&self) -> Option<u32> {
        match self {
            FloatingWindow::Prewarming{..} => { None }
            FloatingWindow::Ready{value, ..} => { Some(*value) }
        }
    }
}

impl Default for FloatingWindow {
    fn default() -> Self {
        FloatingWindow::Prewarming{value:0,previous:[0;2],index:0}
    }
}

impl Add<u32> for FloatingWindow {
    type Output = FloatingWindow;
    fn add(self, rhs : u32) -> FloatingWindow {
        match self {
            FloatingWindow::Prewarming{value, previous, index} if index < 2 => {
                let mut arr = previous;
                arr[index] = rhs;
                FloatingWindow::Prewarming{value: value + rhs, previous : arr, index: index + 1}
            }
            FloatingWindow::Prewarming{value, previous, index} => {
                let arr = [previous[0], previous[1], rhs];
                FloatingWindow::Ready{value: value + rhs, previous : arr, index: 0}
            }
            FloatingWindow::Ready{value, previous, index} => {
                let mut arr = previous;
                let old = arr[index];
                arr[index] = rhs;
                FloatingWindow::Ready{value: (value + rhs - old), previous : arr, index: (index + 1)%3}
            }
        }
    }
}
#[aoc(day1, part2, AccumulatorType)]
pub fn solve_part2_accumulator_type(input : &Vec<u32>) -> u32 {
    struct Helper {
        floating_window : FloatingWindow,
        prev : Option<u32>,
        count : u32,
    }
    input.iter()
        .fold(
            Helper{floating_window : FloatingWindow::default(), prev : None, count: 0}, 
            |Helper{floating_window, prev, count}, i| {
                let prev = floating_window.try_get();
                let floating_window = floating_window + *i;
                let curr = floating_window.try_get();
                let count = count + prev.zip(curr).and_then(|(prev,curr)| (curr > prev).then(|| 1)).unwrap_or(0);
                Helper{floating_window, prev, count}
            }
        ).count
}
