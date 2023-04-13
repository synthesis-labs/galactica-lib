use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until, take_until1},
    character::complete::digit1,
    combinator::opt,
    sequence::tuple,
};

use crate::parser::Parser;

#[derive(Debug)]
pub struct Version {
    pub arch_triple: String,
    pub version: String,
    pub build: String,
    pub commit: String,
}

pub fn valid_arch_name() -> impl Fn(&str) -> Parser<&str, String> {
    move |i| tuple((take_until1("-"), tag("-")))(i).map(|(n, (a, _))| (n, a.to_string()))
}

pub fn arch_triple_parser() -> impl Fn(&str) -> Parser<&str, String> {
    // aarch64-apple-darwin
    move |i| {
        tuple((
            valid_arch_name(),
            valid_arch_name(),
            valid_arch_name(),
            opt(valid_arch_name()),
        ))(i)
        .map(|(n, (a, b, c, d))| {
            if let Some(d) = d {
                (n, format!("{}-{}-{}-{}", a, b, c, d))
            } else {
                (n, format!("{}-{}-{}", a, b, c))
            }
        })
    }
}

pub fn version_parser() -> impl Fn(&str) -> Parser<&str, String> {
    move |i| {
        tuple((digit1, tag("."), digit1, tag("."), digit1))(i)
            .map(|(n, (a, _, b, _, c))| (n, format!("{}.{}.{}", a, b, c)))
    }
}

pub fn filename_parser() -> impl Fn(&str) -> Parser<&str, Version> {
    // galactica-aarch64-apple-darwin-0.1.0-build.17.014b5bf.tar.gz
    move |i| {
        tuple((
            tag("galactica-"),
            arch_triple_parser(),
            version_parser(),
            tag("+build."),
            digit1,
            tag("."),
            take(7usize),
        ))(i)
        .map(|(next, (_, arch_triple, version, _, build, _, commit))| {
            (
                next,
                Version {
                    arch_triple,
                    version,
                    build: build.to_string(),
                    commit: commit.to_string(),
                },
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;

    use super::*;

    #[test]
    pub fn test_parser() {
        let input1 = "galactica-aarch64-apple-darwin-0.1.0+build.17.014b5bf.tar.gz";
        println!("Input1 => {:?}", parse(filename_parser(), input1));

        let input2 = "galactica-x86_64-apple-darwin-0.1.0+build.17.014b5bf.tar.gz";
        println!("Input2 => {:?}", parse(filename_parser(), input2));

        let input3 = "galactica-x86_64-pc-windows-gnu-0.1.0+build.17.014b5bf.zip";
        println!("Input3 => {:?}", parse(filename_parser(), input3));

        let input4 = "galactica-x86_64-apple-darwin-0.1.0+build.92.c770688";
        println!("Input4 => {:?}", parse(filename_parser(), input4));
    }
}
