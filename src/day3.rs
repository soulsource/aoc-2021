use aoc_runner_derive::*;

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

trait ClonableIterator<'a>: Iterator<Item = &'a u32> + 'a {
    fn box_clone(&self) -> Box<dyn ClonableIterator<'a>>;
}

impl<'a, T: Iterator<Item = &'a u32> + Clone + 'a> ClonableIterator<'a> for T {
    fn box_clone(&self) -> Box<dyn ClonableIterator<'a>> {
        Box::new(self.clone())
    }
}
mod helper {
    impl<'a> Clone for Box<dyn super::ClonableIterator<'a>> {
        fn clone(&self) -> Self {
            (*self).box_clone()
        }
    }
}

fn count_ones_and_zeros<'a, T>(iterator : T , bit_from_right : usize) -> (usize, usize)
    where T : Iterator<Item=&'a u32>,
{
    iterator.fold((0,0), |(ones, zeros), value| if value & (1 <<bit_from_right) != 0 { (ones + 1, zeros) } else { (ones, zeros + 1) })
}

fn has_record_desired_bit(record : u32, most_common_bit_mask : u32, target : u32) -> bool {
    (record & most_common_bit_mask) == target
}

fn find_wanted_rating<'a, T : 'a,U>(iterator : T, max_len : usize, comparison : U) -> Option<&'a u32>
    where T : Iterator<Item=&'a u32>+Clone,
          U : Fn(usize,usize)->bool
{
    use std::ops::ControlFlow as Cf;
    let init : Box<dyn ClonableIterator<'a>> = Box::new(iterator);
    let result = (0..max_len).rev().try_fold(init,|iterator, bit_from_right| {
        let mask = 1 << bit_from_right;
        let (ones, zeros) = count_ones_and_zeros((iterator).clone(), bit_from_right);
        match ones + zeros {
            0 | 1 => { Cf::Break(iterator) }
            _ => {
                let target_bit_value = if comparison(ones, zeros) { mask } else { 0 };
                let filtered = iterator.filter(move |&&value| has_record_desired_bit(value,mask,target_bit_value));
                let boxed : Box<dyn ClonableIterator<'a>> = Box::new(filtered);
                Cf::Continue(boxed)
            }
        }
    });
    match result {
        Cf::Break(mut result) => { result.next() }
        Cf::Continue(remainder) => { remainder.take(2).fold(None,|x,y| x.xor(Some(y))) }
    }
}

type Solver<'a, T> = fn(T, usize, fn(usize, usize)->bool) -> Option<&'a u32>;

fn find_oxygen_rating<'a, T>(iterator : T, max_len : usize, solver : Solver<'a, T>) -> Option<&'a u32> 
    where T : Iterator<Item=&'a u32>+Clone,
{
    solver(iterator, max_len, |a,b| a>=b)
}

fn find_co2_rating<'a,T>(iterator : T, max_len : usize, solver : Solver<'a, T>) -> Option<&'a u32>
    where T : Iterator<Item=&'a u32>+Clone
{
    solver(iterator, max_len, |a,b| a<b)
}

#[aoc(day3, part2, CloneableIterator)]
pub fn solve_part2((input, max_len) : &(Vec<u32>, usize)) -> Option<u32> {
    let solver = find_wanted_rating;
    let oxygen_rating = find_oxygen_rating(input.iter(),*max_len, solver);
    oxygen_rating.and_then(|oxygen_rating| {
        find_co2_rating(input.iter(), *max_len,solver).map(|co2_rating| co2_rating * oxygen_rating)
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
    fn test_find_oxygen_rating_clonable_iterator() {
        let data = get_day3_processed_testdata();
        let result = find_oxygen_rating(data.0.iter(),data.1, find_wanted_rating);
        assert_eq!(result, Some(&23))
    }

    #[test]
    fn test_find_co2_rating_clonable_iterator() {
        let data = get_day3_processed_testdata();
        let result = find_co2_rating(data.0.iter(), data.1, find_wanted_rating);
        assert_eq!(result, Some(&10))
    }

    #[test]
    fn test_part2() {
        let result = solve_part2(&get_day3_processed_testdata());
        assert_eq!(result,Some(230))
    }
}
