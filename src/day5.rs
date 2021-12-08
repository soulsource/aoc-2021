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

impl AxisAlignedLine {
    fn iter(&self) -> AxisAlignedLineIterator<&AxisAlignedLine> {
        AxisAlignedLineIterator{
            line : self,
            index : 0,
        }
    }
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

#[aoc(day5, part1, Plotted)]
pub fn solve_day5_part1_plotted(lines : &Vec<Line>) -> usize {
    use std::collections::hash_map::HashMap as Map;
    let hit_locations = lines.iter()
        .filter_map(|line| line.try_into().ok())
        .flat_map(|axis_aligned_line : AxisAlignedLine| axis_aligned_line.into_iter());
    let hit_counts = hit_locations.fold(Map::new(), |mut map, point| {
        map.entry(point).and_modify(|x| *x+=1).or_insert(1);
        map
    });
    hit_counts.iter().filter(|(_, value)| **value >= 2).count()
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

}
