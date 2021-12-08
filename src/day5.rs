use aoc_runner_derive::*;
use std::fmt::{Display, Formatter};
use std::error::Error;
use std::convert::TryFrom;
use std::borrow::Borrow;

#[derive(Eq, PartialEq, Hash, Debug)]
struct Point {
    x : usize,
    y : usize,
}

#[derive(Eq, Debug)]
pub struct Line {
    endpoints : [Point;2]
}

impl PartialEq for Line {
    fn eq(&self, other : &Self) -> bool {
        (
            self.endpoints[0] == other.endpoints[0]
            && self.endpoints[1] == other.endpoints[1]
        )
        || (
            self.endpoints[1] == other.endpoints[0]
            && self.endpoints[0] == other.endpoints[1]
        )
    }
}

#[derive(Debug)]
enum AxisAlignedLine {
    Horizontal {
        start : Point,
        length : usize
    },
    Vertical {
        start: Point,
        length : usize
    }
}

impl std::iter::IntoIterator for AxisAlignedLine {
    type Item = Point;
    type IntoIter = AxisAlignedLineIterator<AxisAlignedLine>;
    fn into_iter(self) -> AxisAlignedLineIterator<AxisAlignedLine> {
        AxisAlignedLineIterator {
            line :self,
            index : 0,
        }
    }
}

#[derive(Debug, Clone)]
struct AxisAlignedLineIterator<T>
    where T : Borrow<AxisAlignedLine>
{
    line : T,
    index : usize
}

impl<T> Iterator for AxisAlignedLineIterator<T> 
    where T : Borrow<AxisAlignedLine>
{
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
        let index = self.index;
        match self.line.borrow() {
            AxisAlignedLine::Horizontal{ start, length } => {
                if index <= *length {
                    self.index = index + 1;
                    Some(Point{ x : start.x + index, y: start.y})
                } else {
                    None
                }
            }
            AxisAlignedLine::Vertical{ start, length } => {
                if index <= *length {
                    self.index = index + 1;
                    Some(Point{x:start.x, y: start.y + index})
                }
                else {
                    None
                }
            }
        }
    }
}

#[derive(Debug)]
struct NotAxisAlignedError;
impl Display for NotAxisAlignedError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Line not axis aligned, cannot be converted to asix aligned line")
    }
}
impl Error for NotAxisAlignedError {}

impl TryFrom<&Line> for AxisAlignedLine {
    type Error = NotAxisAlignedError;
    fn try_from(line : &Line) -> Result<Self,NotAxisAlignedError> {
        use std::cmp::{min,max};
        if line.endpoints[0].y == line.endpoints[1].y {
            let smaller_x = min(line.endpoints[0].x, line.endpoints[1].x);
            let larger_x = max(line.endpoints[0].x, line.endpoints[1].x);
            Ok(Self::Horizontal{
                start : Point { x: smaller_x, y: line.endpoints[0].y },
                length : larger_x - smaller_x
            })
        }
        else if line.endpoints[0].x == line.endpoints[1].x {
            let smaller_y = min(line.endpoints[0].y, line.endpoints[1].y);
            let larger_y = max(line.endpoints[0].y, line.endpoints[1].y);
            Ok(Self::Vertical {
                start : Point { x: line.endpoints[0].x, y: smaller_y },
                length : larger_y - smaller_y
            })
        }
        else {
            Err(NotAxisAlignedError)
        }
    }
}

#[derive(Debug)]
struct DiagonalLine {
    start : Point,
    length : usize,
    y_negative : bool,
}

#[derive(Debug)]
enum AlignedLine {
    AxisAligned(AxisAlignedLine),
    Diagonal(DiagonalLine), 
}

#[derive(Debug)]
struct NotAlignedError;
impl Display for NotAlignedError {
    fn fmt(&self, f : &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Line neither axis aligned nor 45° diagonal, cannot be converted to AlignedLine")
    }
}
impl Error for NotAlignedError {}

impl TryFrom<&Line> for AlignedLine {
    type Error = NotAlignedError;
    fn try_from(line : &Line) -> Result<Self, NotAlignedError> {
        use std::cmp::{min,max};
        if let Ok(axis_aligned) = line.try_into() {
            Ok(Self::AxisAligned(axis_aligned))
        }
        else {
            let min_x = min(line.endpoints[0].x, line.endpoints[1].x);
            let max_x = max(line.endpoints[0].x, line.endpoints[1].x);
            let min_y = min(line.endpoints[0].y, line.endpoints[1].y);
            let max_y = max(line.endpoints[0].y, line.endpoints[1].y);
            if max_y - min_y == max_x - min_x {
                let start_index = if line.endpoints[0].x < line.endpoints[1].x { 0 } else { 1 };
                Ok(Self::Diagonal(DiagonalLine {
                    start : Point { x: line.endpoints[start_index].x, y: line.endpoints[start_index].y},
                    length : max_y-min_y,
                    y_negative : line.endpoints[1-start_index].y < line.endpoints[start_index].y,
                }))
            }
            else {
                Err(NotAlignedError)
            }
        }
    }
}

impl std::iter::IntoIterator for  AlignedLine {
    type Item=Point;
    type IntoIter = AlignedLineIterator<DiagonalLine,AxisAlignedLine>;
    fn into_iter(self) -> AlignedLineIterator<DiagonalLine, AxisAlignedLine> {
        match self {
            AlignedLine::AxisAligned(axis_aligned_line) => {
                AlignedLineIterator::Aligned(axis_aligned_line.into_iter())
            }
            AlignedLine::Diagonal(line) => {
                AlignedLineIterator::Diagonal { line, index : 0 }
            }
        }
    }
}

enum AlignedLineIterator<T, Q>
    where T: Borrow<DiagonalLine>,
          Q: Borrow<AxisAlignedLine>
{
    Aligned(AxisAlignedLineIterator<Q>),
    Diagonal {
        line : T,
        index : usize,
    }
}

impl<T,Q> Iterator for AlignedLineIterator<T,Q> 
    where T: Borrow<DiagonalLine>,
          Q: Borrow<AxisAlignedLine>
{
    type Item = Point;
    fn next(&mut self) -> Option<Point> {
        match self {
            AlignedLineIterator::Aligned(aligned_iterator) => { aligned_iterator.next() }
            AlignedLineIterator::Diagonal { line, index : self_index } => {
                let index = *self_index;
                let line = (*line).borrow();
                if index <= line.length {
                    *self_index += 1;
                    Some(Point {
                        x: line.start.x + index,
                        y: if line.y_negative { line.start.y - index } else { line.start.y + index }
                    })
                }
                else {
                    None
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum LineParsingError {
    ParseIntError(std::num::ParseIntError),
    MalformedLine(String),
    MalformedPoint(String),
}

impl From<std::num::ParseIntError> for LineParsingError {
    fn from(e : std::num::ParseIntError) -> Self {
        LineParsingError::ParseIntError(e)
    }
}

impl Display for LineParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            LineParsingError::ParseIntError(e) => { e.fmt(f) }
            LineParsingError::MalformedLine(s) => {
                write!(f, "Line is malformed: {}", s)
            }
            LineParsingError::MalformedPoint(s) => {
                write!(f, "Point is malformed: {}", s)
            }
        }
    }
}

impl Error for LineParsingError {}

fn parse_point_from_string(string : &str) -> Result<Point, LineParsingError> {
    if let Some((x, y)) = string.split_once(",") {
        x.trim().parse().and_then(|x| y.trim().parse().and_then(|y| Ok(Point {x,y}))).map_err(|e| e.into())
    }
    else {
        Err(LineParsingError::MalformedPoint(string.into()))
    }
}

fn parse_line_from_string(string : &str) -> Result<Line, LineParsingError> {
    let (parsed_points,index) = string.split("->").map(|string| parse_point_from_string(string))
        .try_fold(([None, None],0),|(mut result, index), point| {
            if index < 2 {
                result[index] = Some(point?);
                let index = index + 1;
                Ok((result, index))
            }
            else {
                Err(LineParsingError::MalformedLine(string.into()))
            }
        })?;
    if index < 2 {
        Err(LineParsingError::MalformedLine(string.into()))
    }
    else {
        Ok(Line {
            endpoints : parsed_points.map(|x| x.unwrap()),
        })
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input : &str) -> Result<Vec<Line>, LineParsingError> {
    input.lines()
        .map(|string| parse_line_from_string(string))
        .try_fold(Vec::new(),|mut result, line| {
            line.map(|line| {
                result.push(line);
                result
            })
        }
        )
}

fn solve_day5_with_type<'a, T>(lines : &'a Vec<Line>) -> usize
    where T : IntoIterator<Item=Point>+TryFrom<&'a Line>,
{
    use std::collections::hash_map::HashMap as Map;
    let hit_locations = lines.iter()
        .filter_map(|line| line.try_into().ok())
        .flat_map(|line : T| line.into_iter());
    let hit_counts = hit_locations.fold(Map::new(), |mut map, point| {
        map.entry(point).and_modify(|x| *x+=1).or_insert(1);
        map
    });
    hit_counts.iter().filter(|(_, value)| **value >= 2).count()

}

#[aoc(day5, part1, Plotted)]
pub fn solve_day5_part1_plotted(lines : &Vec<Line>) -> usize {
    solve_day5_with_type::<AxisAlignedLine>(lines)
}

#[aoc(day5, part2, Plotted)]
pub fn solve_day5_part2_plotted(lines : &Vec<Line>) -> usize {
    solve_day5_with_type::<AlignedLine>(lines)
}

#[cfg(test)]
mod day5_tests{
    use super::*;

    fn get_day5_string_testdata() -> &'static str {
r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#
    }

    fn get_day5_parsed_testdata() -> Vec<Line> {
        vec!{
            Line{endpoints:[Point{x:0,y:9},Point{x:5,y:9}]},
            Line{endpoints:[Point{x:8,y:0},Point{x:0,y:8}]},
            Line{endpoints:[Point{x:9,y:4},Point{x:3,y:4}]},
            Line{endpoints:[Point{x:2,y:2},Point{x:2,y:1}]},
            Line{endpoints:[Point{x:7,y:0},Point{x:7,y:4}]},
            Line{endpoints:[Point{x:6,y:4},Point{x:2,y:0}]},
            Line{endpoints:[Point{x:0,y:9},Point{x:2,y:9}]},
            Line{endpoints:[Point{x:3,y:4},Point{x:1,y:4}]},
            Line{endpoints:[Point{x:0,y:0},Point{x:8,y:8}]},
            Line{endpoints:[Point{x:5,y:5},Point{x:8,y:2}]},
        }
    }
    
    #[test]
    fn test_day5_input_generator() {
        let parse_result = input_generator(get_day5_string_testdata()).unwrap();
        assert_eq!(parse_result, get_day5_parsed_testdata())
    }

    #[test]
    fn test_day5_part1_solution_plotted() {
        assert_eq!(solve_day5_part1_plotted(&get_day5_parsed_testdata()), 5)
    }

    #[test]
    fn test_day5_part2_solution_plotted() {
        assert_eq!(solve_day5_part2_plotted(&get_day5_parsed_testdata()), 12)
    }

}
