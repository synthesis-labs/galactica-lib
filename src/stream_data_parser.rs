use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{multispace0, newline};
use nom::combinator::opt;
use nom::multi::many0;
use nom::sequence::tuple;

use crate::parser::Parser;

fn single_data_record() -> impl Fn(&str) -> Parser<&str, String> {
    move |i| {
        tuple((tag("data:"), multispace0, take_until("\n"), newline))(i)
            .map(|(next, (_, _, value, _))| (next, value.to_string()))
    }
}

// Weirdly we get the random ":\n" (blank record added to the stream)
// usually right at the beginning...
fn blank_data_record() -> impl Fn(&str) -> Parser<&str, ()> {
    move |i| tuple((tag(":"), newline))(i).map(|(next, _)| (next, ()))
}

pub fn stream_data_parser() -> impl Fn(&str) -> Parser<&str, Vec<String>> {
    move |i| {
        tuple((
            opt(newline),
            opt(blank_data_record()),
            many0(single_data_record()),
        ))(i)
        .map(|(n, (_, _, v))| (n, v))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;

    use super::*;

    #[test]
    pub fn test_parser() {
        let input1 = "data: {\"some_json\":\"blahx\"}\n";
        println!("Input1 => {:?}", parse(stream_data_parser(), input1));

        let input2 = "data: {\"some_json\":\"blah1\"}\ndata: {\"some_json\":\"blah2\"}\n";
        println!("Input2 => {:?}", parse(stream_data_parser(), input2));

        let input3 = ":\n";
        println!("Input3 => {:?}", parse(stream_data_parser(), input3));
    }
}
