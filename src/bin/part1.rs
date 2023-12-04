use std::fs::File;
use std::io::{self, BufRead};

use anyhow::{Context, Result};
use itertools::Itertools;

const PATH: &str = "input.txt";

#[derive(Debug)]
struct Number {
    num: usize,
    row_number: usize,
    start_index: usize,
    end_index: usize,
}

#[derive(Debug)]
struct Symbol {
    row_number: usize,
    index: usize,
}

#[derive(Debug)]
struct Schematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

#[derive(PartialEq)]
enum Token {
    Blank,
    Digit,
    Symbol,
}

fn parse_schematic(path: &str) -> Result<Schematic> {
    let input = io::BufReader::new(File::open(path)?);
    let mut schematic: Schematic = Schematic {
        numbers: Vec::new(),
        symbols: Vec::new(),
    };

    for (i, l) in input.lines().enumerate() {
        let line = l?;
        // parse each row and group by same token type
        let group_iterator = line.chars().enumerate().group_by(|(_, c)| {
            if c == &'.' {
                Token::Blank
            } else if c.is_numeric() {
                Token::Digit
            } else {
                Token::Symbol
            }
        });

        for (t, group) in group_iterator.into_iter() {
            match t {
                Token::Blank => {}
                Token::Digit => {
                    let digits: Vec<(usize, char)> = group.collect();
                    schematic.numbers.push(Number {
                        num: digits.iter().map(|(_, c)| *c).collect::<String>().parse()?,
                        row_number: i,
                        start_index: digits[0].0,
                        end_index: digits.last().context("empty vectors")?.0,
                    });
                }
                Token::Symbol => {
                    for (j, _) in group {
                        schematic.symbols.push(Symbol {
                            row_number: i,
                            index: j,
                        });
                    }
                }
            }
        }
    }

    Ok(schematic)
}

fn number_is_part(number: &Number, schematic: &Schematic) -> bool {
    // to figure out if a number is a valid part number, we iterate through
    // all symbols, and see if there's a connection
    schematic.symbols.iter().any(|s| {
        // check that symbol is row before or same or row after
        if !(number.row_number as isize >= (s.row_number as isize) - 1
            && number.row_number <= s.row_number + 1)
        {
            return false;
        }

        // check that symbol column is between of number start index - 1 and number end index + 1
        s.index as isize >= (number.start_index as isize) - 1 && s.index <= number.end_index + 1
    })
}

fn part1(path: &str) -> Result<usize> {
    let schematic = parse_schematic(path)?;
    // println!("{:?}", schematic);
    Ok(schematic
        .numbers
        .iter()
        .filter(|n| number_is_part(n, &schematic))
        .map(|n| n.num)
        .sum())
}

fn main() -> Result<()> {
    let res = part1(PATH)?;
    println!("{}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn t1() {
        assert_eq!(super::part1("test_input.txt").unwrap(), 4361);
    }
}
