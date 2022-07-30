use std::result::Result;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, tag_no_case, take_till},
    character::complete::{line_ending, multispace0, multispace1},
    combinator::map,
    multi::fold_many0,
    sequence::{delimited, preceded},
};

use super::{Token, TokenizeError};

pub fn parse_mtl(input: &str) -> Result<Vec<Token>, TokenizeError> {
    match fold_many0(
        alt((
            delimited(
                multispace0,
                alt((
                    alt((
                        keyword_rule!("newmtl", NewMaterial),
                        keyword_rule!("ka", AmbientColor),
                        keyword_rule!("kd", DiffuseColor),
                        keyword_rule!("ks", SpecularColor),
                        keyword_rule!("ke", EmissiveCoefficient),
                        keyword_rule!("ns", SpecularExponent),
                        keyword_rule!("tr", Transparancy),
                        keyword_rule!("spectral", Spectral),
                        keyword_rule!("xyz", Xyz),
                        keyword_rule!("d", Disolved),
                        keyword_rule!("-halo", Halo),
                        keyword_rule!("Tf", TransmissionFactor),
                        keyword_rule!("sharpness", Sharpness),
                        keyword_rule!("Ni", IndexOfRefraction),
                        keyword_rule!("illum", IlluminationModel),
                        keyword_rule!("map_disp", DisplacementMap),
                        keyword_rule!("map_Ka", TextureMapAmbient),
                        keyword_rule!("map_Kd", TextureMapDiffuse),
                        keyword_rule!("map_Ks", TextureMapSpecular),
                        keyword_rule!("map_Ns", TextureMapShininess),
                        keyword_rule!("map_aat", AntiAliasMap),
                    )),
                    alt((
                        keyword_rule!("map_d", TextureMapDisolved),
                        keyword_rule!("disp", DisplacementMap),
                        keyword_rule!("decal", Decal),
                        keyword_rule!("bump", BumpMap),
                        keyword_rule!("refl", ReflectionMap),
                        keyword_rule!("-type", ReflectionType),
                        keyword_rule!("-texres", OptionTextureResolution),
                        keyword_rule!("-blendu", OptionBlendU),
                        keyword_rule!("-blendv", OptionBlendV),
                        keyword_rule!("-bm", OptionBumpMultiplier),
                        keyword_rule!("-boost", OptionBoost),
                        keyword_rule!("-cc", OptionColorCorrect),
                        keyword_rule!("-clamp", OptionClamp),
                        keyword_rule!("-imfchan", OptionIMFChan),
                        keyword_rule!("-mm", OptionRange),
                        keyword_rule!("-o", OptionOffset),
                        keyword_rule!("-s", OptionScale),
                        keyword_rule!("-t", OptionTurbulence),
                    )),
                )),
                multispace1,
            ),
            super::parse_float,
            super::parse_digit,
            map(
                preceded(tag("#"), take_till(|c| c == '\n' || c == '\r')),
                |_| Token::Ignore,
            ),
            map(alt((line_ending, multispace1)), |_| Token::Ignore),
            map(is_not(" \r\n"), |s: &str| Token::String(s.to_string())),
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
