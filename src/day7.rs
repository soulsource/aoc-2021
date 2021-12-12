use aoc_runner_derive::*;

//This problem has an interesting property that we can exploit. Between the positions of the crabs
//the function we check is linear.
//This means that we can immediately disregard all positions where no crab is at the start of the
//computation. Those points might be a solution, but only if the next crab's location is a solution
//as well.
//
//Another property that we can exploit is that the fuel costs of each crab are linear (except
//directly at its starting position) and extend infinitely. This means that the slope on both
//sides of the global minimum is always strictly positive. In other words: there are no local
//minima.
//
//We could now start searching for the minimum, for instance with a bisection solver.
//(Side note: Newton's method, the first that comes to mind, is not suitable here, as it assumes
//that the function behaves quadratic at the minimum.)
//
//However looking at the above constraints, and at the form of our problem, which basically is
//f(x) = sum(abs(x-pos(crab)), crabs)
//
//we can see another interesting detail: The slope of the derivative of this function is simply
//given by the number of crabs to the right minus the number of crabs to the left.
//
//f'(x) = sum(sign(x-pos(crab)), crabs)
//
//Now that's something, isn't it? 
//
//Long story short: we need the position of the first crab, at which there are more crabs left of and at
//the position than there are crabs right of it.
//There is a word for that condition: Median.

fn compute_fuel_costs_for_position_part_1(input : &Vec<usize>, position : usize) -> usize {
    input.iter().map(|c| {
        if *c < position {
            position - c
        }
        else {
            c - position
        }
    }).sum()
}

#[aoc_generator(day7)]
pub fn input_generator(input : &str) -> Result<Vec<usize>, std::num::ParseIntError> {
    input.split(",").try_fold(Vec::new(), |mut v,string| {
        v.push(string.trim().parse::<usize>()?);
        Ok(v)
    })
}

#[aoc(day7, part1, Median)]
pub fn solve_day7_part1_median(input : &Vec<usize>) -> usize {
    let mut input = input.clone();
    if input.len() == 0 {
        0
    }
    else {
        let midpoint = input.len()/2;
        let (_, &mut optimum_position, _) = input.select_nth_unstable(midpoint);
        compute_fuel_costs_for_position_part_1(&input, optimum_position)
    }
}

//Part 2 is more "difficult". Here the fuel function is no longer linear, but rather quadratic.
//This means all the nice properties we found in the first part are not applicable. On the plus
//side, now we have quadratic behaviour around the minimum, so Newton's method is on the table
//again.
//But before jumping to conclusions, let's analyze the maths.
//The fuel costs for a single crab are (|Δx|+1)*(0.5*|Δx|). Multiplied out we have 
//f(Δx) = 0.5 * (Δx²+|Δx|)
//f'(Δx) = Δx + 0.5*sign(Δx)
//The 0.5*sign(Δx) "offsets" all crabs parabolas by 0.5 to the left if viewed from the right,
//or by 0.5 to the right, if viewed from the left.
//
//Still, put in words, the global minimum is there, where the sum of all distances + half the count
//is the same on both sides.
#[cfg(test)]
mod tests {
    use super::*;
    fn get_day7_test_string() -> &'static str {
        "16,1,2,0,4,2,7,1,2,14"
    }
    #[test]
    fn test_day7_generator() {
        let input = input_generator(get_day7_test_string());
        assert_eq!(input, Ok(vec![16,1,2,0,4,2,7,1,2,14]))
    }
    #[test]
    fn test_day7_part1_median() {
        let input = input_generator(get_day7_test_string()).unwrap();
        let result = solve_day7_part1_median(&input);
        assert_eq!(result, 37);
    }
}
