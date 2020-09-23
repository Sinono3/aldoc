use crate::format_text;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while1},
    combinator::{verify, recognize, opt, not, map, map_opt},
    multi::{many1, count},
    sequence::{pair, terminated, preceded},
};
use numerals::roman::Numeral;

use nom::IResult;
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ListToken {
    // unordered
    Unnumbered,

    // ordered
    Numbered,
    Roman(bool),        // uppercase or lowercase
    Alphabetical(bool), // uppercase or lowercase
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

fn parse_terminator(input: &str) -> IResult<&str, &str> {
    alt((
        tag(")"),
        tag("-"),
        tag("."),
    ))(input)
}
// this parses the full token.
fn parse_item_token(input: &str) -> IResult<&str, ListToken> {
    let unnumbered = 
        map(
            alt((
                tag("-"),
                tag("+")
            )),
            |_| ListToken::Unnumbered
        );
    let numbered =
        map(
            terminated(
                take_while1(|c: char| c.is_digit(10)),
                parse_terminator,
            ),
            |_| ListToken::Numbered
        );
    let alphabetic = 
        map(
            terminated(
                verify(
                    take(1u8),
                    |c: &str| {
                        let c = c.chars().nth(0).unwrap();
                        c.is_alphabetic()
                    }
                ),
                parse_terminator,
            ),
            |a| {
                let ch = a.chars().nth(0).unwrap();
                ListToken::Alphabetical(ch.is_uppercase())
            }
        );
    let roman =
        map(
            terminated(
                take_while1(|r| Numeral::from_char(r).is_some()),
                parse_terminator
            ),
            |n| {
                let ch = n.chars().nth(0).unwrap();
                ListToken::Roman(ch.is_uppercase())
            }
        );
    
    alt((
        unnumbered,
        numbered,
        roman,
        alphabetic,
    ))(input)
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
    use ListToken::*;

    move |input: &str|
        verify(
            parse_item_start(indent),
            |token| {
                if *token == enforced {
                    return true;
                } 
                // this enables roman numerals to also qualify
                // as alphabetic tokens
                if let Alphabetical(alpha_upper) = enforced {
                    if let Roman(roman_upper) = token {
                        // this part checks if they have the same case
                        return alpha_upper == *roman_upper;
                    }
                }
                false
            }
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
        let mut token = ListToken::Unnumbered;

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

