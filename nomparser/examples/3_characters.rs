use nom::character::complete::{alpha0, digit0};
use nom::IResult;

fn parse_alpha(input: &str) -> IResult<&str, &str> {
    alpha0(input)
}

fn parse_digit(input: &str) -> IResult<&str, &str> {
    digit0(input)
}

fn main() {
    match parse_alpha("hello world is the test 123") {
        Ok(res) => {
            println!("{:?}", res)
        }
        Err(err) => println!("{:?} Error", err),
    }

    match parse_alpha("hellohowareyoudoing") {
        Ok(res) => {
            println!("{:?}", res)
        }
        Err(err) => println!("{:?} Error", err),
    }
    match parse_digit("123hello") {
        Ok(res) => {
            println!("{:?}", res)
        }
        Err(err) => println!("{:?} Error", err),
    }
}
