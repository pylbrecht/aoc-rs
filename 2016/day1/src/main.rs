use std::str::FromStr;

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
}
