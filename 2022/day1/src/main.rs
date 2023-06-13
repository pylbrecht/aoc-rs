use std::fs;
use std::str::FromStr;
use std::result::Result;
use std::error::Error;

#[derive(Debug, PartialEq)]
struct Elf {
    calories: usize,
}

#[derive(Debug)]
struct ParseElfError;

impl FromStr for Elf {
    type Err = ParseElfError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let calories: usize = s.trim_end_matches("\n").split("\n").map(|calories| usize::from_str(calories).unwrap()).sum();
        Ok(Elf { calories })
    }
}

impl Elf {
    fn new() -> Self {
        Self { calories: 0 }
    }
}

fn parse_elves(input: &str) -> Vec<Elf> {
   input.trim_end_matches('\n').split("\n\n").map(|chunk| Elf::from_str(chunk).unwrap()).collect()
}

fn find_max_calories(elves: &mut [Elf]) -> &Elf {
    elves.sort_by_key(|elf| elf.calories);
    elves.last().unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("input.txt")?;
    let mut elves = parse_elves(&content);
    let elf = find_max_calories(&mut elves);
    println!("{}", elf.calories);

    elves.sort_by_key(|elf| elf.calories);
    let top_three = elves.iter().rev().take(3);
    println!("{}", top_three.map(|elf| elf.calories).sum::<usize>());


    Ok(())
}

mod tests {
    use super::*;

    #[test]
    fn test_construct_elf_from_str() {
        let input = "1000\n2000\n3000\n";
        let elf = Elf::from_str(input).unwrap();
        assert_eq!(elf.calories, 6000)
    }

    #[test]
    fn test_parse_elves() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n";
        let elves = parse_elves(input);
        assert_eq!(elves[0].calories, 6000);
        assert_eq!(elves[1].calories, 4000);
        assert_eq!(elves[2].calories, 11000);
    }

    #[test]
    fn test_find_max_calories() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n";
        let mut elves = parse_elves(input);
        let elf = find_max_calories(&mut elves);
        assert_eq!(elf.calories, 11000);
    }
}
