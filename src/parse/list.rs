use super::format_text;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{verify, recognize, opt, not, map},
    multi::{many1, count},
    sequence::{pair, terminated, preceded},
    character::complete::line_ending,
};
use nom::IResult;

pub use super::token::{ListToken, TokenEnumerator, TokenWrapper};
use super::token::parse_item_token;

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

fn parse_item_start_and_enforce(indent: usize, enforced: &'_ ListToken) -> impl Fn(&str) -> IResult<&str, ListToken> + '_ {
    move |input: &str|
        verify(
            parse_item_start(indent),
            |token| {
                if token == enforced {
                    return true;
                } 
                // this enables roman numerals to also qualify
                // as alphabetic tokens
                if let Some(a) = &enforced.enumerator {
                    if let Some(b) = &token.enumerator {
                        use TokenEnumerator::*;

                        if let Alphabetical(alpha_upper) = &a {
                            if let Roman(roman_upper) = &b {
                                // this part checks if they have the same case
                                return *alpha_upper == *roman_upper;
                            }
                        }
                    }
                }
                false
            }
        )(input)
}

fn parse_item(indent: usize, token: &ListToken) -> impl Fn(&str) -> IResult<&str, ListItem> + '_ {
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
                                    line_ending,
                                    alt((
                                        recognize(line_ending),
                                        recognize(&item_start),
                                    ))
                                )),
                                take(1u8)
                            )
                        ),
                        |s: Vec<&str>| s.join("").trim().to_string()
                    ),
                ),
                opt(line_ending)
            ),
            |(_, content)| {
                let mut list = None;

                let text;
                // checks for sublists
                for (i, _) in content.char_indices() {
                    let s = &content[i..];

                    if let Ok(_) = next_item_start(s) {
                        if let Ok(result) = parse_list(indent + 1)(s) {
                            let consumed = content[..i].trim();
                            list = Some((consumed, result.1));
                            break;
                        }
                    }
                }
                if let Some((consumed, _)) = list {
                    // -1 because of the newline
                    text = format_text(&consumed);
                } else {
                    text = format_text(&content.trim());
                }

                ListItem {
                    text,
                    list: list.map(|l| l.1)
                }
            }
        )(input)
}

pub fn parse_list(indent: usize) -> impl Fn(&str) -> IResult<&str, List> + 'static {
    move |input: &str| {
        // this works, but there must be a better way
        let mut token = ListToken::bullet();

        if let Ok((_, result)) = parse_item_start(indent)(input) {
            token = result;
        }
        let clone = token.clone();

        map(
            terminated(
                many1(
                    parse_item(indent, &token.clone())
                ),
                opt(line_ending)
            ),
            move |items| {
                List {
                    vec: items,
                    token: clone.clone()
                }
            }
        )(input)
    }
}

