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

#[aoc(day1, part2, ItTookMeWayTooLongToRealizeThis)]
pub fn solve_part2_correct_solution(input : &Vec<u32>) -> usize {
    input.iter().skip(3).zip(input.iter()).filter(|(new, old)| new > old).count()
}

//--------------------------------------------------------------------------------------------
//let's not talk about the stuff below. It's just here for reference. Because it took me way too
//long to realize that part 2 is just part 1 with a different offset...

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

enum FloatingWindowFallible {
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

impl FloatingWindowFallible {
    fn try_get(&self) -> Option<u32> {
        match self {
            FloatingWindowFallible::Prewarming{..} => { None }
            FloatingWindowFallible::Ready{value, ..} => { Some(*value) }
        }
    }
}

impl Default for FloatingWindowFallible {
    fn default() -> Self {
        FloatingWindowFallible::Prewarming{value:0,previous:[0;2],index:0}
    }
}

impl Add<u32> for FloatingWindowFallible {
    type Output = FloatingWindowFallible;
    fn add(self, rhs : u32) -> FloatingWindowFallible {
        match self {
            FloatingWindowFallible::Prewarming{value, previous, index} if index < 2 => {
                let mut arr = previous;
                arr[index] = rhs;
                FloatingWindowFallible::Prewarming{value: value + rhs, previous : arr, index: index + 1}
            }
            FloatingWindowFallible::Prewarming{value, previous, ..} => {
                let arr = [previous[0], previous[1], rhs];
                FloatingWindowFallible::Ready{value: value + rhs, previous : arr, index: 0}
            }
            FloatingWindowFallible::Ready{value, previous, index} => {
                let mut arr = previous;
                let old = arr[index];
                arr[index] = rhs;
                FloatingWindowFallible::Ready{value: (value + rhs) - old, previous : arr, index: (index + 1)%3}
            }
        }
    }
}
#[aoc(day1, part2, AccumulatorTypeFallible)]
pub fn solve_part2_accumulator_type(input : &Vec<u32>) -> u32 {
    struct Helper {
        floating_window : FloatingWindowFallible,
        prev : Option<u32>,
        count : u32,
    }
    input.iter()
        .fold(
            Helper{floating_window : FloatingWindowFallible::default(), prev : None, count: 0}, 
            |Helper{floating_window, prev, count}, i| {
                let floating_window = floating_window + *i;
                let curr = floating_window.try_get();
                let count = count + prev.zip(curr).map(|(prev,curr)| if curr > prev {1} else {0}).unwrap_or(0);
                Helper{floating_window, prev: curr, count}
            }
        ).count
}

struct FloatingWindow {
    value : u32,
    previous : [u32;3],
    index : usize,
}

impl Add<u32> for FloatingWindow {
    type Output = FloatingWindow;
    fn add(self, rhs : u32) -> FloatingWindow {
        let mut arr = self.previous;
        let old = arr[self.index];
        arr[self.index] = rhs;
        FloatingWindow{value : (self.value + rhs) - old, previous : arr, index: (self.index + 1)%3}
    }
}

struct FloatingWindowInitializer {
    value : u32,
    previous : [u32;2],
    index : usize,
}

type FloatingWindowInitializerStep = std::ops::ControlFlow<FloatingWindow,FloatingWindowInitializer>;

impl FloatingWindowInitializer {
    fn add_step(self, input : &u32) -> FloatingWindowInitializerStep {
        if self.index == 2 {
            FloatingWindowInitializerStep::Break(FloatingWindow{ value : self.value + input, previous : [self.previous[0], self.previous[1], *input], index : 0})
        }
        else {
            let mut arr = self.previous;
            arr[self.index] = *input;
            FloatingWindowInitializerStep::Continue(FloatingWindowInitializer{ value: self.value + input, previous : arr, index : self.index + 1})
        }
    }
}

impl Default for FloatingWindowInitializer {
    fn default() -> Self {
        Self{value : 0, previous : [0;2], index : 0}
    }
}

#[aoc(day1, part2, AccumulatorTypeInfallible)]
pub fn solve_part2_accumulator_infallible(input : &Vec<u32>) -> Option<u32> {
    let floating_window = match input.iter()
        .try_fold(
            FloatingWindowInitializer::default(),
            FloatingWindowInitializer::add_step) { 
            FloatingWindowInitializerStep::Break(fw) => {Some(fw)}
            FloatingWindowInitializerStep::Continue(_) => {None}
        };
    floating_window.map(|f| {
        input.iter().skip(3).fold((f,0),|(f,c), u| {
            let old = f.value; 
            let f = f+*u; 
            if f.value > old { 
                (f,c+1) 
            } else { 
                (f,c) 
            }}).1
        }
    )
}


