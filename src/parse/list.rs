use crate::format_text;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while},
    combinator::{verify, recognize, opt, not, map, map_res},
    multi::{many1, count},
    sequence::{pair, terminated, preceded},
};

use nom::IResult;
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ListToken {
    // unordered
    Dot,

    // ordered
    Numbered,
    RomanLowercase,
    RomanUppercase,
    AlphabeticalLowercase,
    AlphabeticalUppercase,
}

#[derive(Debug, Clone)]
pub struct List {
    pub vec: Vec<ListItem>,
    pub token: ListToken
}

#[derive(Debug, Clone)]
pub struct ListItem {
    pub text: String, 
    pub list: Option<List>
}

// this parses the full token.
fn parse_item_token(input: &str) -> IResult<&str, ListToken> {
    let terminator =
        alt((
            tag(")"),
            tag("-"),
            tag("."),
        ));
    let numbered =
        terminated(
            take_while(|c: char| c.is_digit(10)),
            terminator,
        );

    map_res(
        alt((
           tag("-"),
            tag("+"),
            numbered
        )),
        |t: &str| {
            // check if it is a number
            let ch = t.chars().nth(0).unwrap();

            if ch.is_digit(10) {
                return Ok(ListToken::Numbered);
            }

            match t {
                "-" => Ok(ListToken::Dot),
                "+" => Ok(ListToken::Dot),
                _ => Err("No matching token.")
            }
        }
    )(input)
}

fn parse_item_start(indent: usize) -> impl Fn(&str) -> IResult<&str, ListToken> {
    move |input: &str|
        map(
            pair(
                count(tag("\t"), indent),
                parse_item_token,
            ),
            |(_, token)| token
        )(input)
}

fn parse_item_start_and_enforce(indent: usize, enforced: ListToken) -> impl Fn(&str) -> IResult<&str, ListToken> {
    move |input: &str|
        verify(
            parse_item_start(indent),
            |token| *token == enforced
        )(input)
}

fn parse_item(indent: usize, token: ListToken) -> impl Fn(&str) -> IResult<&str, ListItem> {
    let item_start = parse_item_start_and_enforce(indent, token);
    let next_item_start = parse_item_start(indent + 1);

    move |input: &str|
        map(
            terminated(
                pair(
                    &item_start,
                    map(
                        many1(
                            preceded(
                                not(pair(
                                    tag("\n"),
                                    alt((
                                        tag("\n"),
                                        recognize(&item_start),
                                    ))
                                )),

                                take(1u8)
                            )
                        ),
                        |s: Vec<&str>| s.join("").trim().to_string()
                    ),
                ),
                opt(tag("\n"))
            ),
            |(_, mut content)| {
                let mut list = None;

                // checks for sublists
                for i in 0..content.len() {
                    let s = &content[i..];

                    if let Ok(_) = next_item_start(s) {
                        if let Ok((_, result)) = parse_list(indent + 1)(s) {
                            list = Some((i, result));
                            println!("YEAH {}", s);
                            break;
                        }
                    }
                }
                if let Some((i, _)) = list {
                    // -1 because of the newline
                    content = format_text(&content[..i-1]);
                } else {
                    content = format_text(&content);
                }

                ListItem {
                    text: content,
                    list: list.map(|l| l.1)
                }
            }
        )(input)
}

pub fn parse_list(indent: usize) -> impl Fn(&str) -> IResult<&str, List> {
    move |input: &str| {
        // this works, but there must be a better way
        let mut token = ListToken::Dot;

        if let Ok((_, result)) = parse_item_start(indent)(input){
            token = result;
        }

        map(
            terminated(
                many1(
                    parse_item(indent, token)
                ),
                opt(tag("\n"))
            ),
            move |items| {
                List {
                    vec: items,
                    token
                }
            }
        )(input)
    }
}

