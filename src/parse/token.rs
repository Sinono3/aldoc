use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while1},
    combinator::{verify, map},
    sequence::terminated,
    IResult
};
use numerals::roman::Numeral;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListToken {
    pub wrapper: TokenWrapper,
    // If the enumerator is `None` then it is unnumbered
    pub enumerator: Option<TokenEnumerator>,
}
impl ListToken {
    fn unnumbered(wrapper: &str) -> ListToken {
        ListToken {
            wrapper: TokenWrapper(String::from(wrapper)),
            enumerator: None,
        }
    }
    pub fn bullet() -> ListToken {
        ListToken::unnumbered("*")
    }
    pub fn plus() -> ListToken {
        ListToken::unnumbered("+")
    }
    pub fn hyphen() -> ListToken {
        ListToken::unnumbered("-")
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenEnumerator {
    Numerical,
    Roman(bool),        // uppercase or lowercase
    Alphabetical(bool), // uppercase or lowercase
}
impl TokenEnumerator {
    pub fn latex(&self) -> String {
        use TokenEnumerator::*;

        let s = match self {
            Numerical           => "\\arabic",
            Alphabetical(true)  => "\\Alph",
            Alphabetical(false) => "\\alph",
            Roman(true)     	=> "\\Roman",
            Roman(false)    	=> "\\roman",
        };
        format!("{}*", s)
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TokenWrapper(String);

impl TokenWrapper {
    pub fn label(&self, enumerator: &str) -> String {
        self.0.replace("{}", enumerator)
    }
    pub fn unnumbered(&self) -> String {
        self.0.clone()
    }
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

fn parse_unnumbered_token(input: &str) -> IResult<&str, ListToken> {
    let bullet = 
        map(
            tag("*"),
            |_| ListToken::bullet()
        );
    let plus = 
        map(
            tag("+"),
            |_| ListToken::plus()
        );
    let hyphen = 
        map(
            tag("-"),
            |_| ListToken::hyphen()
        );
    alt((
        bullet, 
        plus, 
        hyphen
    ))(input)
}
fn parse_enumerator(input: &str) -> IResult<&str, TokenEnumerator> {
    let numerical =
        map(
            take_while1(|c: char| c.is_digit(10)),
            |_| TokenEnumerator::Numerical
        );
    let alphabetic = 
        map(
            verify(
                take(1u8),
                |c: &str| {
                    let c = c.chars().nth(0).unwrap();
                    c.is_alphabetic()
                }
            ),
            |a: &str| {
                let ch = a.chars().nth(0).unwrap();
                TokenEnumerator::Alphabetical(ch.is_uppercase())
            }
        );
    let roman =
        map(
            take_while1(|r| Numeral::from_char(r).is_some()),
            |n: &str| {
                let ch = n.chars().nth(0).unwrap();
                TokenEnumerator::Roman(ch.is_uppercase())
            }
        );
    alt((
        numerical,
        roman,
        alphabetic,
    ))(input)
}

fn parse_enumerated_token(input: &str) -> IResult<&str, ListToken> {
    // TODO move all of these to their own functions
    let symbol_terminated = |symbol: &'static str|
        map(
            terminated(
                parse_enumerator,
                tag(symbol)
            ),
            move |e| (format!("{{}}{}", symbol), e)
        );
    let dot_terminated = symbol_terminated(".");
    let hyphen_terminated = symbol_terminated("-");
    let parenthesis_terminated = symbol_terminated(")");
    
    map(
        alt((
            dot_terminated,
            hyphen_terminated,
            parenthesis_terminated
        )),
        |(wrapper, enumerator)| 
            ListToken { 
                wrapper: TokenWrapper(wrapper), 
                enumerator: Some(enumerator) 
            }
    )(input)
}
// this parses the full token.
pub fn parse_item_token(input: &str) -> IResult<&str, ListToken> {
    alt((
        parse_unnumbered_token,
        parse_enumerated_token,
    ))(input)
}
