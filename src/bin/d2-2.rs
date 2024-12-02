use nom::{
    character::complete::{i32, multispace0, multispace1},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::once;
use std::path::Path;

fn row_parser(input: &str) -> IResult<&str, Vec<i32>> {
    delimited(multispace0, separated_list0(multispace1, i32), multispace0)(input)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_safe(x: i32) -> bool {
    let x = x.abs();
    (1..=3).contains(&x)
}

fn is_report_safe(mut iter: impl Iterator<Item = i32>) -> bool {
    let first = iter.next().unwrap();
    is_safe(first) && iter.all(|x| x * first > 0 && is_safe(x))
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let mut accum: i32 = 0;
    for line in read_lines(filename)? {
        if let Ok((_, report)) = row_parser(&line?) {
            let r: Vec<i32> = report.windows(2).map(|v| v[0] - v[1]).collect();
            if is_report_safe(r.iter().copied()) {
                accum += 1;
            } else {
                let skip = report.windows(3).map(|v| v[0] - v[2]).enumerate();

                if is_report_safe(r[1..].iter().copied())
                    || is_report_safe(r[..r.len() - 1].iter().copied())
                {
                    accum += 1
                } else {
                    for (i, skipped) in skip {
                        if is_report_safe(
                            r[0..i]
                                .iter()
                                .chain(once(&skipped))
                                .chain(r[i + 2..].iter())
                                .copied(),
                        ) {
                            accum += 1;
                            break;
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
    fn parse_row() {
        assert_eq!(row_parser("   10   30   "), Ok(("", vec![10, 30])));
        assert!(row_parser("   10   30   30").is_ok());
        assert_eq!(row_parser("   10   30   30"), Ok(("30", vec![10, 30])));
    }
}
