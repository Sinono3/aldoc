/// Takes characters until the parser matches
pub fn take_until_match<'a, P, O, E: nom::error::ParseError<&'a str>>(parser: P) 
-> impl Fn(&'a str) -> IResult<&'a str, &'a str, E> 
    where P: Fn(&'a str) -> IResult<&'a str, O, E>
{
    move |input: &str| 
        recognize(
            many1_count(
                not(&parser)
            )
        )(input)
}
