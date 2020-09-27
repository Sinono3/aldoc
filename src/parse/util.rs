use nom::{
    IResult,
    combinator::{not, recognize},
    multi::many1_count,
    sequence::preceded,
    bytes::complete::take,
};


/// Takes characters until the parser matches
//pub fn take_until_match<'a, P, O, E: nom::error::ParseError<&'a str>>(parser: P) 
//-> impl Fn(&'a str) -> IResult<&'a str, &'a str, E> 
//    where P: Fn(&'a str) -> IResult<&'a str, O, E>
//{
//    move |input: &str| 
//        recognize(
//            many1_count(
//                preceded(
//                    not(&parser),
//                    take(1u8)
//                )
//            )
//        )(input)
//}
/// Takes characters until the parser matches
pub fn take_until_match<'a, P, O, E: nom::error::ParseError<&'a str>>(parser: P) 
-> impl Fn(&'a str) -> IResult<&'a str, &'a str, E> 
    where P: Fn(&'a str) -> IResult<&'a str, O, E>
{
    move |input: &str| 
        recognize(
            many1_count(
                preceded(
                    not(&parser),
                    take(1u8)
                )
            )
        )(input)
}
