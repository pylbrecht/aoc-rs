use std::str::FromStr;
use std::ops::{Add, Mul};

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    direction: Direction,
    distance: usize,
}

#[derive(PartialEq, Debug)]
struct Point(isize, isize);

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Mul<usize> for Point {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self {
        Point(self.0 * rhs as isize, self.1 * rhs as isize)
    }
}

const NORTH: Point = Point(0, 1);
const EAST: Point = Point(1, 0);
const SOUTH: Point = Point(-1, 0);
const WEST: Point = Point(0, -1);

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .split(", ")
        .map(|inst| {
            let (direction, distance) = inst.split_at(1);
            let direction = match direction {
                "R" => Direction::Right,
                "L" => Direction::Left,
                _ => panic!("unknown direction"),
            };
            let distance = usize::from_str(distance).unwrap();
            Instruction {
                direction,
                distance,
            }
        })
        .collect()
}

fn main() { }

mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "R2, L3";
        let directions = vec![
            Instruction {
                direction: Direction::Right,
                distance: 2,
            },
            Instruction {
                direction: Direction::Left,
                distance: 3,
            },
        ];
        let parsed = parse_input(input);
        assert_eq!(parsed, directions);
    }

    #[test]
    fn test_travel() {
        let input = "R3, L2";
        let start = Point(0, 0);

        assert_eq!(travel(input, start), Point(-2, 3));
    }

    #[test]
    fn add_point() {
        let p1 = Point(0, 0);
        let p2 = Point(0, 1);
        assert_eq!(p1 + p2, Point(0, 1));
    }

    #[test]
    fn mul_point() {
        let p1 = Point(0, 0);
        let p2 = Point(0, 1);
        assert_eq!(p1 + p2, Point(0, 1));
    }
}
