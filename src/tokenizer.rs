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

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Ignore,
    String(String),
    Float(f32),
    Int(i32),
    /// List of geometric vertices, with (x, y, z [,w]) coordinates, w is
    /// optional and defaults to 1.0.
    Vertex,
    /// List of vertex normals in (x,y,z) form; normals might not be unit
    /// vectors.
    VertexNormal,
    /// List of texture coordinates, in (u, [,v ,w]) coordinates, these will
    /// vary between 0 and 1. v, w are optional and default to 0.
    VertexTexture,
    /// Parameter space vertices in ( u [,v] [,w] ) form; free form geometry
    /// statement
    VertexParam,
    /// Polygonal face element
    /// f v v v
    /// f v/vt v/vt v/vt
    /// f v/vt/vn v/vt/vn v/vt/vn
    /// f v//vn v//vn v//vn
    Face,
    /// Line element
    /// l v v v v ...
    MaterialLib,
    UseMaterial,
    NewMaterial,
    Object,
    Group,
    Shading,
    Line,
    Slash,
}

pub fn parse(input: &str) -> IResult<&str, Vec<Token>> {
    fold_many0(
        alt((
            delimited(
                multispace0,
                alt((
                    map(tag_no_case("mtllib"), |_| Token::MaterialLib),
                    map(tag_no_case("usemtl"), |_| Token::UseMaterial),
                    map(tag_no_case("vt"), |_| Token::VertexTexture),
                    map(tag_no_case("vn"), |_| Token::VertexNormal),
                    map(tag_no_case("vp"), |_| Token::VertexParam),
                    map(tag_no_case("v"), |_| Token::Vertex),
                    map(tag_no_case("f"), |_| Token::Face),
                    map(tag_no_case("l"), |_| Token::Line),
                    map(tag_no_case("o"), |_| Token::Object),
                    map(tag_no_case("g"), |_| Token::Group),
                    map(tag_no_case("s"), |_| Token::Shading),
                )),
                multispace1,
            ),
            map(tag("/"), |_| Token::Slash),
            parse_float,
            parse_digit,
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
}

pub fn parse_digit(input: &str) -> IResult<&str, Token> {
    map(
        tuple((
            opt(alt((tag("+"), tag("-")))),
            fold_many1(digit1, Vec::new(), |mut acc: Vec<_>, item| {
                acc.push(item);
                acc
            }),
        )),
        |(sign, s): (Option<&str>, Vec<&str>)| {
            let mut val = s.join("").parse::<i32>().unwrap_or_default();
            if sign == Some("-") {
                val = val * -1;
            }
            Token::Int(val)
        },
    )(input)
}

pub fn parse_float(input: &str) -> IResult<&str, Token> {
    map(
        tuple((
            opt(alt((tag("+"), tag("-")))),
            alt((
                map(
                    tuple((
                        fold_many1(digit1, Vec::new(), |mut acc: Vec<_>, item| {
                            acc.push(item);
                            acc
                        }),
                        tag("."),
                        opt(fold_many1(digit1, Vec::new(), |mut acc: Vec<_>, item| {
                            acc.push(item);
                            acc
                        })),
                    )),
                    |(f, _, s)| (f, s.unwrap_or_default()),
                ),
                map(
                    tuple((
                        opt(fold_many1(digit1, Vec::new(), |mut acc: Vec<_>, item| {
                            acc.push(item);
                            acc
                        })),
                        tag("."),
                        fold_many1(digit1, Vec::new(), |mut acc: Vec<_>, item| {
                            acc.push(item);
                            acc
                        }),
                    )),
                    |(f, _, s)| (f.unwrap_or_default(), s),
                ),
            )),
        )),
        |(sign, (f, s)): (Option<&str>, (Vec<&str>, Vec<&str>))| {
            let mut acc = Vec::new();
            if !f.is_empty() {
                acc.extend(f);
            }
            acc.push(".");
            if !s.is_empty() {
                acc.extend(s);
            }
            let mut val = acc.join("").parse::<f32>().unwrap_or_default();
            if sign == Some("-") {
                val = val * -1.0;
            }
            Token::Float(val)
        },
    )(input)
}
