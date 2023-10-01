use nom::character::complete::{alpha0, alphanumeric0, digit0};
use nom::sequence::tuple;
use nom::IResult;

// First element of return tuple is the strings which were not parsed by any of the parsers
// Second element of return tuple is the output of tuple combinator which is a tuple of the output of the parsers passed in the tuple combinator
fn parse_composition(input: &str) -> IResult<&str, (&str, &str, &str)> {
    tuple((alpha0, digit0, alphanumeric0))(input)
}

fn main() {
    println!("{:?}", parse_composition("how1youdoing"));
    println!("{:?}", parse_composition("123howareyoudoing"));
    println!("{:?}", parse_composition("howareyoudoing12221"));
    println!("{:?}", parse_composition("hoiwa1rey3oudoing12221"));
    println!("{:?}", parse_composition("_@"));
}

// Output:
//   Ok(("", ("how", "1", "youdoing")))
//   Ok(("", ("", "123", "howareyoudoing")))
//   Ok(("", ("howareyoudoing", "12221", "")))
//   Ok(("", ("hoiwa", "1", "rey3oudoing12221")))
//   Ok(("_@", ("", "", "")))
