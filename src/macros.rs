#[macro_export]
macro_rules! token_match {
    ($($token:tt)*) => {{
        fn inner() -> impl Fn(&[Token]) -> IResult<&[Token], Token> {
            move |input: &[Token]| -> IResult<&[Token], Token> {
                if input.is_empty() {
                    Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Eof,
                    )))
                } else if matches!(input[0], $($token)*) {
                    let token = input[0].clone();
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
