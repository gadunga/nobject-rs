use std::borrow::Cow;
use std::result::Result;

use nom::{
    branch::alt,
    bytes::{complete::is_not, tag, tag_no_case, take_till},
    character::{
        complete::{line_ending, multispace1},
        multispace0,
    },
    combinator::map,
    multi::fold_many0,
    sequence::{delimited, preceded},
    Parser,
};

use super::{Token, TokenSet, TokenizeError};

pub fn parse_obj(input: &str) -> Result<TokenSet, TokenizeError> {
    match fold_many0(
        alt((
            delimited(
                multispace0(),
                map(
                    alt([
                        tag_no_case("mtllib"),
                        tag_no_case("usemtl"),
                        tag_no_case("bevel"),
                        tag_no_case("c_interp"),
                        tag_no_case("d_interp"),
                        tag_no_case("lod"),
                        tag_no_case("shadow_obj"),
                        tag_no_case("trace_obj"),
                        tag_no_case("maplib"),
                        tag_no_case("usemap"),
                        tag_no_case("vt"),
                        tag_no_case("vn"),
                        tag_no_case("vp"),
                        tag_no_case("v"),
                        tag_no_case("f"),
                        tag_no_case("l"),
                        tag_no_case("p"),
                        tag_no_case("o"),
                        tag_no_case("g"),
                        tag_no_case("s"),
                    ]),
                    kw_map,
                ),
                map(multispace1, |_| Token::Ignore),
            ),
            map(tag("/"), |_| Token::Slash),
            super::parse_float(),
            super::parse_digit(),
            map(
                preceded(tag("#"), take_till(|c| c == '\n' || c == '\r')),
                |_| Token::Ignore,
            ),
            map(alt((line_ending, multispace1)), |_| Token::Ignore),
            map(is_not("\r\n"), |s: &str| Token::String(Cow::Borrowed(s))),
        )),
        Vec::new,
        |mut acc: Vec<Token>, item| {
            if item != Token::Ignore {
                acc.push(item);
            }
            acc
        },
    )
    .parse_complete(input)
    {
        Ok((_, v)) => Ok(v.into()),
        Err(e) => Err(TokenizeError::Parse(e.to_string())),
    }
}

fn kw_map(value: &str) -> Token<'_> {
    match value.to_lowercase().as_ref() {
        "mtllib" => Token::MaterialLib,
        "usemtl" => Token::UseMaterial,
        "vt" => Token::VertexTexture,
        "vn" => Token::VertexNormal,
        "vp" => Token::VertexParam,
        "v" => Token::Vertex,
        "f" => Token::Face,
        "l" => Token::Line,
        "p" => Token::Point,
        "o" => Token::Object,
        "g" => Token::Group,
        "s" => Token::Smoothing,
        "bevel" => Token::Bevel,
        "c_interp" => Token::CInterp,
        "d_interp" => Token::DInterp,
        "lod" => Token::Lod,
        "shadow_obj" => Token::ShadowObj,
        "trace_obj" => Token::TraceObj,
        "maplib" => Token::TextureMapLib,
        "usemap" => Token::UseTextureMap,
        _ => Token::Ignore,
    }
}
