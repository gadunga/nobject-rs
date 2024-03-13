mod mtl;
mod obj;

#[cfg(test)]
mod test;

pub use mtl::parse_mtl;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, opt},
    multi::{fold_many0, fold_many1},
    sequence::tuple,
    IResult,
};
pub use obj::parse_obj;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TokenizeError {
    #[error("Parse Error: `{0}`")]
    Parse(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Ignore,
    String(String),
    Float(f32),
    Int(i32),
    Slash,

    // Obj
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

    /// Point element
    /// p  v1 v2 v3 ...
    Point,

    /// L:ine element
    /// l  v1/vt1   v2/vt2   v3/vt3 ...
    /// textures are optional
    Line,

    /// mtllib filename1 filename2 ...
    MaterialLib,

    /// usemtl material_name
    UseMaterial,

    /// o object_name
    Object,

    /// g group_name1 group_name2 ...
    /// Currently ignores all names after the first
    Group,

    /// s group_number
    Smoothing,

    /// bevel on/off
    Bevel,

    /// c_interp on/off
    CInterp,

    /// d_interp on/off
    DInterp,

    /// lod level
    /// Level is a value from 0 to 100
    Lod,

    /// shadow_obj filename
    ShadowObj,

    /// trace_obj filename
    TraceObj,

    /// maplib filename1 filename2 ...
    TextureMapLib,

    /// usemap map_name/off
    UseTextureMap,

    /// Used in Ka/Kd/Ks
    Spectral,

    /// Used in Ka/Kd/Ks
    Xyz,

    // MTL
    /// newmtl my_mtl
    NewMaterial,

    /// # Variants
    /// Ka r g b
    /// Ka spectral filename factor
    /// Ka xyz x y z
    ///
    /// Variant Notes:
    /// The "Ka spectral" statement specifies the ambient reflectivity using a
    /// spectral curve. "factor" is an optional and defaults to 1.0 if not
    /// specified.
    ///
    /// "x y z" are the values of the CIEXYZ color space.  The y and z
    /// arguments are optional.  If only x is specified, then y and z are
    /// assumed to be equal to x.
    AmbientColor,

    /// # Variants
    /// Kd r g b
    /// Kd spectral filename factor
    /// Kd xyz x y z
    ///
    /// Variant Notes:
    /// The "Kd spectral" statement specifies the diffuse reflectivity using a
    /// spectral curve. "factor" is an optional and defaults to 1.0 if not
    /// specified.
    ///
    /// "x y z" are the values of the CIEXYZ color space.  The y and z
    /// arguments are optional.  If only x is specified, then y and z are
    /// assumed to be equal to x.
    DiffuseColor,

    /// # Variants
    /// Ks r g b
    /// Ks spectral filename factor
    /// Ks xyz x y z
    ///
    /// Variant Notes:
    /// The "Ks spectral" statement specifies the specular reflectivity using a
    /// spectral curve. "factor" is an optional and defaults to 1.0 if not
    /// specified.
    ///
    /// "x y z" are the values of the CIEXYZ color space.  The y and z
    /// arguments are optional.  If only x is specified, then y and z are
    /// assumed to be equal to x.
    SpecularColor,

    /// # Variants
    /// Ke r g b
    /// Ke spectral filename factor
    /// Ke xyz x y z
    ///
    /// Variant Notes:
    /// The "Ke spectral" statement specifies the emissive coefficient using a
    /// spectral curve. "factor" is an optional and defaults to 1.0 if not
    /// specified.
    ///
    /// "x y z" are the values of the CIEXYZ color space.  The y and z
    /// arguments are optional.  If only x is specified, then y and z are
    /// assumed to be equal to x.
    EmissiveCoefficient,

    /// Ns s
    /// Shininess of the material. Default is 0.0
    SpecularExponent,

    /// # Variants
    /// d alpha
    /// d -halo factor
    ///
    /// Specifies the dissolve for the current material.
    /// The second variant specifies that a dissolve is dependent on the surface
    /// orientation relative to the viewer
    Disolved,

    /// Used in dissolve
    Halo,

    /// Tr alpha
    /// Transparency
    Transparancy,

    /// # Variants
    /// Tf r g b
    /// Tf spectral file.rfl factor
    /// Tf xyz x y z
    ///
    /// Transmission factor
    TransmissionFactor,

    /// sharpness n
    /// Defaults to 60
    Sharpness,

    /// Ni optical_density
    /// Index of refraction
    IndexOfRefraction,

    /// illum n
    /// Illumination Model
    IlluminationModel,

    /// map_Ka -options args filename
    /// Ambient Texture Map
    ///
    /// # Example
    ///
    /// map_Ka -s 1 1 1 -o 0 0 0 -mm 0 1 chrome.mpc
    ///
    /// # Options
    /// - -blendu on | off
    /// - -blendv on | off
    /// - -cc on | off
    /// - -clamp on | off
    /// - -mm base gain
    /// - -o u v w
    /// - -s u v w
    /// - -t u v w
    /// - -texres value
    TextureMapAmbient,

    /// map_Kd -options args filename
    /// Diffuse Texture Map
    ///
    /// # Example
    ///
    /// map_Kd -s 1 1 1 -o 0 0 0 -mm 0 1 chrome.mpc
    ///
    /// # Options
    /// - -blendu on | off
    /// - -blendv on | off
    /// - -cc on | off
    /// - -clamp on | off
    /// - -mm base gain
    /// - -o u v w
    /// - -s u v w
    /// - -t u v w
    /// - -texres value
    TextureMapDiffuse,

    /// map_Ks -options args filename
    /// Specular Texture Map
    ///
    /// # Example
    ///
    /// map_Ks -s 1 1 1 -o 0 0 0 -mm 0 1 chrome.mpc
    ///
    /// # Options
    /// - -blendu on | off
    /// - -blendv on | off
    /// - -cc on | off
    /// - -clamp on | off
    /// - -mm base gain
    /// - -o u v w
    /// - -s u v w
    /// - -t u v w
    /// - -texres value
    TextureMapSpecular,

    /// map_Ns -s 1 1 1 -o 0 0 0 -mm 0 1 wisp.mps
    /// Shininess map
    ///
    /// # Example
    ///
    /// map_Ns -s 1 1 1 -o 0 0 0 -mm 0 1 wisp.mps
    ///
    /// # Options
    /// - -blendu on | off
    /// - -blendv on | off
    /// - -clamp on | off
    /// - -imfchan r | g | b | m | l | z
    /// - -mm base gain
    /// - -o u v w
    /// - -s u v w
    /// - -t u v w
    /// - -texres value
    TextureMapShininess,

    /// map_d -s 1 1 1 -o 0 0 0 -mm 0 1 wisp.mps
    /// Disolve matp
    /// # Example
    ///
    /// map_d -s 1 1 1 -o 0 0 0 -mm 0 1 wisp.mps
    ///
    /// # Options
    /// - -blendu on | off
    /// - -blendv on | off
    /// - -clamp on | off
    /// - -imfchan r | g | b | m | l | z
    /// - -mm base gain
    /// - -o u v w
    /// - -s u v w
    /// - -t u v w
    /// - -texres value
    TextureMapDisolved,

    /// map_aat on
    /// Turns on anti-aliasing of textures in this material only.
    AntiAliasMap,

    /// disp -s 1 1 .5 wisp.mps
    /// Displacement map
    /// # Example
    ///
    /// disp -s 1 1 .5 wisp.mps
    ///
    /// # Options
    /// - -blendu on | off
    /// - -blendv on | off
    /// - -clamp on | off
    /// - -imfchan r | g | b | m | l | z
    /// - -mm base gain
    /// - -o u v w
    /// - -s u v w
    /// - -t u v w
    /// - -texres value
    DisplacementMap,

    /// decal -s 1 1 1 -o 0 0 0 -mm 0 1 sand.mps
    /// Displacement map
    /// # Example
    ///
    /// decal -s 1 1 1 -o 0 0 0 -mm 0 1 sand.mps
    ///
    /// # Options
    /// - -blendu on | off
    /// - -blendv on | off
    /// - -clamp on | off
    /// - -imfchan r | g | b | m | l | z
    /// - -mm base gain
    /// - -o u v w
    /// - -s u v w
    /// - -t u v w
    /// - -texres value
    Decal,

    /// bump -s 1 1 1 -o 0 0 0 -bm 1 sand.mpb
    /// Bump map
    /// # Example
    ///
    /// bump -s 1 1 1 -o 0 0 0 -bm 1 sand.mpb
    ///
    /// # Options
    /// - -bm mult
    /// - -clamp on | off
    /// - -blendu on | off
    /// - -blendv on | off
    /// - -imfchan r | g | b | m | l | z
    /// - -mm base gain
    /// - -o u v w
    /// - -s u v w
    /// - -t u v w
    /// - -texres value
    BumpMap,

    /// refl -type sphere -mm 0 1 clouds.mpc
    /// Reflection map
    /// # Example
    ///
    /// refl -type sphere -mm 0 1 clouds.mpc
    ///
    /// # Options
    /// - -blendu on | off
    /// - -blendv on | off
    /// - -cc on | off
    /// - -clamp on | off
    /// - -mm base gain
    /// - -o u v w
    /// - -s u v w
    /// - -t u v w
    /// - -texres value
    ReflectionMap,

    /// -type
    ReflectionType,

    /// texture blending in the horizontal direction
    /// -blendu on | off
    ///
    /// # Default
    /// On
    OptionBlendU,

    /// texture blending in the vertical direction
    /// -blendv on | off
    ///
    /// # Default
    /// On
    OptionBlendV,

    /// Bump multiplier
    /// -bm mult
    OptionBumpMultiplier,

    /// increases the sharpness, or clarity, of mip-mapped
    /// texture files. This does not appear to be used anywhere.
    /// -boost value
    OptionBoost,

    /// -cc on | off
    /// color correction for the texture
    OptionColorCorrect,

    /// -clamp on | off
    /// Texture clamp
    ///
    /// # Default
    /// Off
    OptionClamp,

    /// -imfchan r | g | b | m | l | z
    /// Color channel
    OptionIMFChan,

    /// -mm base gain
    /// Color/Scalar Texture range
    OptionRange,

    /// -o u v w
    /// Texture map offset on the surface.
    /// v/w are optional.
    ///
    /// # Default
    /// (0, 0, 0)
    OptionOffset,

    /// -s u v w
    /// Scale the size of the texture pattern
    /// v/w are optional.
    ///
    /// # Default
    /// (1, 1, 1)
    OptionScale,

    /// -t u v w
    /// Texture turbulence
    /// v/w are optional
    ///
    /// # Default
    /// (0, 0, 0)
    OptionTurbulence,

    /// -texres resolution
    /// Texture resolution to use
    OptionTextureResolution,
}

pub(self) fn parse_digit(input: &str) -> IResult<&str, Token> {
    map(
        tuple((
            opt(alt((tag("+"), tag("-")))),
            fold_many1(digit1, Vec::new, |mut acc: Vec<_>, item| {
                acc.push(item);
                acc
            }),
        )),
        |(sign, s): (Option<&str>, Vec<&str>)| {
            let mut val = s.join("").parse::<i32>().unwrap_or_default();
            if sign == Some("-") {
                val *= -1;
            }
            Token::Int(val)
        },
    )(input)
}

#[allow(clippy::type_complexity)]
pub(self) fn parse_float(input: &str) -> IResult<&str, Token> {
    map(
        tuple((
            opt(alt((tag("+"), tag("-")))),
            alt((
                map(
                    tuple((
                        fold_many0(digit1, Vec::new, |mut acc: Vec<_>, item| {
                            acc.push(item);
                            acc
                        }),
                        tag("."),
                        opt(fold_many1(digit1, Vec::new, |mut acc: Vec<_>, item| {
                            acc.push(item);
                            acc
                        })),
                        opt(map(
                            tuple((
                                alt((tag("e"), tag("E"))),
                                opt(alt((tag("+"), tag("-")))),
                                digit1,
                            )),
                            |(e, sign, digits)| {
                                let mut acc = String::new();
                                acc.push_str(e);
                                if let Some(sign) = sign {
                                    acc.push_str(sign);
                                }
                                acc.push_str(digits);
                                acc
                            },
                        )),
                    )),
                    |(f, _, s, e)| (f, s.unwrap_or_default(), e.unwrap_or_default()),
                ),
                map(
                    tuple((
                        opt(fold_many1(digit1, Vec::new, |mut acc: Vec<_>, item| {
                            acc.push(item);
                            acc
                        })),
                        tag("."),
                        fold_many1(digit1, Vec::new, |mut acc: Vec<_>, item| {
                            acc.push(item);
                            acc
                        }),
                        opt(map(
                            tuple((
                                alt((tag("e"), tag("E"))),
                                opt(alt((tag("+"), tag("-")))),
                                digit1,
                            )),
                            |(e, sign, digits)| {
                                let mut acc = String::new();
                                acc.push_str(e);
                                if let Some(sign) = sign {
                                    acc.push_str(sign);
                                }
                                acc.push_str(digits);
                                acc
                            },
                        )),
                    )),
                    |(f, _, s, e)| (f.unwrap_or_default(), s, e.unwrap_or_default()),
                ),
            )),
        )),
        |(sign, (f, s, e)): (Option<&str>, (Vec<&str>, Vec<&str>, String))| {
            let mut acc = Vec::new();
            if !f.is_empty() {
                acc.extend(f);
            }
            acc.push(".");
            if !s.is_empty() {
                acc.extend(s);
            }
            if !e.is_empty() {
                acc.push(e.as_str());
            }
            let mut val = acc.join("").parse::<f32>().unwrap_or_default();
            if sign == Some("-") {
                val *= -1.0;
            }
            Token::Float(val)
        },
    )(input)
}
