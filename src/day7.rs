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
//(see https://de.wikipedia.org/wiki/Gaußsche_Summenformel - no, there's no English wiki page
//on this)
//
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
//
//Finding the exact point is not straightforward, but we can get a reasonably good estimate and
//an upper bound for the error.
//
//First, let's write the full function that has to be fulfilled:
//sum((x-pos[i]), i) + 0.5*sum(sign(x-pos[i]),i) = 0
//sum((x-pos[i]), i) + 0.5*sum(1, i where pos[i] < x) - 0.5*sum(1, i where pos[i] > x) = 0
//sum((x-pos[i]), i) = 0.5*(n_right - n_left)
//
//Now let's look at how those two values change when we change x by 1.
//The left hand side will change by the total number of crabs, as each term will change by 1.
//That is already the total range the right side can ever cover, as it can, at maximum change by
//the total number of crabs if all crabs were within a single unit. It is also bounded, meaning
//that it can never be larger than half the number of crabs (if all crabs are at the same side).
//
//In other words, if we find a solution to 
//sum((x-pos[i]),i) = 0
//we are already at most 1 unit away from the exact solution, and the exact solution is always in
//the direction that makes the imbalance 0.5*(n_right - n_left) smaller, or, put differently,
//towards the median.
//
//This information about the direction allows us to further reduce our error estimate. The latest
//after we have passed half the number of crabs, the sign of the imbalance 0.5*(n_right - n_left)
//must flip, meaning that the solution to sum((x-pos[i]),i) = 0 actually can never be more than 0.5
//units away from the true solution. (Assuming otherwise would need us to correct our result in
//both directions at the same time, what is absurd -> point proven.)
//
//We could have gotten the same reduction in error by exploiting the quantization of the problem.
//With full integer steps it is not possible to move "over" a crab by moving one step, at best we
//can move "on" or "off" a crab. This already limits the maximum change of the imbalance to half
//the number of crabs.
//
//Now knowing that the maximum error from solving 
//sum((x-pos[i]),i) = 0
//is 0.5 steps, we can limit our search range to two integer position values, namely the rounded
//solution, and the rounded solution ±1 in direction towards the median.
//
//The condition that fulfills 
//sum((x-pos[i]),i) = 0
//again is a well known quantity: The arithmetic mean.
//
//So, long story short: If we test the fuel consumption at the rounded arithmetic mean of the
//crab positions, the two neighbouring positions, and then pick the result with the lowest value,
//we have a solution.

fn compute_fuel_costs_for_position_part2(input : &Vec<usize>, position : usize) -> usize {
    input.iter()
        .map(|&x| if x > position { x-position } else { position - x })
        .map(|dx| dx*dx+dx).sum::<usize>()
        /2
}

fn rounded_arithmetic_mean(v : &Vec<usize>) -> usize {
    ((2*v.iter().sum::<usize>()) / v.len() + 1) / 2
}

#[derive(Debug)]
pub struct NoInputError;
impl std::fmt::Display for NoInputError {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "No input from generator. Could not compute fuel consumption.")
    }
}
impl std::error::Error for NoInputError {}

#[aoc(day7,part2,Mean)]
pub fn solve_part2_mean(input :&Vec<usize>) -> Result<usize, NoInputError> {
    //computing the fuel consumption a third time is probably cheaper than finding the median.
    //Let's just try three values, take the best one.
    let mean = rounded_arithmetic_mean(input);
    (mean-1..=mean+1).map(|position| compute_fuel_costs_for_position_part2(input,position)).min()
        .ok_or(NoInputError)
}

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
    #[test]
    fn test_day7_part2_average() {
        let input = input_generator(get_day7_test_string()).unwrap();
        let result = solve_part2_mean(&input).unwrap();
        assert_eq!(result,168)
     }
}
