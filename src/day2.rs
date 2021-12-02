use aoc_runner_derive::*;
use std::str::FromStr;
use std::error::Error;
use std::ops::Add;
use std::iter::Sum;

enum Direction {
    Forward,
    Up,
    Down,
    Back,
}

pub struct Command {
    direction : Direction,
    amount : u32
}

#[derive(Debug)]
pub struct CommandError(String);
impl std::fmt::Display for CommandError {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f,"Invalid Input: {}",self.0)
    }
}
impl Error for CommandError {}

struct Movement {
    x : i32,
    y : i32,
}

impl Add for Movement{
    type Output = Movement;
    fn add(self, other : Movement) -> Movement {
        Movement{x : self.x + other.x, y : self.y + other.y}
    }
}

impl Sum for Movement{
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Movement{x:0,y:0},|x,y| x+y)
    }
}

impl From<&Command> for Movement {
    fn from(c: &Command) -> Self {
        match c.direction {
            Direction::Forward => { Movement{x : c.amount as i32, y : 0} }
            Direction::Up => { Movement{x : 0, y : -(c.amount as i32)}}
            Direction::Down => { Movement{x :0, y : c.amount as i32}}
            Direction::Back => { Movement{x : -(c.amount as i32), y : 0} }
        }
    }
}

enum ReinterpretedCommand {
    Move(i32),
    ChangeAim(i32),
}

impl From<&Command> for ReinterpretedCommand {
    fn from(c: &Command) -> Self {
        match c.direction {
            Direction::Forward => { ReinterpretedCommand::Move(c.amount as i32)}
            Direction::Up => { ReinterpretedCommand::ChangeAim(-(c.amount as i32))}
            Direction::Down => { ReinterpretedCommand::ChangeAim(c.amount as i32)}
            Direction::Back => {ReinterpretedCommand::Move(-(c.amount as i32))}
        }
    }
}

struct MovementWithDirection {
    x : i32,
    y : i32,
    aim : i32,
}

impl Add<ReinterpretedCommand> for MovementWithDirection {
    type Output = Self;
    fn add(self, command : ReinterpretedCommand) -> Self {
        match command {
            ReinterpretedCommand::Move(distance) => { 
                MovementWithDirection{ 
                    x: self.x + distance, 
                    y: self.y + self.aim * distance, 
                    aim : self.aim,
                }
            }
            ReinterpretedCommand::ChangeAim(delta) => {
                MovementWithDirection {
                    x: self.x,
                    y: self.y,
                    aim : self.aim + delta,
                }
            }
        }
    }
}

impl FromStr for Command {
    type Err = CommandError;
    fn from_str(s: &str) -> Result<Self, CommandError> {
        let mut parts = s.split(" ");
        let direction = match parts.next() {
            Some("down") => { Some(Direction::Down) }
            Some("up") => { Some(Direction::Up) }
            Some("forward") => { Some(Direction::Forward) }
            Some("back") => { Some(Direction::Back) }
            None | Some(_) => { None } 
        };
        let amount = parts.next().map(|ss| FromStr::from_str(ss).ok()).flatten();
        direction.zip(amount)
            .map(|(direction, amount)| Command{direction,amount})
            .ok_or_else(||{ CommandError(String::from(s)) })
    }
}

#[aoc_generator(day2)]
pub fn input_generator<'c>(input : &'c str) -> Vec<Command>{ 
    input.lines().map(Command::from_str).map(Result::unwrap).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input : &Vec<Command>) -> i32 {
    let target : Movement = input.iter().map(From::from).sum();
    target.x*target.y
}

#[aoc(day2, part2)]
pub fn solve_part2(input : &Vec<Command>) -> i32 {
    let target = input.iter()
        .map(From::from)
        .fold(MovementWithDirection{x:0,y:0,aim:0},Add::add);
    target.x*target.y
}

