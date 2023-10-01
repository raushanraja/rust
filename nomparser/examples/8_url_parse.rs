use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1, hex_digit1, multispace0, one_of},
    combinator::{map, map_res, opt},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug)]
struct Query {
    key: String,
    value: String,
}


#[derive(Debug)]
struct Uri {
    scheme: String,
    authority: Option<Authority>,
    path: Path,
    query: Option<String>,
    fragment: Option<String>,
}

#[derive(Debug)]
pub struct UserInfo {
    username: Option<String>,
    password: Option<String>,
}

#[derive(Debug)]
struct Authority {
    userinfo: Option<UserInfo>,
    host: String,
    port: Option<u16>,
}

#[derive(Debug)]
enum Path {
    AbEmpty(Vec<String>),
    Absolute(Vec<String>),
    NoScheme(Vec<String>),
    Rootless(Vec<String>),
    Empty,
}

fn ipv6_address(input: &str) -> IResult<&str, String> {
    let (input, addr) = alt((ipv6_literal, ipv6_hex_comp))(input)?;
    Ok((input, addr))
}

fn ipv6_literal(input: &str) -> IResult<&str, String> {
    let (input, _open_bracket) = char('[')(input)?;
    let (input, address) = separated_pair(
        many1(terminated(hex_digit1, char(':'))),
        char(':'),
        many1(terminated(hex_digit1, char(':'))),
    )(input)?;
    let (input, _close_bracket) = char(']')(input)?;

    // Combine and format the address parts
    let address_parts: Vec<_> = address
        .0
        .iter()
        .chain(address.1.iter())
        .map(|s| s.to_string())
        .collect();
    let formatted_address = format!("[{}]", address_parts.join(":"));

    Ok((input, formatted_address))
}

fn ipv6_hex_comp(input: &str) -> IResult<&str, String> {
    let (input, parts) = many1(terminated(hex_digit1, char(':')))(input)?;
    let formatted_address = format!("::{}", parts.join(":"));

    Ok((input, formatted_address))
}

fn ipv4_address(input: &str) -> IResult<&str, String> {
    let (input, ((a, b), (c, d))) = separated_pair(
        separated_pair(digit1, char('.'), digit1),
        char('.'),
        separated_pair(digit1, char('.'), digit1),
    )(input)?;

    let ipv4_addr = format!("{}.{}.{}.{}", a, b, c, d);
    Ok((input, ipv4_addr))
}

fn pct_encoded(input: &str) -> IResult<&str, String> {
    map_res(preceded(tag("%"), digit1), |s: &str| {
        u8::from_str_radix(s, 16).map(|b| b.to_string())
    })(input)
}

fn unreserved(input: &str) -> IResult<&str, String> {
    map(
        alt((alpha1, digit1, tag("-"), tag("."), tag("_"), tag("~"))),
        |s: &str| s.to_string(),
    )(input)
}

fn gen_delims(input: &str) -> IResult<&str, String> {
    map(
        alt((
            tag(":"),
            tag("/"),
            tag("?"),
            tag("#"),
            tag("["),
            tag("]"),
            tag("@"),
        )),
        |s: &str| s.to_string(),
    )(input)

    // map(one_of(":/?#[]@"), |c| c.to_string())(input)
}

fn sub_delims(input: &str) -> IResult<&str, String> {
    map(
        alt((
            tag("!"),
            tag("$"),
            tag("&"),
            tag("'"),
            tag("("),
            tag(")"),
            tag("*"),
            tag("+"),
            tag(","),
            tag(";"),
            tag("="),
        )),
        |s: &str| s.to_string(),
    )(input)

    // map(one_of("!$&'()*+,;="), |c| c.to_string())(input)
}

fn atsign(input: &str) -> IResult<&str, String> {
    map(alt((tag("@"),)), |s: &str| s.to_string())(input)
}

fn slash(input: &str) -> IResult<&str, String> {
    map(alt((tag("/"),)), |s: &str| s.to_string())(input)
}

fn quesetion(input: &str) -> IResult<&str, String> {
    map(alt((tag("/"),)), |s: &str| s.to_string())(input)
}

fn colon(input: &str) -> IResult<&str, String> {
    map(alt((tag(":"),)), |s: &str| s.to_string())(input)
}

fn reserved(input: &str) -> IResult<&str, String> {
    alt((gen_delims, sub_delims))(input)
}

fn userinfo(input: &str) -> IResult<&str, UserInfo> {
    map(
        pair(
            opt(alt((unreserved, pct_encoded, sub_delims))),
            opt(preceded(colon, alt((unreserved, pct_encoded, sub_delims)))),
        ),
        |(username, password)| UserInfo { username, password },
    )(input)
}
fn reg_name(input: &str) -> IResult<&str, String> {
    map(many0(alt((unreserved, pct_encoded, sub_delims))), |vec| {
        vec.join("")
    })(input)
}
fn host(input: &str) -> IResult<&str, String> {
    alt((ipv4_address, ipv6_address, reg_name))(input)
}

fn port(input: &str) -> IResult<&str, u16> {
    map_res(digit1, |s: &str| s.parse::<u16>())(input)
}

fn authority(input: &str) -> IResult<&str, Authority> {
    let (input, userinfo) = opt(terminated(userinfo, atsign))(input)?;
    let (input, host) = host(input)?;
    let (input, port) = opt(preceded(char(':'), port))(input)?;

    Ok((
        input,
        Authority {
            userinfo,
            host,
            port,
        },
    ))
}

fn path_abempty(input: &str) -> IResult<&str, Path> {
    let (input, segments) = many0(segment)(input)?;
    Ok((input, Path::AbEmpty(segments)))
}

fn path_absolute(input: &str) -> IResult<&str, Path> {
    let (input, segments) = preceded(char('/'), many0(segment))(input)?;
    Ok((input, Path::Absolute(segments)))
}

fn path_noscheme(input: &str) -> IResult<&str, Path> {
    let (input, segments) = many1(segment)(input)?;
    Ok((input, Path::NoScheme(segments)))
}

fn path_rootless(input: &str) -> IResult<&str, Path> {
    let (input, segments) = many1(preceded(char('/'), segment))(input)?;
    Ok((input, Path::Rootless(segments)))
}

fn path_empty(input: &str) -> IResult<&str, Path> {
    Ok((input, Path::Empty))
}

fn segment(input: &str) -> IResult<&str, String> {
    map(
        many1(alt((unreserved, pct_encoded, sub_delims, atsign, slash))),
        |vec| vec.join(""),
    )(input)
}

fn query_pair(input: &str) -> IResult<&str, Query> {

    let (input, key) = map(
        many1(alt((unreserved, pct_encoded, sub_delims, atsign, colon))),
        |vec| vec.join(""),
    )(input)?;
    

    let (input, _) = tag("=")(input)?;
    let (input, value) = map(many1(alt((unreserved, pct_encoded, sub_delims, atsign, colon))), |vec| vec.join(""),)(input)?;

    println!("key: {:?}, value: {:?}", key, value);

    Ok((
        input,
        Query {
            key,
            value,
        },
    ))
}



fn query_parse(input: &str) -> IResult<&str, Vec<Query>> {
    let (input, queries) = many1(query_pair)(input)?;
    println!("________--queries: {:?}", queries);
    Ok((input, queries))
}



fn scheme(input: &str) -> IResult<&str, String> {
    map(
        pair(
            alpha1,
            many0(alt((alpha1, digit1, tag("+"), tag("-"), tag(".")))),
        ),
        |(first, rest)| format!("{}{}", first, rest.join("")),
    )(input)
}

fn uri(input: &str) -> IResult<&str, Uri> {
    let (input, scheme) = scheme(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = tag("//")(input)?;
    let (input, authority) = opt(terminated(authority, tag("/")))(input)?;

    let (input, path) = terminated(
        preceded(
            opt(tag("/")),
            alt((
                path_abempty,
                path_absolute,
                path_noscheme,
                path_rootless,
                path_empty,
            )),
        ),
        tag("?"),
    )(input)?;
    let (input, query) = opt(many0(pchar))(input)?;
    let (input, fragment) = opt(preceded(char('#'), many0(pchar)))(input)?;

    println!("query: {:?}", query);

    Ok((
        input,
        Uri {
            scheme,
            authority,
            path,
            query:query.map(|vec| vec.join("")),
            fragment: fragment.map(|vec| vec.join("")),
        },
    ))
}

fn pchar(input: &str) -> IResult<&str, String> {
    alt((unreserved, pct_encoded, sub_delims, atsign, colon))(input)
}

fn main() {
    let input = "http://raushan:112@example.com:8080/path?query#fragment";
    match uri(input) {
        Ok((_, parsed_uri)) => println!("{:?}", parsed_uri),
        Err(e) => println!("Error: {:?}", e),
    }

    match uri("http://abc:somepass@192.168.68.33:8080/path?query#fragment") {
        Ok((_, parsed_uri)) => println!("{:?}", parsed_uri),
        Err(e) => println!("Error: {:?}", e),
    }

    match uri("https://www.example.com/secondpath/resource/someohter?param1=value1&param2=value2&param3=value3#fragment") {
        Ok((_, parsed_uri)) => println!("{:?}", parsed_uri),
        Err(e) => println!("Error: {:?}", e),
    }
}
