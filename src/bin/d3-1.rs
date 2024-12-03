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

fn mul_parser_opt(input: &str) -> IResult<&str, Option<Mul>> {
    alt((mul_parser.map(Some), anychar.map(|_| None))).parse(input)
}

fn mul_parser0(input: &str) -> IResult<&str, Vec<Mul>> {
    fold_many0(mul_parser_opt, Vec::new, |mut acc: Vec<_>, item| {
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
    for line in read_lines(filename)? {
        if let Ok((_, muls)) = mul_parser0(&line?) {
            for Mul(a, b) in muls {
                accum += a * b;
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
    fn parse_digit() {
        assert!(digit_parser("1").is_ok());
        assert!(digit_parser("12").is_ok());
        assert!(digit_parser("132").is_ok());
        assert!(digit_parser("1324").is_ok());
        assert_eq!(digit_parser("1324"), Ok(("4", 132)));
    }

    #[test]
    fn parse_mul() {
        assert!(mul_parser("mul(1,2)").is_ok());
        assert!(mul_parser("mul(1,23)").is_ok());
        assert!(mul_parser("mul(1,234)").is_ok());
        assert!(mul_parser("mul(1,2344)").is_err());
    }

    #[test]
    fn parse_mul_opt() {
        assert!(mul_parser_opt("mul(1,2)").is_ok());
        assert!(mul_parser_opt("mul(1,23)").is_ok());
        assert!(mul_parser_opt("mul(1,234)").is_ok());
        assert!(mul_parser_opt("mul(1,2344)").is_ok());
    }

    #[test]
    fn parse_mul1() {
        assert!(mul_parser0("mul(1,2)").is_ok());
        assert!(mul_parser0("mul(1,23)").is_ok());
        assert!(mul_parser0("mul(1,234)").is_ok());
        assert!(mul_parser0("mul(1,2344)").is_ok());
        assert_eq!(mul_parser0("mul(1,2344)"), Ok(("", vec![])));
        assert!(mul_parser0("    mul(1,2)").is_ok());
    }

    #[test]
    fn parse_sample() {
        let ref r @ Ok((s, ref v)) =
            mul_parser0("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")
        else {
            panic!()
        };
        assert_eq!(s.len(), 0);
        assert!(r.is_ok());
        assert_eq!(v.len(), 4);
    }
}
