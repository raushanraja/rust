use nom::bytes::complete::{tag, tag_no_case};
use nom::IResult;

fn parse_case_input(input: &str) -> IResult<&str, &str> {
    tag("abc")(input)
}

fn parse_nocase_input(input: &str) -> IResult<&str, &str> {
    tag_no_case("abc")(input)
}

fn main() {
    match parse_case_input("abc is my name seems okay??") {
        Ok(res) => {
            let (remaining_input, output) = res;
            assert_eq!(remaining_input, " is my name seems okay??");
            assert_eq!(output, "abc")
        }
        Err(err) => println!("Errored whiled parsing {:?}", err),
    }

    match parse_nocase_input("ABC is my name seems okay??") {
        Ok(res) => {
            let (remaining_input, output) = res;
            assert_eq!(remaining_input, " is my name seems okay??");
            assert_eq!(output, "ABC")
        }
        Err(err) => println!("Errored whiled parsing {:?}", err),
    }

    let res = parse_nocase_input("is my name seems okay??");
    assert!(res.is_err());
}
