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

use std::collections::{HashMap, BinaryHeap};

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
    let mut accum: i32 = 0;
    // let mut heap_a = BinaryHeap::new();
    let mut heap_a = BinaryHeap::new();
    let mut count_a = HashMap::new();
    let mut heap_b = BinaryHeap::new();
    let mut count_b = HashMap::new();
    for line in read_lines(filename)?  {
        if let Ok((_, (a, b))) = row_parser(&line?) {
            count_a.entry(a).and_modify(|counter| *counter += 1).or_insert(1);
            count_b.entry(b).and_modify(|counter| *counter += 1).or_insert(1);
            heap_a.push(Reverse(a));
            heap_b.push(Reverse(b));
        } else {
            eprintln!("eek");
        }
    }
    while !heap_a.is_empty() {
        let a = heap_a.pop().unwrap().0;
        // let b = heap_b.pop().unwrap().0;
        accum += a * count_b.get(&a).unwrap_or(&0)
        // b * count_a.get(b).unwrap_or(0)
        // accum += (a - b).abs()
    }

    println!("{}", accum);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_row() {
        assert_eq!(row_parser("   10   30   "), Ok(("", (10, 30))));
        assert!(row_parser("   10   30   30").is_ok());
        assert_eq!(row_parser("   10   30   30"), Ok(("30", (10, 30))));
    }
}
