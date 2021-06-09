use std::result::Result;

use nom::{
    branch::alt,
    bytes::complete::{
        is_not,
        tag,
        tag_no_case,
    },
    character::complete::{
        digit1,
        line_ending,
        multispace0,
        multispace1,
    },
    combinator::{
        map,
        opt,
    },
    multi::{
        fold_many0,
        fold_many1,
    },
    sequence::{
        delimited,
        preceded,
        tuple,
    },
    IResult,
};

use crate::keyword_rule;

use super::{
    Token,
    TokenizeError,
};

pub fn parse_obj(input: &str) -> Result<Vec<Token>, TokenizeError> {
    match fold_many0(
        alt((
            delimited(
                multispace0,
                alt((
                    keyword_rule!("mtllib", MaterialLib),
                    keyword_rule!("usemtl", UseMaterial),
                    keyword_rule!("vt", VertexTexture),
                    keyword_rule!("vn", VertexNormal),
                    keyword_rule!("vp", VertexParam),
                    keyword_rule!("v", Vertex),
                    keyword_rule!("f", Face),
                    keyword_rule!("l", Line),
                    keyword_rule!("p", Point),
                    keyword_rule!("o", Object),
                    keyword_rule!("g", Group),
                    keyword_rule!("s", Smoothing),
                    keyword_rule!("bevel", Bevel),
                    keyword_rule!("c_interp", CInterp),
                    keyword_rule!("d_interp", DInterp),
                    keyword_rule!("lod", Lod),
                    keyword_rule!("shadow_obj", ShadowObj),
                    keyword_rule!("trace_obj", TraceObj),
                    keyword_rule!("maplib", TextureMapLib),
                    keyword_rule!("usemap", UseTextureMap),
                )),
                multispace1,
            ),
            map(tag("/"), |_| Token::Slash),
            super::parse_float,
            super::parse_digit,
            map(preceded(tag("#"), is_not("\r\n")), |_| Token::Ignore),
            map(alt((line_ending, multispace1)), |_| Token::Ignore),
            map(is_not("\r\n"), |s: &str| Token::String(s.to_string())),
        )),
        Vec::new(),
        |mut acc: Vec<Token>, item| {
            if item != Token::Ignore {
                acc.push(item);
            }
            acc
        },
    )(input)
    {
        Ok((_, v)) => Ok(v),
        Err(e) => Err(TokenizeError::Parse(e.to_string())),
    }
}
