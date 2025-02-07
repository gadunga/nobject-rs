use std::borrow::Cow;
use std::result::Result;

use nom::{
    branch::alt,
    bytes::{is_not, tag, tag_no_case, take_till},
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

pub fn parse_mtl(input: &str) -> Result<TokenSet, TokenizeError> {
    match fold_many0(
        alt((
            delimited(
                multispace0(),
                map(
                    alt([
                        tag_no_case("newmtl"),
                        tag_no_case("spectral"),
                        tag_no_case("xyz"),
                        tag_no_case("sharpness"),
                        tag_no_case("illum"),
                        tag_no_case("map_disp"),
                        tag_no_case("map_Ka"),
                        tag_no_case("map_Kd"),
                        tag_no_case("map_Ks"),
                        tag_no_case("map_Ns"),
                        tag_no_case("map_aat"),
                        tag_no_case("map_d"),
                        tag_no_case("disp"),
                        tag_no_case("decal"),
                        tag_no_case("bump"),
                        tag_no_case("refl"),
                        tag_no_case("-halo"),
                        tag_no_case("-type"),
                        tag_no_case("-texres"),
                        tag_no_case("-blendu"),
                        tag_no_case("-blendv"),
                        tag_no_case("-boost"),
                        tag_no_case("-clamp"),
                        tag_no_case("-imfchan"),
                        tag_no_case("-bm"),
                        tag_no_case("-cc"),
                        tag_no_case("-mm"),
                        tag_no_case("-o"),
                        tag_no_case("-s"),
                        tag_no_case("-t"),
                        tag_no_case("ka"),
                        tag_no_case("kd"),
                        tag_no_case("ks"),
                        tag_no_case("ke"),
                        tag_no_case("ns"),
                        tag_no_case("tr"),
                        tag_no_case("Tf"),
                        tag_no_case("Ni"),
                        tag_no_case("d"),
                    ]),
                    kw_map,
                ),
                map(multispace1, |_| Token::Ignore),
            ),
            super::parse_float(),
            super::parse_digit(),
            map(
                preceded(tag("#"), take_till(|c| c == '\n' || c == '\r')),
                |_| Token::Ignore,
            ),
            map(alt((line_ending, multispace1)), |_| Token::Ignore),
            map(is_not(" \r\n"), |s: &str| Token::String(Cow::Borrowed(s))),
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
        "newmtl" => Token::NewMaterial,
        "ka" => Token::AmbientColor,
        "kd" => Token::DiffuseColor,
        "ks" => Token::SpecularColor,
        "ke" => Token::EmissiveCoefficient,
        "ns" => Token::SpecularExponent,
        "tr" => Token::Transparancy,
        "spectral" => Token::Spectral,
        "xyz" => Token::Xyz,
        "d" => Token::Disolved,
        "-halo" => Token::Halo,
        "tf" => Token::TransmissionFactor,
        "sharpness" => Token::Sharpness,
        "ni" => Token::IndexOfRefraction,
        "illum" => Token::IlluminationModel,
        "map_disp" => Token::DisplacementMap,
        "map_ka" => Token::TextureMapAmbient,
        "map_kd" => Token::TextureMapDiffuse,
        "map_ks" => Token::TextureMapSpecular,
        "map_ns" => Token::TextureMapShininess,
        "map_aat" => Token::AntiAliasMap,
        "map_d" => Token::TextureMapDisolved,
        "disp" => Token::DisplacementMap,
        "decal" => Token::Decal,
        "bump" => Token::BumpMap,
        "refl" => Token::ReflectionMap,
        "-type" => Token::ReflectionType,
        "-texres" => Token::OptionTextureResolution,
        "-blendu" => Token::OptionBlendU,
        "-blendv" => Token::OptionBlendV,
        "-bm" => Token::OptionBumpMultiplier,
        "-boost" => Token::OptionBoost,
        "-cc" => Token::OptionColorCorrect,
        "-clamp" => Token::OptionClamp,
        "-imfchan" => Token::OptionIMFChan,
        "-mm" => Token::OptionRange,
        "-o" => Token::OptionOffset,
        "-s" => Token::OptionScale,
        "-t" => Token::OptionTurbulence,
        _ => Token::Ignore,
    }
}
