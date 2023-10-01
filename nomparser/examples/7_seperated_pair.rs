use nom::bytes::complete::tag;
use nom::character::complete::i64;
use nom::sequence::separated_pair;
use nom::IResult;

fn parse_coordinate(input: &str) -> IResult<&str, (i64, i64)> {
    separated_pair(i64, tag(","), i64)(input)
}


fn main(){
    println!("{:?}", parse_coordinate("1,2,33"));
}
