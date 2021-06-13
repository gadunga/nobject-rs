use std::result::Result;

use crate::{
    get_on_off_from_str,
    get_opt_token_float_opt,
    get_token_float,
    get_token_int,
    get_token_string,
    tokenizer::Token,
};
use nom::{
    branch::alt,
    combinator::{
        map,
        opt,
    },
    multi::many1,
    sequence::{
        preceded,
        tuple,
    },
    IResult,
};
use thiserror::Error;

/// An enum for possible ways of specifying a material color
#[derive(Debug, Clone, PartialEq)]
pub enum ColorType {
    /// RGB
    Rgb(f32, f32, f32),
    /// Reflectivity using a spectral curve.
    /// This is specified as a filename and a multiplier (defaults to 1.0)
    Spectral(String, f32),
    /// CIEXYZ color space
    CieXyz(f32, f32, f32),
}

/// Enum for the possible ways to specify the disolve
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DisolveType {
    /// The amount this material dissolves into the background. 1.0 is fully
    /// opaque
    Alpha(f32),
    /// Specifies that the disolve is based on the orientation of the viewer.
    /// The value is the minimum to apply to a material.
    Halo(f32),
}

#[derive(Clone, Debug)]
enum OptionElement {
    FileName(String),
    BlendU(bool),
    BlendV(bool),
    Cc(bool),
    Clamp(bool),
    TextureRange((f32, f32)),
    Offset((f32, Option<f32>, Option<f32>)),
    Scale((f32, Option<f32>, Option<f32>)),
    Turbulance((f32, Option<f32>, Option<f32>)),
    TextureRes(i32),
    ImfChan(String),
    BumpMultiplier(f32),
    ReflectionType(String),
}

/// Common settings for texture maps which can be color corrected.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ColorCorrectedMap {
    /// The name of the texture map file.
    pub file_name:     String,
    /// Enable horizontal texture blending
    pub blend_u:       Option<bool>,
    /// Enable vertical texture blending
    pub blend_v:       Option<bool>,
    /// Enable color correction
    pub color_correct: Option<bool>,
    /// Enables clamping.
    pub clamp:         Option<bool>,
    /// Specifies the range over which scalar or color texture
    /// values may vary. Corresponds to the `-mm` option.
    pub texture_range: Option<(f32, f32)>,
    /// Offset the position in the texture map.
    pub offset:        Option<(f32, Option<f32>, Option<f32>)>,
    /// Scale the size of the texture pattern.
    pub scale:         Option<(f32, Option<f32>, Option<f32>)>,
    /// A turbulance value to apply to the texture.
    pub turbulance:    Option<(f32, Option<f32>, Option<f32>)>,
    /// Allows the specification of a specific resolution to use
    /// when an image is used as a texture.
    pub texture_res:   Option<i32>,
}

impl ColorCorrectedMap {
    fn new(o: &[OptionElement]) -> Self {
        let mut res = Self::default();
        for e in o {
            match e {
                OptionElement::FileName(n) => res.file_name = n.clone(),
                OptionElement::BlendU(b) => {
                    res.blend_u = Some(*b);
                },
                OptionElement::BlendV(b) => {
                    res.blend_v = Some(*b);
                },
                OptionElement::Cc(b) => {
                    res.color_correct = Some(*b);
                },
                OptionElement::Clamp(b) => {
                    res.clamp = Some(*b);
                },
                OptionElement::TextureRange((base, gain)) => {
                    res.texture_range = Some((*base, *gain));
                },
                OptionElement::Offset((x, y, z)) => {
                    res.offset = Some((*x, *y, *z));
                },
                OptionElement::Scale((x, y, z)) => {
                    res.scale = Some((*x, *y, *z));
                },
                OptionElement::Turbulance((x, y, z)) => {
                    res.turbulance = Some((*x, *y, *z));
                },
                OptionElement::TextureRes(tex_res) => {
                    res.texture_res = Some(*tex_res);
                },
                _ => {},
            }
        }
        res
    }
}

/// Common settings for texture maps which can not be color corrected.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NonColorCorrectedMap {
    /// The name of the texture map file.
    pub file_name:     String,
    /// Enable horizontal texture blending
    pub blend_u:       Option<bool>,
    /// Enable vertical texture blending
    pub blend_v:       Option<bool>,
    /// Enables clamping.
    pub clamp:         Option<bool>,
    /// Specifies the channel used to create a scalar or
    /// bump texture.
    pub imf_chan:      Option<String>,
    /// Specifies the range over which scalar or color texture
    /// values may vary. Corresponds to the `-mm` option.
    pub texture_range: Option<(f32, f32)>,
    /// Offset the position in the texture map.
    pub offset:        Option<(f32, Option<f32>, Option<f32>)>,
    /// Scale the size of the texture pattern.
    pub scale:         Option<(f32, Option<f32>, Option<f32>)>,
    /// A turbulance value to apply to the texture.
    pub turbulance:    Option<(f32, Option<f32>, Option<f32>)>,
    /// Allows the specification of a specific resolution to use
    /// when an image is used as a texture.
    pub texture_res:   Option<i32>,
}

impl NonColorCorrectedMap {
    fn new(o: &[OptionElement]) -> Self {
        let mut res = Self::default();
        for e in o {
            match e {
                OptionElement::FileName(n) => res.file_name = n.clone(),
                OptionElement::BlendU(b) => {
                    res.blend_u = Some(*b);
                },
                OptionElement::BlendV(b) => {
                    res.blend_v = Some(*b);
                },
                OptionElement::Clamp(b) => {
                    res.clamp = Some(*b);
                },
                OptionElement::ImfChan(chan) => res.imf_chan = Some(chan.clone()),
                OptionElement::TextureRange((base, gain)) => {
                    res.texture_range = Some((*base, *gain));
                },
                OptionElement::Offset((x, y, z)) => {
                    res.offset = Some((*x, *y, *z));
                },
                OptionElement::Scale((x, y, z)) => {
                    res.scale = Some((*x, *y, *z));
                },
                OptionElement::Turbulance((x, y, z)) => {
                    res.turbulance = Some((*x, *y, *z));
                },
                OptionElement::TextureRes(tex_res) => {
                    res.texture_res = Some(*tex_res);
                },
                _ => {},
            }
        }
        res
    }
}

/// Contains information specific to bump maps.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BumpMap {
    /// Specifies a bump multiplier
    pub bump_multiplier: Option<f32>,
    /// Additional map settings.
    pub map_settings:    Option<NonColorCorrectedMap>,
}

impl BumpMap {
    fn new(o: &[OptionElement]) -> Self {
        let mut res = Self {
            map_settings: Some(NonColorCorrectedMap::new(&o)),
            ..Default::default()
        };

        for e in o {
            if let OptionElement::BumpMultiplier(bm) = e {
                res.bump_multiplier = Some(*bm);
                break;
            }
        }
        res
    }
}

/// Reflection specific information.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ReflectionMap {
    /// This contains the name of the type of reflection to use.
    /// Corresponds to `-type` in the specification.
    pub reflection_type: String,
    /// Additional map settings.
    pub map_settings:    Option<ColorCorrectedMap>,
}

impl ReflectionMap {
    fn new(o: &[OptionElement]) -> Self {
        let mut res = Self {
            map_settings: Some(ColorCorrectedMap::new(&o)),
            ..Default::default()
        };

        for e in o {
            if let OptionElement::ReflectionType(ty) = e {
                res.reflection_type = ty.clone();
                break;
            }
        }
        res
    }
}

/// Defines a single material.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Material {
    /// The name of the material.
    /// Corresponds to `newmtl` in the specification.
    pub name:                 String,
    /// The ambient reflectivity value.
    /// Corresponds to `Ka` in the specification.
    pub ambient:              Option<ColorType>,
    /// The diffuse reflectivity value
    /// Corresponds to `Kd` in the specification.
    pub diffuse:              Option<ColorType>,
    /// The specular reflectivity value
    /// Corresponds to `Ks` in the specification.
    pub specular:             Option<ColorType>,
    /// The specular exponent.
    /// Corresponds to `Ns` in the specification.
    pub specular_exponent:    Option<f32>,
    /// The disolve.
    /// Corresponds to `d` in the specification.
    pub disolve:              Option<DisolveType>,
    /// Transparancy.
    /// Corresponds to `Tr` in the specification.
    pub transparancy:         Option<f32>,
    /// Transmission factor.
    /// Corresponds to `Tf` in the specification.
    pub transmission_factor:  Option<ColorType>,
    /// Corresponds to `sharpness` in the specification.
    pub sharpness:            Option<f32>,
    /// Corresponds to `Ni` in the specification.
    pub index_of_refraction:  Option<f32>,
    /// Corresponds to `illum` in the specification.
    pub illumination_mode:    Option<u32>,
    /// Corresponds to `map_Ka` in the specification.
    pub texture_map_ambient:  Option<ColorCorrectedMap>,
    /// Corresponds to `map_Kd` in the specification.
    pub texture_map_diffuse:  Option<ColorCorrectedMap>,
    /// Corresponds to `map_Ks` in the specification.
    pub texture_map_specular: Option<ColorCorrectedMap>,
    /// Corresponds to `map_Ns` in the specification.
    pub shininess_map:        Option<NonColorCorrectedMap>,
    /// Corresponds to `map_d` in the specification.
    pub disolve_map:          Option<NonColorCorrectedMap>,
    /// Corresponds to `disp` in the specification.
    pub displacement_map:     Option<NonColorCorrectedMap>,
    /// Corresponds to `decal` in the specification.
    pub decal:                Option<NonColorCorrectedMap>,
    /// Corresponds to `bump` in the specification.
    pub bump_map:             Option<BumpMap>,
    /// Corresponds to `refl` in the specification.
    pub reflection_map:       Option<ReflectionMap>,
    /// Enables/Disables anti-aliasing of textures in THIS material only.
    /// Corresponds to `map_aat` in the specification.
    pub anti_alias_map:       Option<bool>,
}

impl Material {
    fn set_from_material_element(&mut self, element: &MaterialElement) {
        match element {
            MaterialElement::Name(n) => {
                self.name = n.clone();
            },
            MaterialElement::Ambient(c) => {
                self.ambient = Some(c.clone());
            },
            MaterialElement::Diffuse(c) => {
                self.diffuse = Some(c.clone());
            },
            MaterialElement::Specular(c) => {
                self.specular = Some(c.clone());
            },
            MaterialElement::SpecularExponent(f) => {
                self.specular_exponent = Some(*f);
            },
            MaterialElement::Disolve(d) => {
                self.disolve = Some(*d);
            },
            MaterialElement::Transparency(f) => {
                self.transparancy = Some(*f);
            },
            MaterialElement::TransmissionFactor(c) => {
                self.transmission_factor = Some(c.clone());
            },
            MaterialElement::Sharpness(f) => {
                self.sharpness = Some(*f);
            },
            MaterialElement::IndexOfRefraction(f) => {
                self.index_of_refraction = Some(*f);
            },
            MaterialElement::IlluminationModel(u) => {
                self.illumination_mode = Some(*u);
            },
            MaterialElement::TexMapAmbient(cc) => {
                self.texture_map_ambient = Some(cc.clone());
            },
            MaterialElement::TexMapDiffuse(cc) => {
                self.texture_map_diffuse = Some(cc.clone());
            },
            MaterialElement::TexMapSpecular(cc) => {
                self.texture_map_specular = Some(cc.clone());
            },
            MaterialElement::ShininessMap(ncc) => {
                self.shininess_map = Some(ncc.clone());
            },
            MaterialElement::DisolveMap(ncc) => {
                self.disolve_map = Some(ncc.clone());
            },
            MaterialElement::DisplacementMap(ncc) => {
                self.displacement_map = Some(ncc.clone());
            },
            MaterialElement::Decal(ncc) => {
                self.decal = Some(ncc.clone());
            },
            MaterialElement::BumpMap(bm) => {
                self.bump_map = Some(bm.clone());
            },
            MaterialElement::ReflectionMap(rm) => {
                self.reflection_map = Some(rm.clone());
            },
            MaterialElement::AntiAliasMap(b) => {
                self.anti_alias_map = Some(*b);
            },
        }
    }
}

/// A wrapper for an underlying error which occurred
/// while parsing the token stream.
#[derive(Error, Debug)]
pub enum MaterialError {
    #[error("Parse Error: `{0}`")]
    Parse(String),

    /// The specification generally requires a newmtl statement
    /// to come before all other statements. If this error occurs
    /// it's because we also expect a newmtl statement first.
    #[error("New Material expected, but not found.")]
    NewMaterial,
}

#[derive(Clone, Debug)]
enum MaterialElement {
    Name(String),
    Ambient(ColorType),
    Diffuse(ColorType),
    Specular(ColorType),
    SpecularExponent(f32),
    Disolve(DisolveType),
    Transparency(f32),
    TransmissionFactor(ColorType),
    Sharpness(f32),
    IndexOfRefraction(f32),
    IlluminationModel(u32),
    TexMapAmbient(ColorCorrectedMap),
    TexMapDiffuse(ColorCorrectedMap),
    TexMapSpecular(ColorCorrectedMap),
    ShininessMap(NonColorCorrectedMap),
    DisolveMap(NonColorCorrectedMap),
    DisplacementMap(NonColorCorrectedMap),
    Decal(NonColorCorrectedMap),
    BumpMap(BumpMap),
    ReflectionMap(ReflectionMap),
    AntiAliasMap(bool),
}

pub(crate) fn parse(input: &[Token]) -> Result<Vec<Material>, MaterialError> {
    let elements: Vec<MaterialElement> = match many1(alt((
        parse_new_material,
        parse_ambient,
        parse_diffuse,
        parse_specular,
        parse_specular_exponent,
        parse_disolve,
        parse_transparency,
        parse_transmission_factor,
        parse_sharpness,
        parse_index_of_refraction,
        parse_illumination_model,
        parse_texture_map_ambient,
        parse_texture_map_diffuse,
        parse_texture_map_specular,
        parse_shininess_map,
        parse_disolve_map,
        parse_displacement_map,
        parse_decal,
        parse_bump_map,
        parse_reflection_map,
        parse_anti_alias_map,
    )))(input)
    {
        Ok((_, x)) => x,
        Err(e) => return Err(MaterialError::Parse(e.to_string())),
    };

    let mut res = Vec::new();
    dbg!(&elements);
    for e in elements {
        if let MaterialElement::Name(n) = e {
            res.push(Material::default());
            if let Some(l) = res.last_mut() {
                l.name = n;
            }
        } else if let Some(l) = res.last_mut() {
            l.set_from_material_element(&e);
        } else {
            return Err(MaterialError::NewMaterial);
        }
    }
    Ok(res)
}

fn parse_new_material(input: &[Token]) -> IResult<&[Token], MaterialElement> {
    map(
        preceded(
            token_match!(Token::NewMaterial),
            token_match!(Token::String(_)),
        ),
        |s| {
            let name = match get_token_string(&s) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };
            MaterialElement::Name(name)
        },
    )(input)
}

fn parse_color_type(input: &[Token]) -> IResult<&[Token], ColorType> {
    alt((
        map(
            tuple((
                token_match!(Token::Spectral),
                token_match!(Token::String(_)),
                opt(token_match!(Token::Float(_) | Token::Int(_))),
            )),
            |(_, file, factor)| {
                let file_name = match get_token_string(&file) {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!("{}", e);
                        Default::default()
                    },
                };
                let factor = match get_opt_token_float_opt(&factor) {
                    Ok(s) => s.unwrap_or(1.0),
                    Err(e) => {
                        log::error!("{}", e);
                        Default::default()
                    },
                };
                ColorType::Spectral(file_name, factor)
            },
        ),
        map(
            tuple((
                token_match!(Token::Xyz),
                token_match!(Token::Float(_) | Token::Int(_)),
                opt(token_match!(Token::Float(_) | Token::Int(_))),
                opt(token_match!(Token::Float(_) | Token::Int(_))),
            )),
            |(_, x_token, y_token, z_token)| {
                let x = match get_token_float(&x_token) {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!("{}", e);
                        Default::default()
                    },
                };
                let y = match y_token {
                    Some(y) => match get_token_float(&y) {
                        Ok(s) => s,
                        Err(e) => {
                            log::error!("{}", e);
                            Default::default()
                        },
                    },
                    None => x,
                };
                let z = match z_token {
                    Some(z) => match get_token_float(&z) {
                        Ok(s) => s,
                        Err(e) => {
                            log::error!("{}", e);
                            Default::default()
                        },
                    },
                    None => x,
                };

                ColorType::CieXyz(x, y, z)
            },
        ),
        map(
            tuple((
                token_match!(Token::Float(_) | Token::Int(_)),
                token_match!(Token::Float(_) | Token::Int(_)),
                token_match!(Token::Float(_) | Token::Int(_)),
            )),
            |(r, g, b)| {
                let (r, g, b) = (
                    match get_token_float(&r) {
                        Ok(s) => s,
                        Err(e) => {
                            log::error!("{}", e);
                            Default::default()
                        },
                    },
                    match get_token_float(&g) {
                        Ok(s) => s,
                        Err(e) => {
                            log::error!("{}", e);
                            Default::default()
                        },
                    },
                    match get_token_float(&b) {
                        Ok(s) => s,
                        Err(e) => {
                            log::error!("{}", e);
                            Default::default()
                        },
                    },
                );

                ColorType::Rgb(r, g, b)
            },
        ),
    ))(input)
}

fn parse_ambient(input: &[Token]) -> IResult<&[Token], MaterialElement> {
    preceded(
        token_match!(Token::AmbientColor),
        map(parse_color_type, MaterialElement::Ambient),
    )(input)
}

fn parse_diffuse(input: &[Token]) -> IResult<&[Token], MaterialElement> {
    preceded(
        token_match!(Token::DiffuseColor),
        map(parse_color_type, MaterialElement::Diffuse),
    )(input)
}

fn parse_specular(input: &[Token]) -> IResult<&[Token], MaterialElement> {
    preceded(
        token_match!(Token::SpecularColor),
        map(parse_color_type, MaterialElement::Specular),
    )(input)
}

fn parse_specular_exponent(input: &[Token]) -> IResult<&[Token], MaterialElement> {
    preceded(
        token_match!(Token::SpecularExponent),
        map(token_match!(Token::Float(_) | Token::Int(_)), |f| {
            let f = match get_token_float(&f) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };
            MaterialElement::SpecularExponent(f)
        }),
    )(input)
}

fn parse_disolve(input: &[Token]) -> IResult<&[Token], MaterialElement> {
    preceded(
        token_match!(Token::Disolved),
        alt((
            map(
                preceded(
                    token_match!(Token::Halo),
                    token_match!(Token::Float(_) | Token::Int(_)),
                ),
                |f| {
                    let f = match get_token_float(&f) {
                        Ok(s) => s,
                        Err(e) => {
                            log::error!("{}", e);
                            Default::default()
                        },
                    };
                    MaterialElement::Disolve(DisolveType::Halo(f))
                },
            ),
            map(token_match!(Token::Float(_) | Token::Int(_)), |f| {
                let f = match get_token_float(&f) {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!("{}", e);
                        Default::default()
                    },
                };
                MaterialElement::Disolve(DisolveType::Alpha(f))
            }),
        )),
    )(input)
}

fn parse_transparency(input: &[Token]) -> IResult<&[Token], MaterialElement> {
    preceded(
        token_match!(Token::Transparancy),
        map(token_match!(Token::Float(_)), |f| {
            let f = match get_token_float(&f) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };
            MaterialElement::Transparency(f)
        }),
    )(input)
}

fn parse_transmission_factor(input: &[Token]) -> IResult<&[Token], MaterialElement> {
    preceded(
        token_match!(Token::TransmissionFactor),
        map(parse_color_type, MaterialElement::TransmissionFactor),
    )(input)
}

fn parse_sharpness(input: &[Token]) -> IResult<&[Token], MaterialElement> {
    preceded(
        token_match!(Token::Sharpness),
        map(token_match!(Token::Float(_)), |f| {
            let f = match get_token_float(&f) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };
            MaterialElement::Sharpness(f)
        }),
    )(input)
}

fn parse_index_of_refraction(input: &[Token]) -> IResult<&[Token], MaterialElement> {
    preceded(
        token_match!(Token::IndexOfRefraction),
        map(token_match!(Token::Float(_)), |f| {
            let f = match get_token_float(&f) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };
            MaterialElement::IndexOfRefraction(f)
        }),
    )(input)
}

fn parse_illumination_model(input: &[Token]) -> IResult<&[Token], MaterialElement> {
    preceded(
        token_match!(Token::IlluminationModel),
        map(token_match!(Token::Int(_)), |f| {
            let f = match get_token_int(&f) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };
            MaterialElement::IlluminationModel(f as u32)
        }),
    )(input)
}

fn parse_texture_map_ambient(input: &[Token]) -> IResult<&[Token], MaterialElement> {
    preceded(
        token_match!(Token::TextureMapAmbient),
        map(parse_options, |o| {
            MaterialElement::TexMapAmbient(ColorCorrectedMap::new(&o))
        }),
    )(input)
}

fn parse_texture_map_diffuse(input: &[Token]) -> IResult<&[Token], MaterialElement> {
    preceded(
        token_match!(Token::TextureMapDiffuse),
        map(parse_options, |o| {
            MaterialElement::TexMapDiffuse(ColorCorrectedMap::new(&o))
        }),
    )(input)
}

fn parse_texture_map_specular(input: &[Token]) -> IResult<&[Token], MaterialElement> {
    preceded(
        token_match!(Token::TextureMapSpecular),
        map(parse_options, |o| {
            MaterialElement::TexMapSpecular(ColorCorrectedMap::new(&o))
        }),
    )(input)
}

fn parse_shininess_map(input: &[Token]) -> IResult<&[Token], MaterialElement> {
    preceded(
        token_match!(Token::TextureMapShininess),
        map(parse_options, |o| {
            MaterialElement::ShininessMap(NonColorCorrectedMap::new(&o))
        }),
    )(input)
}

fn parse_disolve_map(input: &[Token]) -> IResult<&[Token], MaterialElement> {
    preceded(
        token_match!(Token::TextureMapDisolved),
        map(parse_options, |o| {
            MaterialElement::DisolveMap(NonColorCorrectedMap::new(&o))
        }),
    )(input)
}

fn parse_displacement_map(input: &[Token]) -> IResult<&[Token], MaterialElement> {
    preceded(
        token_match!(Token::DisplacementMap),
        map(parse_options, |o| {
            MaterialElement::DisplacementMap(NonColorCorrectedMap::new(&o))
        }),
    )(input)
}

fn parse_decal(input: &[Token]) -> IResult<&[Token], MaterialElement> {
    preceded(
        token_match!(Token::Decal),
        map(parse_options, |o| {
            MaterialElement::Decal(NonColorCorrectedMap::new(&o))
        }),
    )(input)
}

fn parse_bump_map(input: &[Token]) -> IResult<&[Token], MaterialElement> {
    preceded(
        token_match!(Token::BumpMap),
        map(parse_options, |o| {
            MaterialElement::BumpMap(BumpMap::new(&o))
        }),
    )(input)
}

fn parse_reflection_map(input: &[Token]) -> IResult<&[Token], MaterialElement> {
    preceded(
        token_match!(Token::ReflectionMap),
        map(parse_options, |o| {
            MaterialElement::ReflectionMap(ReflectionMap::new(&o))
        }),
    )(input)
}

fn parse_anti_alias_map(input: &[Token]) -> IResult<&[Token], MaterialElement> {
    preceded(
        token_match!(Token::AntiAliasMap),
        map(token_match!(Token::String(_)), |o| {
            let val = match get_on_off_from_str(&o) {
                Ok(v) => v,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };
            MaterialElement::AntiAliasMap(val)
        }),
    )(input)
}

fn parse_options(input: &[Token]) -> IResult<&[Token], Vec<OptionElement>> {
    many1(alt((
        parse_option_blend,
        parse_option_bm,
        parse_option_cc,
        parse_option_clamp,
        parse_option_texture_range,
        parse_option_offset,
        parse_option_scale,
        parse_option_turbulance,
        parse_option_texture_resolution,
        parse_option_imf_channel,
        parse_option_reflection_type,
        map(token_match!(Token::String(_)), |s| {
            let name = match get_token_string(&s) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };
            OptionElement::FileName(name)
        }),
    )))(input)
}

fn parse_option_blend(input: &[Token]) -> IResult<&[Token], OptionElement> {
    alt((
        map(
            preceded(
                token_match!(Token::OptionBlendU),
                token_match!(Token::String(_)),
            ),
            |s| {
                let val = match get_on_off_from_str(&s) {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!("{}", e);
                        Default::default()
                    },
                };
                OptionElement::BlendU(val)
            },
        ),
        map(
            preceded(
                token_match!(Token::OptionBlendV),
                token_match!(Token::String(_)),
            ),
            |s| {
                let val = match get_on_off_from_str(&s) {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!("{}", e);
                        Default::default()
                    },
                };
                OptionElement::BlendV(val)
            },
        ),
    ))(input)
}

fn parse_option_bm(input: &[Token]) -> IResult<&[Token], OptionElement> {
    map(
        preceded(
            token_match!(Token::OptionBumpMultiplier),
            token_match!(Token::Float(_)),
        ),
        |s| {
            let val = match get_token_float(&s) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };
            OptionElement::BumpMultiplier(val)
        },
    )(input)
}

fn parse_option_cc(input: &[Token]) -> IResult<&[Token], OptionElement> {
    map(
        preceded(
            token_match!(Token::OptionColorCorrect),
            token_match!(Token::String(_)),
        ),
        |s| {
            let val = match get_on_off_from_str(&s) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };
            OptionElement::Cc(val)
        },
    )(input)
}

fn parse_option_clamp(input: &[Token]) -> IResult<&[Token], OptionElement> {
    map(
        preceded(
            token_match!(Token::OptionClamp),
            token_match!(Token::String(_)),
        ),
        |s| {
            let val = match get_on_off_from_str(&s) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };
            OptionElement::Clamp(val)
        },
    )(input)
}

fn parse_option_texture_range(input: &[Token]) -> IResult<&[Token], OptionElement> {
    map(
        preceded(
            token_match!(Token::OptionRange),
            tuple((token_match!(Token::Float(_)), token_match!(Token::Float(_)))),
        ),
        |(base, gain)| {
            let base = match get_token_float(&base) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };
            let gain = match get_token_float(&gain) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };
            OptionElement::TextureRange((base, gain))
        },
    )(input)
}

fn parse_option_offset(input: &[Token]) -> IResult<&[Token], OptionElement> {
    map(
        preceded(
            token_match!(Token::OptionOffset),
            tuple((
                token_match!(Token::Float(_)),
                opt(token_match!(Token::Float(_))),
                opt(token_match!(Token::Float(_))),
            )),
        ),
        |(x, y, z)| {
            let x = match get_token_float(&x) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };
            let y = match get_opt_token_float_opt(&y) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    None
                },
            };
            let z = match get_opt_token_float_opt(&z) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    None
                },
            };
            OptionElement::Offset((x, y, z))
        },
    )(input)
}

fn parse_option_scale(input: &[Token]) -> IResult<&[Token], OptionElement> {
    map(
        preceded(
            token_match!(Token::OptionScale),
            tuple((
                token_match!(Token::Float(_)),
                opt(token_match!(Token::Float(_))),
                opt(token_match!(Token::Float(_))),
            )),
        ),
        |(x, y, z)| {
            let x = match get_token_float(&x) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };
            let y = match get_opt_token_float_opt(&y) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    None
                },
            };
            let z = match get_opt_token_float_opt(&z) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    None
                },
            };
            OptionElement::Scale((x, y, z))
        },
    )(input)
}

fn parse_option_turbulance(input: &[Token]) -> IResult<&[Token], OptionElement> {
    map(
        preceded(
            token_match!(Token::OptionTurbulence),
            tuple((
                token_match!(Token::Float(_)),
                opt(token_match!(Token::Float(_))),
                opt(token_match!(Token::Float(_))),
            )),
        ),
        |(x, y, z)| {
            let x = match get_token_float(&x) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };
            let y = match get_opt_token_float_opt(&y) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    None
                },
            };
            let z = match get_opt_token_float_opt(&z) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    None
                },
            };
            OptionElement::Turbulance((x, y, z))
        },
    )(input)
}

fn parse_option_texture_resolution(input: &[Token]) -> IResult<&[Token], OptionElement> {
    map(
        preceded(
            token_match!(Token::OptionTextureResolution),
            token_match!(Token::Int(_)),
        ),
        |s| {
            let val = match get_token_int(&s) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };
            OptionElement::TextureRes(val)
        },
    )(input)
}

fn parse_option_imf_channel(input: &[Token]) -> IResult<&[Token], OptionElement> {
    map(
        preceded(
            token_match!(Token::OptionIMFChan),
            token_match!(Token::String(_)),
        ),
        |s| {
            let val = match get_token_string(&s) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };
            OptionElement::ImfChan(val)
        },
    )(input)
}

fn parse_option_reflection_type(input: &[Token]) -> IResult<&[Token], OptionElement> {
    map(
        preceded(
            token_match!(Token::ReflectionType),
            token_match!(Token::String(_)),
        ),
        |s| {
            let val = match get_token_string(&s) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };
            OptionElement::ReflectionType(val)
        },
    )(input)
}
