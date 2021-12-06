use aoc_runner_derive::*;
use std::fmt::{Display,Formatter};
use std::borrow::Borrow;
use std::error::Error;
use bingo_internals::*;

mod bingo_internals {
    use std::fmt::{Display, Formatter};
    use std::error::Error;
    use std::borrow::Borrow;

    ///Internal representation of a bingo game field. Basically a list of numbers with encountered
    ///state mapping to their row/column, and a list of integers representing how often a value
    ///in a given row/column has been hit.
    ///This representation was chosen so unfinished games can be continued (not encountered in aoc, but
    ///I'm doing this as programming training after all...).
    #[derive(Debug,Clone)]
    struct UnfinishedBingoCard{
        rows : [u8;5],
        columns : [u8;5],
        ///the values in the bingo game. Those could be stored in a HashMap, but afaik Rust's
        ///hashmap only becomes faster than just iterating over a list if there are dozens of
        ///entries.
        contained_numbers : [BingoNumberState;25]
    }

    #[derive(Debug,Clone,Copy)]
    struct WonBingoCard{
        score : usize
    }

    impl WonBingoCard {
        pub fn get_score(&self) -> usize {
            self.score
        }
    }

    impl UnfinishedBingoCard {
        ///Creates a new bingo card from a stream of numbers. The stream must yield 25 numbers. To
        ///allow chaining of card generation, the stream's iterator is returned along with the
        ///result.
        fn new<'a, T, Q>(mut numbers : T) -> (Result<Self, InvalidBingoFieldError>, T) 
            where T : Iterator<Item=Q>,
                  Q : Borrow<u8>
        {
            let numbers_to_parse = numbers.by_ref().take(25);
            let parsed_values = numbers_to_parse.enumerate().map(|(idx, value)| BingoNumberState::NotCrossed( BingoNumber{ value : *(value.borrow()), row : (idx/5) as u8, column : (idx%5) as u8 })).collect::<Vec<BingoNumberState>>();

            match parsed_values.try_into() {
                Ok(array) => {
                    if Self::validate_contained_numbers(&array) {
                        (Ok(UnfinishedBingoCard { rows : [0;5], columns : [0;5], contained_numbers : array }), numbers)
                    }
                    else {
                        (Err(InvalidBingoFieldError::DuplicateValue),numbers)
                    }
                }
                Err(unparsed) => {
                    if unparsed.len() == 0 {
                        (Err(InvalidBingoFieldError::NoData), numbers)
                    }
                    else {
                        (Err(InvalidBingoFieldError::NotEnoughData), numbers)
                    }
                }
            }
        }
        fn cross_number(self, number : u8) -> BingoCard {
            //Sorry for this. Arrays in Rust aren't really functional-style friendly...
            //Yes, this is coding using side effects, but the API is how it is...
            let mut row_and_column = None;
            let new_state = self.contained_numbers.map(|number_state| match number_state {
                BingoNumberState::NotCrossed(num) if num.value == number  => {
                    row_and_column = Some((num.row, num.column));
                    BingoNumberState::Crossed(num)
                }
                BingoNumberState::Crossed(_) | BingoNumberState::NotCrossed(_) => { number_state }
            });
            if let Some((row, column)) = row_and_column {
                let new_row_hit_count = self.rows[row as usize] + 1;
                let new_col_hit_count = self.columns[column as usize] + 1;
                if new_col_hit_count == 5 || new_row_hit_count == 5 {
                    BingoCard::Won(WonBingoCard { score : Self::calc_current_score(new_state) })
                }
                else {
                    let mut rows = self.rows;
                    rows[row as usize] = new_row_hit_count;
                    let mut columns = self.columns;
                    columns[column as usize] = new_col_hit_count;
                    BingoCard::Unfinished(Self{rows, columns, contained_numbers : new_state})
                }
            }
            else {
                BingoCard::Unfinished(Self{rows : self.rows, columns : self.columns, contained_numbers : new_state})
            }
        }
        fn calc_current_score(numbers : [BingoNumberState;25]) -> usize {
            numbers.iter().map(|x| match x {
                BingoNumberState::NotCrossed(BingoNumber{ value, ..}) => { (*value) as usize }
                BingoNumberState::Crossed(BingoNumber{ .. }) => { 0usize }
            }).sum()
        }
        fn validate_contained_numbers(numbers : &[BingoNumberState;25]) -> bool {
            let bare_values = numbers.iter().map(|x| match x { 
                BingoNumberState::Crossed(BingoNumber{ value, ..}) => {value}
                BingoNumberState::NotCrossed(BingoNumber{ value, ..}) => {value}
            });
            bare_values.clone().enumerate().fold(true,|prev, (index,number)| {
                prev && bare_values.clone().skip(index+1).fold(true, |prev, other_number| prev && other_number != number)
            })
        }
    }

    #[derive(Debug,Clone)]
    enum BingoCard {
        Unfinished(UnfinishedBingoCard),
        Won(WonBingoCard),
    }

    impl BingoCard {
        fn new<'a, T, Q>(numbers : T) -> (Result<Self, InvalidBingoFieldError>, T) 
            where T : Iterator<Item=Q>,
                  Q : Borrow<u8>
        {
            let (result, iterator) = UnfinishedBingoCard::new(numbers);
            (result.map(|x| BingoCard::Unfinished(x)),iterator)
        }
        fn get_score(&self) -> Option<usize> {
            match self {
                BingoCard::Won(won_bingo_card) => { Some(won_bingo_card.get_score()) }
                BingoCard::Unfinished(_) => { None }
            }
        }
        fn cross_number(self, value : u8) -> Self {
            match self {
                BingoCard::Won(_) => { self }
                BingoCard::Unfinished(unfinished_card) => { unfinished_card.cross_number(value) }
            }
        }
    }
    #[derive(Debug)]
    pub enum InvalidBingoFieldError {
        NoData,
        NotEnoughData,
        DuplicateValue
    }

    impl Display for InvalidBingoFieldError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error>{
            match self {
                InvalidBingoFieldError::NotEnoughData => {
                    write!(f, "Not enough data provided to create a bingo field.")
                }
                InvalidBingoFieldError::DuplicateValue => {
                    write!(f, "Bingo field input contained duplicate values. That's against the rules.")
                }
                InvalidBingoFieldError::NoData => {
                    write!(f, "Bingo field cannot be created with empty data.")
                }
            }
        }
    }
    impl Error for InvalidBingoFieldError {}

    #[derive(Debug, Clone)]
    struct BingoNumber{
        value : u8,
        row : u8,
        column : u8,
    }

    #[derive(Debug, Clone)]
    enum BingoNumberState{
        NotCrossed(BingoNumber),
        Crossed(BingoNumber)
    }

    ///Representation of an ongoing bingo game. Actually just a list of cards.
    #[derive(Debug, Clone)]
    pub struct BingoGame{
        cards : Vec<BingoCard>,
    }

    #[derive(Debug)]
    pub enum BingoGameCreationError{
        DataNotMultipleOfTwentyFive,
        NoDataSupplied,
        DuplicateValueInInput,
    }
    impl Display for BingoGameCreationError {
        fn fmt(&self, f : &mut Formatter<'_>) -> Result<(), std::fmt::Error>{
            match self {
                BingoGameCreationError::DataNotMultipleOfTwentyFive => {
                    write!(f,"The input data stopped during the creation of a Bingo card or before the first card was created")
                }
                BingoGameCreationError::NoDataSupplied => {
                    write!(f,"No input data for bingo game creation")
                }
                BingoGameCreationError::DuplicateValueInInput => {
                    write!(f,"There was a duplicate value in the input of a bingo card")
                }
            }
        }
    }
    impl Error for BingoGameCreationError {}

    impl BingoGame {
        pub fn new<T, Q>(input : T) -> Result<Self, BingoGameCreationError> 
            where T : Iterator<Item=Q>,
                  Q : Borrow<u8>

        {
            use std::ops::ControlFlow as Cf;
            let cards = std::iter::repeat(()).scan(input,|input,_| {
                let card = BingoCard::new(input);
                Some(card.0)
            }).try_fold(Vec::new(), |vector, card| {
                match card { 
                    Ok(card) => {
                        Cf::Continue(vector.into_iter().chain(std::iter::once(card)).collect())
                    }
                    Err(card_creation_error) => {
                        Cf::Break((vector, card_creation_error))
                    }
                }
            });
            match cards {
                Cf::Continue(_) => { unreachable!() } //only needed because break_value() method is unstable...
                Cf::Break((_, InvalidBingoFieldError::NotEnoughData)) => {
                    Err(BingoGameCreationError::DataNotMultipleOfTwentyFive)
                }
                Cf::Break((_, InvalidBingoFieldError::DuplicateValue)) => {
                    Err(BingoGameCreationError::DuplicateValueInInput)
                }
                Cf::Break((cards, InvalidBingoFieldError::NoData)) => {
                    if cards.len() == 0 {
                        Err(BingoGameCreationError::NoDataSupplied)
                    }
                    else {
                        Ok(BingoGame{cards})
                    }
                }
            }
        }
        pub fn cross_number(self, value :u8) -> Self {
            Self{ cards : self.cards.into_iter().map(|x| x.cross_number(value)).collect() }
        }

        pub fn get_winner_cards_scores_ascending(&self) -> impl Iterator<Item=WinnerNumberAndCardScore>+Clone+'_ {
            self.cards.iter().enumerate().filter_map(|(winner_number, card)| card.get_score().map(|card_score| WinnerNumberAndCardScore { winner_number, card_score}))
        }
        pub fn get_number_of_cards_in_game(&self) -> usize {
            self.cards.len()
        }
    }

    pub struct WinnerNumberAndCardScore {
        pub winner_number : usize,
        pub card_score : usize,
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use super::super::*;
        use super::super::tests::*;


        #[test]
        fn test_day4_parse_input_numbers() {
            let parsed = parse_input_numbers(get_day_4_string_testdata());
            assert_eq!(parsed, get_day_4_parsed_input_numbers())
        }
        
        #[test]
        fn test_day4_parse_bingo_game() {
            let game = parse_bingo_game(get_day_4_string_testdata());
            assert!(game.is_ok());
            let cards = game.unwrap().cards;
            assert!(cards.len() == 3);
            assert!((0..3).all(|index| match &cards[index] {
                BingoCard::Won(_) => { unreachable!() }
                BingoCard::Unfinished(c) => { 
                    c.contained_numbers.iter().map(|x| match x {
                        BingoNumberState::Crossed(_) => { unreachable!() }
                        BingoNumberState::NotCrossed( BingoNumber{ value , .. }) => { value }
                    }).eq(get_day4_parsed_field_numbers(index).iter())
                }
            }));
            assert_eq!(match &cards[1] {
                BingoCard::Won(_) => { unreachable!() }
                BingoCard::Unfinished(c) => {
                    c.contained_numbers.iter().find_map(|x| {
                        if let BingoNumberState::NotCrossed( BingoNumber{value, column, row}) = x{
                            if *value == 17{ Some((row, column)) } else { None}
                        }
                        else {
                            None
                        }
                    }).unwrap()
                }
            }, (&1,&3))
        }   
        
        #[test]
        fn test_day4_cross_number_in_bingo_card() {
            println!("This test relies on test_day3_parse_bingo_game passing. If both fail, fix test_day3_parse_bingo_game first!");
            let game = parse_bingo_game(get_day_4_string_testdata());
            let mut game = game.unwrap();
            assert!(match &game.cards[0] {
                BingoCard::Won(_) => { unreachable!() }
                BingoCard::Unfinished(c) => {
                    c.contained_numbers.iter().all(|x| match x {
                        BingoNumberState::Crossed(_) => {false}
                        BingoNumberState::NotCrossed(_) => {true} 
                    })
                }
            });
            let new_card = game.cards.remove(0).cross_number(150);
            assert!(match &new_card {
                BingoCard::Won(_) => { unreachable!() }
                BingoCard::Unfinished(c) => {
                    c.contained_numbers.iter().all(|x| match x {
                        BingoNumberState::Crossed(_) => {false}
                        BingoNumberState::NotCrossed(_) => {true} 
                    })
                }
            });
            let new_card = new_card.cross_number(24);
            assert_eq!(match &new_card {
                BingoCard::Won(_) => { unreachable!() }
                BingoCard::Unfinished(c) => { 
                    c.contained_numbers.iter().filter(|x| match x {
                        BingoNumberState::Crossed(_) => {false}
                        BingoNumberState::NotCrossed(_) => {true}
                    }).count()
                }
            }, 24);
            let record = match &new_card {
                BingoCard::Won(_) => { unreachable!() }
                BingoCard::Unfinished(c) => {
                    c.contained_numbers.iter().find(|x| match x {
                        BingoNumberState::Crossed(_) => {true}
                        BingoNumberState::NotCrossed(_) => {false}
                    }).unwrap()
                }
            };
            match record {
                BingoNumberState::Crossed(BingoNumber{value, row, column}) => {
                    assert_eq!(value, &24);
                    assert_eq!(row, &1);
                    assert_eq!(column, &4);
                }
                BingoNumberState::NotCrossed(_) => { unreachable!() }
            };
        }

        #[test]
        fn test_day5_win_card() {
            println!("This test relies on test_day3_parse_bingo_game passing. If both fail, fix test_day3_parse_bingo_game first!");
            let game = parse_bingo_game(get_day_4_string_testdata()).unwrap();
            assert_eq!(game.get_winner_cards_scores_ascending().count(), 0);
            let game = game.cross_number(2);
            assert_eq!(game.get_winner_cards_scores_ascending().count(), 0);
            let game = game.cross_number(17);
            assert_eq!(game.get_winner_cards_scores_ascending().count(), 0);
            let game = game.cross_number(25);
            assert_eq!(game.get_winner_cards_scores_ascending().count(), 0);
            let game = game.cross_number(7);
            assert_eq!(game.get_winner_cards_scores_ascending().count(), 0);
            let game = game.cross_number(24);
            assert_eq!(game.get_winner_cards_scores_ascending().count(), 0);
            let game = game.cross_number(12);
            assert_eq!(game.get_winner_cards_scores_ascending().map(|WinnerNumberAndCardScore { card_score,.. }| card_score).sum::<usize>(),237);
        }
    }
}

#[derive(Debug)]
pub struct GameAndInput {
    game : BingoGame,
    input : Vec<u8>,
}

#[derive(Debug)]
pub struct WinnerNumberAndScore {
    winner_number : usize,
    score : usize,
}

#[derive(Debug)]
pub enum BingoGameSolutionError {
    InsufficientInput { current_game_state : BingoGame },
    Tie { winners : Vec<WinnerNumberAndScore> }
}

impl Display for BingoGameSolutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error>{
        match self {
            BingoGameSolutionError::InsufficientInput{..} => {
                write!(f, "The game is not completed. The returned error contains the current state, you can continue the game if you have more input")
            }
            BingoGameSolutionError::Tie{ winners } => {
                write!(f, "There has been a tie. The following players finished the same turn with score:")?;
                winners.iter().try_for_each(|i| write!(f, " {} {}", i.winner_number, i.score))
            }
        }
    }
}
impl Error for BingoGameSolutionError {}

fn parse_input_numbers(input : &str) -> Vec<u8> {
    use std::str::FromStr;
    input.lines().take(1).flat_map(|line| line.split(","))
        .filter_map(|int_as_string| u8::from_str(int_as_string).ok()).collect()
}

fn parse_bingo_game(input : &str) -> Result<BingoGame, BingoGameCreationError> {
    use std::str::FromStr;
    BingoGame::new(input.lines().skip(1).filter(|line| line.len() > 0).flat_map(|line| {
        line.split(" ").filter_map(|int_as_string| u8::from_str(int_as_string).ok())
    }))
}


#[aoc_generator(day4)]
pub fn input_generator<'c>(input : &'c str) -> Result<GameAndInput, BingoGameCreationError>{ 
    Ok(GameAndInput{
        game : parse_bingo_game(input)?,
        input : parse_input_numbers(input)
    })
}

#[aoc(day4, part1)]
pub fn solve_part1(input : &GameAndInput) -> Result<usize, BingoGameSolutionError> {
    use std::ops::ControlFlow as Cf;
    let result = input.input.iter().try_fold(input.game.clone(),|game, value| {
        let game = game.cross_number(*value);
        let winners = game.get_winner_cards_scores_ascending()
            .map(|WinnerNumberAndCardScore {winner_number, card_score}| WinnerNumberAndScore{winner_number, score : card_score * (*value as usize)})
            .collect::<Vec<_>>();
        match winners.len() {
            0 => { Cf::Continue(game) }
            1 => { Cf::Break(Ok(winners[0].score)) }
            _ => { Cf::Break(Err(BingoGameSolutionError::Tie{ winners : winners })) }
        }
    });
    match result {
        Cf::Continue(g) => { Err(BingoGameSolutionError::InsufficientInput{current_game_state : g})}
        Cf::Break(score) => { score }
    }
}

fn run_game_until_only_one_player_left<T,Q>(game : BingoGame, mut input : T) -> Result<(BingoGame, T), BingoGameSolutionError>
    where T : Iterator<Item=Q>,
          Q : Borrow<u8>
{
    use std::ops::ControlFlow as Cf;
    let card_count = game.get_number_of_cards_in_game();
    let game_before_last_card_finishes = input.by_ref().try_fold(game,|game, value| {
        let game = game.cross_number(*(value.borrow()));
        let winners = game.get_winner_cards_scores_ascending().count();
        match winners {
            x if x < card_count-1 => { Cf::Continue(game) }
            x if x == card_count-1 => { Cf::Break(Ok(game)) }
            _ => { 
                Cf::Break(Err(BingoGameSolutionError::Tie{ 
                    winners : game.get_winner_cards_scores_ascending().map( |WinnerNumberAndCardScore {winner_number, card_score}| {
                        WinnerNumberAndScore{
                            winner_number, 
                            score : card_score * (*(value.borrow()) as usize)}
                        }).collect::<Vec<_>>() 
                }))
            }
        }
    });
    match game_before_last_card_finishes {
        Cf::Continue(g) => { Err(BingoGameSolutionError::InsufficientInput{current_game_state : g})}
        Cf::Break(g) => { g.map(|g| (g,input)) }
    }
}

fn find_first_player_that_hasnt_won(game : &BingoGame) -> Option<usize> {
    game.get_winner_cards_scores_ascending()
        .cycle()
        .take(game.get_number_of_cards_in_game())
        .enumerate()
        .find_map(|(index, WinnerNumberAndCardScore{winner_number, ..})| {
            if index != winner_number { Some(index) } else { None }
        })
}

#[aoc(day4, part2)]
pub fn solve_part2(input : &GameAndInput) -> Result<usize, BingoGameSolutionError> {
    use std::ops::ControlFlow as Cf;
    let (game_before_last_card_finishes, mut input_iterator) = run_game_until_only_one_player_left(input.game.clone(), input.input.iter())?;
    //we can just unwrap here, as the case that there are no players left has been handled by the ?
    //operator on the previous line
    let player_that_hasnt_won = find_first_player_that_hasnt_won(&game_before_last_card_finishes).unwrap(); 
    let result = input_iterator.try_fold(game_before_last_card_finishes, | game, value | {
        let game = game.cross_number(*value);
        let last_player = game.get_winner_cards_scores_ascending().find_map(|WinnerNumberAndCardScore{winner_number, card_score}| {
            if winner_number == player_that_hasnt_won {
                Some(card_score * (*value as usize))
            }
            else {
                None
            }
        });
        match last_player {
            Some(score) => { Cf::Break(score) }
            None => { Cf::Continue(game) }
        }
    });
    match result {
        Cf::Continue(current_game_state) => { Err(BingoGameSolutionError::InsufficientInput{current_game_state}) }
        Cf::Break(score) => { Ok(score) }
    }
}


#[cfg(test)]
pub mod tests{
    use super::*;
    pub fn get_day_4_string_testdata() -> &'static str {
r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#
    }

    pub fn get_day_4_parsed_input_numbers() -> &'static [u8] {
        &[7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1]
    }

    pub fn get_day4_parsed_field_numbers(index : usize) -> &'static [u8;25] {
        use std::cmp::min;
        let index = min(index,2);
        &[[22,13,17,11,0,8,2,23,4,24,21,9,14,16,7,6,10,3,18,5,1,12,20,15,19],
        [3,15,0,2,22,9,18,13,17,5,19,8,7,25,23,20,11,10,24,4,14,21,16,12,6],
        [14,21,17,24,4,10,16,15,9,19,18,8,23,26,20,22,11,13,6,5,2,0,12,3,7],][index]
    }

    #[test]
    pub fn test_day4_solve_part1() {
        let testdata = input_generator(get_day_4_string_testdata()).unwrap();
        assert_eq!(solve_part1(&testdata).unwrap(), 4512)
    }

    #[test]
    pub fn test_day4_solve_part2() {
        let testdata = input_generator(get_day_4_string_testdata()).unwrap();
        assert_eq!(solve_part2(&testdata).unwrap(), 1924)
    }
}
