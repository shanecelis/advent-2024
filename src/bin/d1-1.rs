#![allow(unused_imports)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::num::ParseIntError;
use std::cmp::Reverse;
use nom::{
    IResult,
    Parser,
    error::ParseError,
    bytes::complete::{tag, take_while_m_n, take_while1},
    combinator::{map_res},
    sequence::{tuple, delimited, separated_pair},
    character::{is_digit, complete::{digit1, multispace0, multispace1, i32}},
};

use std::collections::BinaryHeap;

// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
// trailing whitespace, returning the output of `inner`.
// pub fn ws<'a, O, E: ParseError<&'a str>, F>(
//     inner: F,
// ) -> impl Parser<&'a str, Output = O, Error = E>
// where
//     F: Parser<&'a str, Output = O, Error = E>,
// {
//     delimited(multispace0, inner, multispace0)
// }

fn from_int(input: &str) -> Result<i32, ParseIntError> {
    i32::from_str_radix(input, 10)
}

// fn is_digit(c: char) -> bool {
//     c.is_digit(10)
// }

fn int_parser(input: &str) -> IResult<&str, i32> {
    i32(input)
    // map_res(take_while_m_n(2, 2, is_digit),
    //         from_int)(input)
    // map_res(digit1, |s: &str| s.parse::<i32>())
    // map_res(
    // i32::from_str_radix(input, 10)
}

fn row_parser(input: &str) -> IResult<&str, (i32, i32)> {
    delimited(multispace0, separated_pair(i32, multispace1, i32), multispace0)(input)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    eprintln!("filename {filename}");
    let mut accum: i32 = 0;
    let mut heap_a = BinaryHeap::new();
    let mut heap_b = BinaryHeap::new();
    for line in read_lines(filename)?  {
        if let Ok((_, (a, b))) = row_parser(&line?) {
            heap_a.push(Reverse(a));
            heap_b.push(Reverse(b));
        } else {
            eprintln!("eek");
        }
    }
    while !heap_a.is_empty() {
        let a = heap_a.pop().unwrap().0;
        let b = heap_b.pop().unwrap().0;
        accum += (a - b).abs()
    }

    println!("{}", accum);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_number() {
        assert_eq!(from_int("10"), Ok(10));
        assert!(from_int(" 10").is_err());
        assert!(from_int("10 ").is_err());
        // assert!(ws(int_parser)("10 ").is_ok());
    }

    #[test]
    fn parse_row() {
        assert_eq!(row_parser("   10   30   "), Ok(("", (10, 30))));
        assert!(row_parser("   10   30   30").is_ok());
        assert_eq!(row_parser("   10   30   30"), Ok(("30", (10, 30))));
    }
}