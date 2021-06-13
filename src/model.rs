use std::{
    collections::HashMap,
    result::Result,
};

use derive_more::{
    Constructor,
    From,
    Into,
};

use crate::{
    get_on_off_from_str,
    get_token_float,
    get_token_int,
    get_token_string,
    token_match,
    tokenizer::Token,
};

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

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("Parse Error: `{0}`")]
    Parse(String),
}

#[derive(Clone, Constructor, Debug, Default, From, Into, PartialEq)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: Option<f32>,
}

#[derive(Clone, Constructor, Debug, Default, From, Into, PartialEq)]
pub struct Normal {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Constructor, Debug, Default, From, Into, PartialEq)]
pub struct Texture {
    pub u: f32,
    pub v: Option<f32>,
    pub w: Option<f32>,
}

#[derive(Clone, Constructor, Debug, Default, From, Into, PartialEq)]
pub struct Group {
    pub material_name: String,
    pub bevel:         bool,
    pub c_interp:      bool,
    pub d_interp:      bool,
    pub lod:           u8,
    pub texture_map:   Option<String>,
}

#[derive(Clone, Constructor, Debug, Default, From, Into, PartialEq)]
pub struct FaceElement {
    pub vertex_index:  i32,
    pub texture_index: Option<i32>,
    pub normal_index:  Option<i32>,
}

#[derive(Clone, Constructor, Debug, Default, From, Into, PartialEq)]
pub struct Face {
    pub elements:        Vec<FaceElement>,
    pub smoothing_group: i32,
}

#[derive(Clone, Constructor, Debug, Default, From, Into, PartialEq)]
pub struct LineElement {
    pub vertex_index:  i32,
    pub texture_index: Option<i32>,
}

#[derive(Clone, Constructor, Debug, Default, From, Into, PartialEq)]
pub struct Line {
    pub elements: Vec<LineElement>,
}

#[derive(Clone, Constructor, Debug, Default, From, Into, PartialEq)]
pub struct Point {
    pub elements: Vec<i32>,
}

#[derive(Clone, Debug, From, Into)]
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
            map(parse_vertex, ModelElement::Vertex),
            map(parse_vertex_normal, ModelElement::Normal),
            map(parse_vertex_texture, ModelElement::Texture),
            map(parse_face, ModelElement::Face),
            map(parse_line, ModelElement::Line),
            map(parse_point, ModelElement::Point),
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
                        let set = model.faces.entry(g.clone()).or_insert_with(Vec::new);
                        set.push(f.clone());
                    }
                },
                ModelElement::Line(l) => {
                    for g in &model.current_group {
                        let set = model.lines.entry(g.clone()).or_insert_with(Vec::new);
                        set.push(l.clone());
                    }
                },
                ModelElement::Point(p) => {
                    for g in &model.current_group {
                        let set = model.points.entry(g.clone()).or_insert_with(Vec::new);
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
                match get_token_float(&x) {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!("{}", e);
                        Default::default()
                    },
                },
                match get_token_float(&y) {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!("{}", e);
                        Default::default()
                    },
                },
                match get_token_float(&z) {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!("{}", e);
                        Default::default()
                    },
                },
            );
            let w = w.map(|val| match get_token_float(&val) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            });
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
            let (x, y, z) = (
                match get_token_float(&x) {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!("{}", e);
                        Default::default()
                    },
                },
                match get_token_float(&y) {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!("{}", e);
                        Default::default()
                    },
                },
                match get_token_float(&z) {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!("{}", e);
                        Default::default()
                    },
                },
            );
            (x, y, z).into()
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
            let u = match get_token_float(&u) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };
            let v = v.map(|val| match get_token_float(&val) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            });
            let w = w.map(|val| match get_token_float(&val) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            });
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
                    opt(preceded(
                        token_match!(Token::Slash),
                        opt(token_match!(Token::Int(_))),
                    )),
                    opt(preceded(
                        token_match!(Token::Slash),
                        opt(token_match!(Token::Int(_))),
                    )),
                )),
                |(v, t, n)| {
                    let v = match get_token_int(&v) {
                        Ok(s) => s,
                        Err(e) => {
                            log::error!("{}", e);
                            Default::default()
                        },
                    };
                    let t = match t {
                        Some(t) => t.map(|tex| match get_token_int(&tex) {
                            Ok(s) => s,
                            Err(e) => {
                                log::error!("{}", e);
                                Default::default()
                            },
                        }),
                        None => None,
                    };

                    let n = match n {
                        Some(n) => n.map(|norm| match get_token_int(&norm) {
                            Ok(s) => s,
                            Err(e) => {
                                log::error!("{}", e);
                                Default::default()
                            },
                        }),
                        None => None,
                    };
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
                    let v = match get_token_int(&v) {
                        Ok(s) => s,
                        Err(e) => {
                            log::error!("{}", e);
                            Default::default()
                        },
                    };
                    let t = t.map(|tex| match get_token_int(&tex) {
                        Ok(s) => s,
                        Err(e) => {
                            log::error!("{}", e);
                            Default::default()
                        },
                    });
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
            map(token_match!(Token::Int(_)), |v| match get_token_int(&v) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            }),
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
            many1(map(
                token_match!(Token::String(_)),
                |s| match get_token_string(&s) {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!("{}", e);
                        Default::default()
                    },
                },
            )),
        ),
        ModelElement::Group,
    )(input)
}

fn parse_mat_lib(input: &[Token]) -> IResult<&[Token], ModelElement> {
    map(
        preceded(
            token_match!(Token::MaterialLib),
            many1(map(
                token_match!(Token::String(_)),
                |s| match get_token_string(&s) {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!("{}", e);
                        Default::default()
                    },
                },
            )),
        ),
        ModelElement::MaterialLib,
    )(input)
}

fn parse_material(input: &[Token]) -> IResult<&[Token], ModelElement> {
    map(
        preceded(
            token_match!(Token::UseMaterial),
            token_match!(Token::String(_)),
        ),
        |s| {
            let res = match get_token_string(&s) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };

            ModelElement::Material(res)
        },
    )(input)
}

fn parse_obj_name(input: &[Token]) -> IResult<&[Token], ModelElement> {
    map(
        preceded(
            token_match!(Token::Object),
            token_match!(Token::String(_) | Token::Int(_)),
        ),
        |s| {
            let res = match get_token_string(&s) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };
            ModelElement::ObjName(res)
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
                    let val = match get_on_off_from_str(&s) {
                        Ok(v) => v,
                        Err(e) => {
                            log::error!("{}", e);
                            Default::default()
                        },
                    };
                    if !val {
                        Token::Int(0)
                    } else {
                        panic!();
                    }
                }),
            )),
        ),
        |s| {
            let res = match get_token_int(&s) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };
            ModelElement::Smoothing(res)
        },
    )(input)
}

fn parse_bevel(input: &[Token]) -> IResult<&[Token], ModelElement> {
    map(
        preceded(token_match!(Token::Bevel), token_match!(Token::String(_))),
        |s| {
            let res = match get_token_string(&s) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };

            if let Ok(flag) = res.parse::<bool>() {
                ModelElement::Bevel(flag)
            } else {
                ModelElement::Bevel(false)
            }
        },
    )(input)
}

fn parse_c_interp(input: &[Token]) -> IResult<&[Token], ModelElement> {
    map(
        preceded(token_match!(Token::CInterp), token_match!(Token::String(_))),
        |s| {
            let res = match get_token_string(&s) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };

            if let Ok(flag) = res.parse::<bool>() {
                ModelElement::CInterp(flag)
            } else {
                ModelElement::CInterp(false)
            }
        },
    )(input)
}

fn parse_d_interp(input: &[Token]) -> IResult<&[Token], ModelElement> {
    map(
        preceded(token_match!(Token::DInterp), token_match!(Token::String(_))),
        |s| {
            let res = match get_token_string(&s) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };

            if let Ok(flag) = res.parse::<bool>() {
                ModelElement::DInterp(flag)
            } else {
                ModelElement::DInterp(false)
            }
        },
    )(input)
}

fn parse_lod(input: &[Token]) -> IResult<&[Token], ModelElement> {
    map(
        preceded(token_match!(Token::Lod), token_match!(Token::Int(_))),
        |s| {
            let res = match get_token_int(&s) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };
            ModelElement::Lod(res)
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
            let res = match get_token_string(&s) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };

            ModelElement::ShadowObj(res)
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
            let res = match get_token_string(&s) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };

            ModelElement::TraceObj(res)
        },
    )(input)
}

fn parse_texture_lib(input: &[Token]) -> IResult<&[Token], ModelElement> {
    map(
        preceded(
            token_match!(Token::TextureMapLib),
            many1(map(token_match!(Token::String(_)), |s| {
                let res = match get_token_string(&s) {
                    Ok(s) => s,
                    Err(e) => {
                        log::error!("{}", e);
                        Default::default()
                    },
                };

                res
            })),
        ),
        ModelElement::TextureLib,
    )(input)
}

fn parse_texture_map(input: &[Token]) -> IResult<&[Token], ModelElement> {
    map(
        preceded(
            token_match!(Token::UseTextureMap),
            token_match!(Token::String(_)),
        ),
        |s| {
            let res = match get_token_string(&s) {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{}", e);
                    Default::default()
                },
            };

            ModelElement::TextureMap(res)
        },
    )(input)
}
