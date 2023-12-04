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
struct StarSymbol {
    row_number: usize,
    index: usize,
}

#[derive(Debug)]
struct Schematic {
    numbers: Vec<Number>,
    stars: Vec<StarSymbol>,
}

#[derive(PartialEq)]
enum Token {
    Blank,
    Digit,
    StarSymbol,
}

fn parse_schematic(path: &str) -> Result<Schematic> {
    let input = io::BufReader::new(File::open(path)?);
    let mut schematic: Schematic = Schematic {
        numbers: Vec::new(),
        stars: Vec::new(),
    };

    for (i, l) in input.lines().enumerate() {
        let line = l?;
        // parse each row and group by same token type
        let group_iterator = line.chars().enumerate().group_by(|(_, c)| {
            if c == &'*' {
                Token::StarSymbol
            } else if c.is_numeric() {
                Token::Digit
            } else {
                Token::Blank
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
                Token::StarSymbol => {
                    for (j, _) in group {
                        schematic.stars.push(StarSymbol {
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

fn get_powers(star: &StarSymbol, schematic: &Schematic) -> Option<usize> {
    let adjacent_numbers: Vec<_> = schematic
        .numbers
        .iter()
        .filter_map(|n| {
            // check that symbol is row before or same or row after
            if !(n.row_number as isize >= (star.row_number as isize) - 1
                && n.row_number <= star.row_number + 1)
            {
                return None;
            }

            // check that symbol column is between of number start index - 1 and number end index + 1
            if star.index as isize >= (n.start_index as isize) - 1 && star.index <= n.end_index + 1
            {
                Some(n.num)
            } else {
                None
            }
        })
        .collect();

    if adjacent_numbers.len() != 2 {
        return None;
    }

    Some(adjacent_numbers.iter().product())
}

fn part2(path: &str) -> Result<usize> {
    let schematic = parse_schematic(path)?;
    // println!("{:?}", schematic);
    let res = schematic
        .stars
        .iter()
        .filter_map(|s| get_powers(s, &schematic))
        .sum();
    Ok(res)
}

fn main() -> Result<()> {
    let res = part2(PATH)?;
    println!("{}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn t1() {
        assert_eq!(super::part2("test_input.txt").unwrap(), 467835);
    }
}
