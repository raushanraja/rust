use std::error::Error;

use nom::IResult;

fn do_nothing_parser(input: &str) -> IResult<&str, &str> {
    Ok((input, ""))
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!, I'm a Rustacean! and I'm learning nom!");
    let (remaining_input, output) = do_nothing_parser("my_input")?;
    assert_eq!(remaining_input, "my_input");
    assert_eq!(output, "");
    Ok(())
}



/*
 * Simplest parser example
 * Does nothing, but shows how to use nom, pass input and return output
 * The remaining input is the input that was not parsed
 * The output is the output of the parser
*/
