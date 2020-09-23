mod list;
pub use list::*;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{opt, not, map},
    multi::{many0, many1, many1_count},
    sequence::{terminated, preceded, pair},
};
use nom::IResult;

pub type ParseError<'s> = nom::Err<(&'s str, nom::error::ErrorKind)>;

/// Replaces all newlines with a single space
pub fn format_text(s: &str) -> String {
    s.replace("\n", " ")
}

#[derive(Debug, Clone)]
pub enum Block {
    Heading(usize, String),
    Paragraph(String),
    List(List), // contains both ordered and unordered
}

fn end(input: &str) -> IResult<&str, &str> {
    tag("\n\n")(input)
}

fn block_text(input: &str) -> IResult<&str, String> {
    terminated(
        map(
            many1(
                preceded(
                    not(end),
                    take(1u8)
                )
            ),
            |s: Vec<&str>| s.join("").to_string()
        ),
        opt(end)
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
pub fn parse(input: &str) -> Result<Document, nom::Err<(&str, nom::error::ErrorKind)>> {
    many0(
        parse_block
    )(input)
        .map(|(_, blocks)| Document { blocks })
}
