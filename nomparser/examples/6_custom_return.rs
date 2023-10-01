use nom::bytes::complete::tag;
use nom::character::complete::{i64, multispace0};
use nom::sequence::{delimited, separated_pair, tuple};
use nom::IResult;

#[derive(Debug)]
pub struct Coordinate {
    pub x: i64,
    pub y: i64,
}

fn parse_coordinate(input: &str) -> IResult<&str, (i64, i64)> {
    delimited(tag("("), separated_pair(i64, tag(","), i64), tag(")"))(input)
}


fn parse_spaced_coordinate(input: &str) -> IResult<&str, (i64, i64)> {
    delimited(
        tuple((multispace0, tag("("))),
        separated_pair(
            delimited(multispace0, i64, multispace0),
            tag(","),
            delimited(multispace0, i64, multispace0),
        ),
        tuple((multispace0, tag(")"))),
    )(input)
}

fn main() {
    match parse_coordinate("(200,29)") {
        Ok((_, (x, y))) => {
            println!("Result: {:?}", Coordinate { x, y });
        }
        Err(err) => println!("{:?}", err),
    }

    match parse_spaced_coordinate("( 200, 29)") {
        Ok((_, (x, y))) => {
            println!("Result: {:?}", Coordinate { x, y });
        }
        Err(err) => println!("{:?}", err),
    }
}

/*
(200,-234)
   - Remove first `(`
   - Get Integer
   - Remove `,`
   - Get Integer
   - Remove last `)`
*/

/*
(200,-234)
   - Remoce anyspace
   - Remove first `(`
   - Get Integer
   - Remoce anyspace
   - Remove `,`
   - Remoce anyspace
   - Get Integer
   - Remove last `)`
   - Remoce anyspace
* */
