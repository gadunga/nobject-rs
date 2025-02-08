macro_rules! token_match {
    ($($token:tt)*) => {{
        fn inner() -> impl Fn(crate::tokenizer::TokenSet) -> IResult<crate::tokenizer::TokenSet, Token> {
            move |input: crate::tokenizer::TokenSet| -> IResult<crate::tokenizer::TokenSet, Token> {
                if input.is_empty() {
                    Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Eof,
                    )))
                } else if matches!(input.as_ref()[0], $($token)*) {
                    let token = input.as_ref()[0].clone();
                    let (_, remainder) = input.split_at(1);
                    Ok((remainder, token))
                } else {
                    Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Tag,
                    )))
                }
            }
        }
        inner()
    }};
}
