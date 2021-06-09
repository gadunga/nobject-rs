use std::{
    collections::HashMap,
    result::Result,
};

use derive_more::{
    Constructor,
    From,
    Into,
};

use crate::tokenizer::Token;
use nom::{
    branch::alt,
    combinator::{
        map,
        opt,
    },
    multi::{
        fold_many0,
        fold_many1,
        many1,
    },
    sequence::{
        preceded,
        tuple,
    },
    IResult,
};
use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("Parse Error: `{0}`")]
    Parse(String),
}

#[derive(Clone, Constructor, Debug, Default, From, Into)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: Option<f32>,
}

#[derive(Clone, Constructor, Debug, Default, From, Into)]
pub struct Normal {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Constructor, Debug, Default, From, Into)]
pub struct Texture {
    pub u: f32,
    pub v: Option<f32>,
    pub w: Option<f32>,
}

#[derive(Clone, Constructor, Debug, Default, From, Into)]
pub struct Group {
    pub material_name: String,
    pub bevel:         bool,
    pub c_interp:      bool,
    pub d_interp:      bool,
    pub lod:           u8,
    pub texture_map:   Option<String>,
}

#[derive(Clone, Constructor, Debug, Default, From, Into)]
pub struct FaceElement {
    pub vertex_index:  i32,
    pub texture_index: Option<i32>,
    pub normal_index:  Option<i32>,
}

#[derive(Clone, Constructor, Debug, Default, From, Into)]
pub struct Face {
    pub elements:        Vec<FaceElement>,
    pub smoothing_group: i32,
}

#[derive(Clone, Constructor, Debug, Default, From, Into)]
pub struct LineElement {
    pub vertex_index:  i32,
    pub texture_index: Option<i32>,
}

#[derive(Clone, Constructor, Debug, Default, From, Into)]
pub struct Line {
    pub elements: Vec<LineElement>,
}

#[derive(Clone, Constructor, Debug, Default, From, Into)]
pub struct Point {
    pub elements: Vec<i32>,
}

#[derive(Clone, Constructor, Debug, From, Into)]
pub struct Model {
    pub vertices:            Vec<Vertex>,
    pub normals:             Vec<Normal>,
    pub textures:            Vec<Texture>,
    pub faces:               HashMap<String, Vec<Face>>,
    pub lines:               HashMap<String, Vec<Line>>,
    pub points:              HashMap<String, Vec<Point>>,
    pub groups:              HashMap<String, Group>,
    pub material_libs:       Vec<String>,
    pub texture_libs:        Vec<String>,
    pub shadow_obj:          Option<String>,
    pub trace_obj:           Option<String>,
    current_group:           Vec<String>,
    current_smoothing_group: i32,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            vertices:                Default::default(),
            normals:                 Default::default(),
            textures:                Default::default(),
            faces:                   Default::default(),
            lines:                   Default::default(),
            points:                  Default::default(),
            groups:                  {
                let mut res = HashMap::new();
                res.insert("default".into(), Default::default());
                res
            },
            material_libs:           Default::default(),
            texture_libs:            Default::default(),
            shadow_obj:              Default::default(),
            trace_obj:               Default::default(),
            current_group:           vec!["default".into()],
            current_smoothing_group: 0,
        }
    }
}

#[derive(Clone, Debug)]
enum ModelElement {
    Vertex(Vertex),
    Normal(Normal),
    Texture(Texture),
    Face(Face),
    Line(Line),
    Point(Point),
    Group(Vec<String>),
    MaterialLib(Vec<String>),
    Material(String),
    ObjName(String),
    Smoothing(i32),
    Bevel(bool),
    CInterp(bool),
    DInterp(bool),
    Lod(i32),
    ShadowObj(String),
    TraceObj(String),
    TextureLib(Vec<String>),
    TextureMap(String),
}

pub(crate) fn parse(input: &[Token]) -> Result<Model, ModelError> {
    match fold_many0(
        alt((
            map(parse_vertex, |v| ModelElement::Vertex(v)),
            map(parse_vertex_normal, |n| ModelElement::Normal(n)),
            map(parse_vertex_texture, |t| ModelElement::Texture(t)),
            map(parse_face, |f| ModelElement::Face(f)),
            map(parse_line, |l| ModelElement::Line(l)),
            map(parse_point, |p| ModelElement::Point(p)),
            parse_mat_lib,
            parse_material,
            parse_obj_name,
            parse_smoothing,
            parse_bevel,
            parse_c_interp,
            parse_d_interp,
            parse_lod,
            parse_shadow_obj,
            parse_trace_obj,
            parse_texture_lib,
            parse_texture_map,
            parse_group,
        )),
        Model::default(),
        |mut model: Model, item: ModelElement| {
            match item {
                ModelElement::Vertex(x) => model.vertices.push(x),
                ModelElement::Normal(n) => model.normals.push(n),
                ModelElement::Texture(t) => model.textures.push(t),
                ModelElement::Face(mut f) => {
                    f.smoothing_group = model.current_smoothing_group;
                    for g in &model.current_group {
                        let set = model.faces.entry(g.clone()).or_insert(Vec::new());
                        set.push(f.clone());
                    }
                },
                ModelElement::Line(l) => {
                    for g in &model.current_group {
                        let set = model.lines.entry(g.clone()).or_insert(Vec::new());
                        set.push(l.clone());
                    }
                },
                ModelElement::Point(p) => {
                    for g in &model.current_group {
                        let set = model.points.entry(g.clone()).or_insert(Vec::new());
                        set.push(p.clone());
                    }
                },
                ModelElement::Group(groups) => {
                    model.current_group.clear();
                    for g in groups {
                        model.groups.insert(g.clone(), Default::default());
                        model.current_group.push(g);
                    }
                },
                ModelElement::MaterialLib(libs) => model.material_libs.extend(libs),
                ModelElement::Material(name) => {
                    for g in &model.current_group {
                        let group = model.groups.entry(g.clone()).or_default();
                        group.material_name = name.clone();
                    }
                },
                ModelElement::ObjName(_name) => {},
                ModelElement::Smoothing(group_id) => {
                    model.current_smoothing_group = group_id;
                },
                ModelElement::Bevel(_flag) => {},
                ModelElement::CInterp(_flag) => {},
                ModelElement::DInterp(_flag) => {},
                ModelElement::Lod(_level) => {},
                ModelElement::ShadowObj(_name) => {},
                ModelElement::TraceObj(_name) => {},
                ModelElement::TextureLib(libs) => {
                    model.texture_libs.extend(libs);
                },
                ModelElement::TextureMap(name) => {
                    for g in &model.current_group {
                        let group = model.groups.entry(g.clone()).or_default();
                        group.texture_map = Some(name.clone());
                    }
                },
            }
            model
        },
    )(input)
    {
        Ok((_, acc)) => Ok(acc),
        Err(e) => Err(ModelError::Parse(e.to_string())),
    }
}

fn parse_vertex(input: &[Token]) -> IResult<&[Token], Vertex> {
    map(
        preceded(
            token_match!(Token::Vertex),
            tuple((
                token_match!(Token::Float(_)),
                token_match!(Token::Float(_)),
                token_match!(Token::Float(_)),
                opt(token_match!(Token::Float(_))),
            )),
        ),
        |(x, y, z, w)| {
            let (x, y, z) = (
                get_token_float(&x),
                get_token_float(&y),
                get_token_float(&z),
            );
            let w = w.map(|val| get_token_float(&val));
            (x, y, z, w).into()
        },
    )(input)
}

fn parse_vertex_normal(input: &[Token]) -> IResult<&[Token], Normal> {
    map(
        preceded(
            token_match!(Token::VertexNormal),
            tuple((
                token_match!(Token::Float(_)),
                token_match!(Token::Float(_)),
                token_match!(Token::Float(_)),
            )),
        ),
        |(x, y, z)| {
            (
                get_token_float(&x),
                get_token_float(&y),
                get_token_float(&z),
            )
                .into()
        },
    )(input)
}

fn parse_vertex_texture(input: &[Token]) -> IResult<&[Token], Texture> {
    map(
        preceded(
            token_match!(Token::VertexTexture),
            tuple((
                token_match!(Token::Float(_)),
                opt(token_match!(Token::Float(_))),
                opt(token_match!(Token::Float(_))),
            )),
        ),
        |(u, v, w)| {
            let u = get_token_float(&u);
            let v = v.map(|val| get_token_float(&val));
            let w = w.map(|val| get_token_float(&val));
            (u, v, w).into()
        },
    )(input)
}

fn parse_face(input: &[Token]) -> IResult<&[Token], Face> {
    preceded(
        token_match!(Token::Face),
        fold_many1(
            map(
                tuple((
                    token_match!(Token::Int(_)),
                    opt(token_match!(Token::Slash)),
                    opt(token_match!(Token::Int(_))),
                    opt(token_match!(Token::Slash)),
                    opt(token_match!(Token::Int(_))),
                )),
                |(v, _s1, t, _s2, n)| {
                    let v = get_token_int(&v);
                    let t = t.map(|tex| get_token_int(&tex));
                    let n = n.map(|norm| get_token_int(&norm));
                    (v, t, n).into()
                },
            ),
            Face::default(),
            |mut f: Face, item: FaceElement| {
                f.elements.push(item);
                f
            },
        ),
    )(input)
}

fn parse_line(input: &[Token]) -> IResult<&[Token], Line> {
    preceded(
        token_match!(Token::Line),
        fold_many1(
            map(
                tuple((
                    token_match!(Token::Int(_)),
                    opt(token_match!(Token::Slash)),
                    opt(token_match!(Token::Int(_))),
                )),
                |(v, _s1, t)| {
                    let v = get_token_int(&v);
                    let t = t.map(|tex| get_token_int(&tex));
                    (v, t).into()
                },
            ),
            Line::default(),
            |mut f: Line, item: LineElement| {
                f.elements.push(item);
                f
            },
        ),
    )(input)
}

fn parse_point(input: &[Token]) -> IResult<&[Token], Point> {
    preceded(
        token_match!(Token::Point),
        fold_many1(
            map(token_match!(Token::Int(_)), |v| get_token_int(&v)),
            Point::default(),
            |mut f: Point, item: i32| {
                f.elements.push(item);
                f
            },
        ),
    )(input)
}

fn parse_group(input: &[Token]) -> IResult<&[Token], ModelElement> {
    map(
        preceded(
            token_match!(Token::Group),
            many1(map(token_match!(Token::String(_)), |s| {
                if let Token::String(s) = s {
                    s
                } else {
                    panic!();
                }
            })),
        ),
        |v| ModelElement::Group(v),
    )(input)
}

fn parse_mat_lib(input: &[Token]) -> IResult<&[Token], ModelElement> {
    map(
        preceded(
            token_match!(Token::MaterialLib),
            many1(map(token_match!(Token::String(_)), |s| {
                if let Token::String(s) = s {
                    s
                } else {
                    panic!();
                }
            })),
        ),
        |v| ModelElement::MaterialLib(v),
    )(input)
}

fn parse_material(input: &[Token]) -> IResult<&[Token], ModelElement> {
    map(
        preceded(
            token_match!(Token::UseMaterial),
            token_match!(Token::String(_)),
        ),
        |s| {
            if let Token::String(s) = s {
                ModelElement::Material(s)
            } else {
                panic!();
            }
        },
    )(input)
}

fn parse_obj_name(input: &[Token]) -> IResult<&[Token], ModelElement> {
    map(
        preceded(token_match!(Token::Object), token_match!(Token::String(_))),
        |s| {
            if let Token::String(s) = s {
                ModelElement::ObjName(s)
            } else {
                panic!();
            }
        },
    )(input)
}

fn parse_smoothing(input: &[Token]) -> IResult<&[Token], ModelElement> {
    map(
        preceded(
            token_match!(Token::Smoothing),
            alt((
                token_match!(Token::Int(_)),
                map(token_match!(Token::String(_)), |s| {
                    if let Token::String(s) = s {
                        if s == "off" {
                            Token::Int(0)
                        } else {
                            panic!();
                        }
                    } else {
                        panic!();
                    }
                }),
            )),
        ),
        |s| {
            if let Token::Int(s) = s {
                ModelElement::Smoothing(s)
            } else {
                panic!();
            }
        },
    )(input)
}

fn parse_bevel(input: &[Token]) -> IResult<&[Token], ModelElement> {
    map(
        preceded(token_match!(Token::Bevel), token_match!(Token::String(_))),
        |s| {
            if let Token::String(s) = s {
                if let Ok(flag) = s.parse::<bool>() {
                    ModelElement::Bevel(flag)
                } else {
                    panic!();
                }
            } else {
                panic!();
            }
        },
    )(input)
}

fn parse_c_interp(input: &[Token]) -> IResult<&[Token], ModelElement> {
    map(
        preceded(token_match!(Token::CInterp), token_match!(Token::String(_))),
        |s| {
            if let Token::String(s) = s {
                if let Ok(flag) = s.parse::<bool>() {
                    ModelElement::CInterp(flag)
                } else {
                    panic!();
                }
            } else {
                panic!();
            }
        },
    )(input)
}

fn parse_d_interp(input: &[Token]) -> IResult<&[Token], ModelElement> {
    map(
        preceded(token_match!(Token::DInterp), token_match!(Token::String(_))),
        |s| {
            if let Token::String(s) = s {
                if let Ok(flag) = s.parse::<bool>() {
                    ModelElement::DInterp(flag)
                } else {
                    panic!();
                }
            } else {
                panic!();
            }
        },
    )(input)
}

fn parse_lod(input: &[Token]) -> IResult<&[Token], ModelElement> {
    map(
        preceded(token_match!(Token::Lod), token_match!(Token::Int(_))),
        |s| {
            if let Token::Int(s) = s {
                ModelElement::Lod(s)
            } else {
                panic!();
            }
        },
    )(input)
}

fn parse_shadow_obj(input: &[Token]) -> IResult<&[Token], ModelElement> {
    map(
        preceded(
            token_match!(Token::ShadowObj),
            token_match!(Token::String(_)),
        ),
        |s| {
            if let Token::String(s) = s {
                ModelElement::ShadowObj(s)
            } else {
                panic!();
            }
        },
    )(input)
}

fn parse_trace_obj(input: &[Token]) -> IResult<&[Token], ModelElement> {
    map(
        preceded(
            token_match!(Token::TraceObj),
            token_match!(Token::String(_)),
        ),
        |s| {
            if let Token::String(s) = s {
                ModelElement::TraceObj(s)
            } else {
                panic!();
            }
        },
    )(input)
}

fn parse_texture_lib(input: &[Token]) -> IResult<&[Token], ModelElement> {
    map(
        preceded(
            token_match!(Token::TextureMapLib),
            many1(map(token_match!(Token::String(_)), |s| {
                if let Token::String(s) = s {
                    s
                } else {
                    panic!();
                }
            })),
        ),
        |v| ModelElement::TextureLib(v),
    )(input)
}

fn parse_texture_map(input: &[Token]) -> IResult<&[Token], ModelElement> {
    map(
        preceded(
            token_match!(Token::UseTextureMap),
            token_match!(Token::String(_)),
        ),
        |s| {
            if let Token::String(s) = s {
                ModelElement::TextureMap(s)
            } else {
                panic!();
            }
        },
    )(input)
}

fn get_token_float(token: &Token) -> f32 {
    if let Token::Float(f) = token {
        *f
    } else {
        panic!()
    }
}

fn get_token_int(token: &Token) -> i32 {
    if let Token::Int(i) = token {
        *i
    } else {
        panic!()
    }
}