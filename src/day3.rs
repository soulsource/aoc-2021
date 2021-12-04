use aoc_runner_derive::*;
use dyn_clone::{clone_trait_object, DynClone};

#[aoc_generator(day3)]
pub fn input_generator<'c>(input : &'c str) -> (Vec<u32>,usize){ 
    (input
        .lines()
        .map(|l| l.chars().rev()
            .enumerate()
            .fold(0, |count, (index,character)| match character {
                '1' => { count + (1 << index)}
                _ => { count }
            }
            )
        ).collect()
    , input.lines().map(|l| l.len()).max().unwrap_or(1))
}

#[aoc(day3, part1)]
pub fn solve_part1((input,max_len) : &(Vec<u32>,usize)) -> usize {
    let gamma_rate : usize= (0..*max_len).map(|index| 
        (index, input.iter().filter(|&&number| number & (1 << index) != 0).count() * 2 / input.len())
    ).map(|(bit, is_one)| is_one << bit).sum();
    let mask = (1 << max_len) - 1;
    let epsilon_rate = !gamma_rate & mask;
    gamma_rate * epsilon_rate
}

trait ClonableIterator: Iterator + DynClone {}
impl<I: Iterator + DynClone> ClonableIterator for I {}
clone_trait_object!(<T> ClonableIterator<Item = T>);

fn count_ones_and_zeros<'a, T>(iterator : T , bit_from_right : usize) -> (usize, usize)
    where T : Iterator<Item=&'a u32>,
{
    iterator.fold((0,0), |(ones, zeros), value| if value & (1 <<bit_from_right) != 0 { (ones + 1, zeros) } else { (ones, zeros + 1) })
}

fn has_record_desired_bit(record : u32, most_common_bit_mask : u32, target : u32) -> bool {
    (record & most_common_bit_mask) == target
}

fn find_wanted_rating<'a, T,U>(iterator : T, max_len : usize, comparison : U) -> Option<u32>
    where T : Iterator<Item=&'a u32>+Clone,
          U : Fn(usize,usize)->bool
{
    let init : Box<dyn ClonableIterator<Item=&'a u32>> = Box::new(iterator);
    let result = (0..max_len).rev().fold(init,|iterator, bit_from_right| {
        let mask = 1 << bit_from_right;
        let (ones, zeros) = count_ones_and_zeros(iterator.clone(), bit_from_right);
        println!("ones: {}, zeros: {}", ones, zeros);
        let target_bit_value = if comparison(ones, zeros) { mask } else { 0 };
        println!("Most common for bit_from_right {} : {}", bit_from_right, target_bit_value);
        let filtered = iterator.filter(move |&&value| has_record_desired_bit(value,mask,target_bit_value));
        let boxed : Box<dyn ClonableIterator<Item=&'a u32>> = Box::new(filtered);
        boxed
    });
    //the problem is formulated in a way that guarantees at least one result.
    result.take(2).fold(None,|x,y| x.xor(Some(*y)))
}

fn find_oxygen_rating<'a, T>(iterator : T, max_len : usize) -> Option<u32> 
    where T : Iterator<Item=&'a u32>+Clone
{
    find_wanted_rating(iterator, max_len, |a,b| a>=b)
}

fn find_co2_rating<'a,T>(iterator : T, max_len : usize) -> Option<u32>
    where T : Iterator<Item=&'a u32>+Clone
{
    find_wanted_rating(iterator, max_len, |a,b| (a+b == 1 && a>b) || (a+b !=1 && a<b))
}

#[aoc(day3, part2)]
pub fn solve_part2((input, max_len) : &(Vec<u32>, usize)) -> Option<u32> {
    let oxygen_rating = find_oxygen_rating(input.iter(),*max_len);
    oxygen_rating.and_then(|oxygen_rating| {
        find_co2_rating(input.iter(), *max_len).map(|co2_rating| co2_rating * oxygen_rating)
    })
}

#[cfg(test)]
mod day3_tests{
    use super::*;

    fn get_day3_string_testdata() -> &'static str {
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"
    }

    fn get_day3_processed_testdata() -> (Vec<u32>, usize) {
        (vec![0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001, 0b00010, 0b01010], 5)
    }

    #[test]
    fn test_generator() {
        assert_eq!(input_generator(get_day3_string_testdata()),get_day3_processed_testdata());
    }

    #[test]
    fn test_part1() {
        let result = solve_part1(&get_day3_processed_testdata());
        assert_eq!(result,198)
    }

    #[test]
    fn test_find_oxygen_rating() {
        let data = get_day3_processed_testdata();
        let result = find_oxygen_rating(data.0.iter(),data.1);
        assert_eq!(result, Some(23))
    }

    #[test]
    fn test_find_co2_rating() {
        let data = get_day3_processed_testdata();
        let result = find_co2_rating(data.0.iter(), data.1);
        assert_eq!(result, Some(10))
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(&get_day3_processed_testdata());
        assert_eq!(result,Some(230))
    }
}
