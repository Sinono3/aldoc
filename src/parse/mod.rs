mod list;
mod util;
pub use util::*;
pub use list::*;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{recognize, opt, map},
    multi::{many0, many1, many1_count},
    sequence::{terminated, pair},
    character::complete::line_ending,
};
use nom::IResult;
use nom::error::ErrorKind;

/// Replaces all consecutive line endings and tabs with a single space
pub fn format_text(s: &str) -> String {
    let r: IResult<&str, String> = map(
        many0(
            alt((
                map(
                    many1(alt((line_ending, tag("\t")))),
                    |_| " "
                ),
                take(1u8)
            ))
        ),
        |v: Vec<&str>| v.join("")
    )(s);
    r.expect("format_text cannot fail").1
    // This function cannot fail
    // TODO: unit-test this
}

#[derive(Debug, Clone)]
pub enum Block {
    Heading(usize, String),
    Paragraph(String),
    List(List), // contains both ordered and unordered
}

fn end(input: &str) -> IResult<&str, &str> {
    recognize(pair(line_ending, line_ending))(input)
}

fn block_text(input: &str) -> IResult<&str, String> {
    map(
        terminated(
            take_until_match(end),
            opt(end)
        ),
        |s: &str| s.to_string()
    )(input)
}

fn parse_block_heading(input: &str) -> IResult<&str, Block> {
    map(
        pair(
            terminated(
                many1_count(tag("#")),
                tag(" ")
            ),
            block_text
        ),
        |(level, s)| Block::Heading(level, s)
    )(input)
}

fn parse_block_paragraph(input: &str) -> IResult<&str, Block> {
    map(
        block_text,
        |s| Block::Paragraph(format_text(&s))
    )(input)
}

fn parse_block_list(input: &str) -> IResult<&str, Block> {
    map(
        parse_list(0),
        |l| Block::List(l)
    )(input)
}

fn parse_block(input: &str) -> IResult<&str, Block> {
    alt((
        parse_block_heading,
        parse_block_list,
        parse_block_paragraph,
    ))(input)
}

/// An Aldoc document abstraction.
pub struct Document {
    pub blocks: Vec<Block>
}

/// Parses raw Aldoc text into a document abstraction.
pub fn parse(input: &str) -> Result<Document, nom::Err<(&str, ErrorKind)>> {
    many0(
        parse_block
    )(input)
        .map(|(_, blocks)| Document { blocks })
}
