use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{multispace0, newline};
use nom::combinator::{opt, rest};
use nom::multi::many0;
use nom::sequence::tuple;

use crate::parser::Parser;

#[derive(Debug)]
pub enum StreamRecordType {
    Data(Vec<String>),
    Error(String),
}

fn single_data_record() -> impl Fn(&str) -> Parser<&str, String> {
    move |i| {
        tuple((
            opt(newline),
            tag("data:"),
            multispace0,
            take_until("\n"),
            newline,
        ))(i)
        .map(|(next, (_, _, _, value, _))| (next, value.to_string()))
    }
}

// Weirdly we get the random ":\n" (blank record added to the stream)
// usually right at the beginning...
fn blank_data_record() -> impl Fn(&str) -> Parser<&str, ()> {
    move |i| tuple((tag(":"), newline))(i).map(|(next, _)| (next, ()))
}

pub fn stream_data_parser() -> impl Fn(&str) -> Parser<&str, StreamRecordType> {
    move |i| {
        tuple((
            opt(newline),
            opt(blank_data_record()),
            many0(single_data_record()),
        ))(i)
        .map(|(n, (_, _, v))| (n, StreamRecordType::Data(v)))
    }
}

pub fn error_parser() -> impl Fn(&str) -> Parser<&str, StreamRecordType> {
    move |i| {
        tuple((tag("{"), rest))(i)
            .map(|(n, (v, rem))| (n, StreamRecordType::Error(format!("{}{}", v, rem))))
    }
}

pub fn stream_data_or_error_parser() -> impl Fn(&str) -> Parser<&str, StreamRecordType> {
    move |i| {
        let x = alt((error_parser(), stream_data_parser()))(i).map(|(n, v)| (n, v));
        x
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;

    use super::*;

    #[test]
    pub fn test_parser() {
        let input1 = "data: {\"some_json\":\"blahx\"}\n";
        println!(
            "Input1 => {:?}",
            parse(stream_data_or_error_parser(), input1)
        );

        let input2 = "data: {\"some_json\":\"blah1\"}\ndata: {\"some_json\":\"blah2\"}\n";
        println!(
            "Input2 => {:?}",
            parse(stream_data_or_error_parser(), input2)
        );

        let input3 = ":\n";
        println!(
            "Input3 => {:?}",
            parse(stream_data_or_error_parser(), input3)
        );

        let input4 = "{\n\
            \"error\": {\n\
                \"message\": \"You exceeded your current quota, please check your plan and billing details.\",\n\
                \"type\": \"insufficient_quota\",\n\
                \"param\": null,\n\
                \"code\": null\n\
            }\n\
        }\n";
        println!(
            "Input4 => {:?}",
            parse(stream_data_or_error_parser(), input4)
        );

        let input5 = "data: {\"some_json\":\"blah1\"}\n\ndata: {\"some_json\":\"blah2\"}\n\ndata: {\"some_json\":\"blah2\"}\n";
        println!(
            "Input5 => {:?}",
            parse(stream_data_or_error_parser(), input5)
        );
    }
}
