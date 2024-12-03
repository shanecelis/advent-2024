use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, one_of},
    combinator::map,
    multi::{fold_many0, many_m_n},
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn digit_parser(input: &str) -> IResult<&str, i32> {
    // many_m_n(1, 3, map_res(one_of("0123456789"), |c| c.as_str().parse::<i32>()))(input).map(|(s, mut v)| {
    many_m_n(1, 3, map(one_of("0123456789"), |c| c as i32 - 0x30))(input).map(|(s, mut v)| {
        v.reverse();
        let mut accum = 0;
        let mut place = 1;
        for n in v.into_iter() {
            accum += n * place;
            place *= 10;
        }
        (s, accum)
    })
}

enum Inst {
    Do,
    Dont,
    Mul(Mul),
}

#[derive(Debug, PartialEq)]
struct Mul(i32, i32);

fn mul_parser(input: &str) -> IResult<&str, Mul> {
    preceded(
        tag("mul"),
        delimited(
            tag("("),
            separated_pair(digit_parser, tag(","), digit_parser),
            tag(")"),
        ),
    )
    .map(|(a, b)| Mul(a, b))
    .parse(input)
}

fn inst_parser(input: &str) -> IResult<&str, Inst> {
    alt((
        tag("do()").map(|_| Inst::Do),
        tag("don't()").map(|_| Inst::Dont),
        mul_parser.map(Inst::Mul),
    ))
    .parse(input)
}

fn inst_parser_opt(input: &str) -> IResult<&str, Option<Inst>> {
    alt((inst_parser.map(Some), anychar.map(|_| None))).parse(input)
}

fn inst_parser0(input: &str) -> IResult<&str, Vec<Inst>> {
    fold_many0(inst_parser_opt, Vec::new, |mut acc: Vec<_>, item| {
        if let Some(x) = item {
            acc.push(x);
        }
        acc
    })(input)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let mut accum: i32 = 0;
    let mut enabled = true;
    for line in read_lines(filename)? {
        if let Ok((_, insts)) = inst_parser0(&line?) {
            for inst in insts {
                match inst {
                    Inst::Do => enabled = true,
                    Inst::Dont => enabled = false,
                    Inst::Mul(Mul(a, b)) => {
                        if enabled {
                            accum += a * b;
                        }
                    }
                }
            }
        } else {
            eprintln!("eek");
        }
    }

    println!("{}", accum);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_inst() {
        assert!(inst_parser("mul(1,2)").is_ok());
        assert!(inst_parser("do()mul(1,2)").is_ok());
        assert!(inst_parser("mul(1,23)").is_ok());
        assert!(inst_parser("mul(1,234)").is_ok());
        assert!(inst_parser("mul(1,2344)").is_err());
        assert!(inst_parser("don't()mul(1,2344)").is_ok());
    }
}
