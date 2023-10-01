use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit0;
use nom::IResult;

fn parse_alternate(input: &str) -> IResult<&str, &str> {
    alt((tag("abc"), tag("def"), digit0))(input)
}

fn main() {
    match parse_alternate("abc") {
        Ok((_, res)) => println!("res: {}", res),
        Err(e) => println!("err: {}", e),
    }
    match parse_alternate("def") {
        Ok((_, res)) => println!("res: {}", res),
        Err(e) => println!("err: {}", e),
    }

    match parse_alternate("123hello") {
        Ok((_, res)) => println!("res: {}", res),
        Err(e) => println!("err: {}", e),
    }
}
