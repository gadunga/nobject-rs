#![allow(clippy::needless_doctest_main)]
//! # Overview
//!
//! `nobject-rs` is a library for parsing wavefront .obj and .mtl content.
//! To this end, the crate exposes two methos:  
//! * `load_obj`
//! * `load_mtl`
//!
//! Both methods take the content of the respective files (.obj and .mtl),
//! parse and then return a result with either some kind of parse error, or
//! a struct containing the data.  
//!
//! Note that this crate leaves the responsibility of file I/O to the consuming
//! application. For example, it's possible to specify file names as attributes
//! in the material, or file names as material libraries in the obj file. This
//! library will NOT attempt to open and parse those files. It is left to the
//! consuming application/library to take the file information from the results
//! of the parse methods, find and open the appropriate files, and then pass on
//! the contents to be parsed.
//!
//! # Reference
//!
//! Parsing is done based on the specification for Obj's and Mtl's found at:
//! * [Obj]( http://paulbourke.net/dataformats/obj/)
//! * [Mtl](http://paulbourke.net/dataformats/mtl/)
//!
//! # Examples
//!
//! ## Obj parsing
//! ```rust
//! fn main() {
//!     let input =
//!     "
//!     o 1
//!     v -0.5 -0.5 0.5
//!     v -0.5 -0.5 -0.5
//!     v -0.5 0.5 -0.5
//!     v -0.5 0.5 0.5
//!     v 0.5 -0.5 0.5
//!     v 0.5 -0.5 -0.5
//!     v 0.5 0.5 -0.5
//!     v 0.5 0.5 0.5
//!     
//!     usemtl Default
//!     f 4 3 2 1
//!     f 2 6 5 1
//!     f 3 7 6 2
//!     f 8 7 3 4
//!     f 5 8 4 1
//!     f 6 7 8 5
//!     ";
//!
//!     let res = nobject_rs::load_obj(&input).unwrap();
//!     let group = &res.groups["default"];
//!     let face_group = &res.faces["default"];
//!     assert_eq!(res.vertices.len(), 8);
//!     assert_eq!(group.material_name, "Default".to_string());
//!     assert_eq!(res.normals.len(), 0);
//!     assert_eq!(res.faces.len(), 1);
//!     assert_eq!(face_group.len(), 6);;
//! }
//! ```
//!
//! ## Mtl parsing
//! ```rust
//! fn main() {
//!     let input =
//!     "newmtl frost_wind
//!     Ka 0.2 0.2 0.2
//!     Kd 0.6 0.6 0.6
//!     Ks 0.1 0.1 0.1
//!     d 1
//!     Ns 200
//!     illum 2
//!     map_d -mm 0.200 0.800 window.mps";
//!
//!     let res = nobject_rs::load_mtl(&input).unwrap();
//!     assert_eq!(res.len(), 1);
//! }
//! ```
#[macro_use]
mod macros;

#[cfg(test)]
mod test;
mod tokenizer;

mod material;
mod model;

use std::result::Result;

pub use model::{
    Face, FaceElement, Group, Line, LineElement, Model, ModelError, Normal, Point, Texture, Vertex,
};

pub use material::{
    BumpMap, ColorCorrectedMap, ColorType, DisolveType, Material, MaterialError,
    NonColorCorrectedMap, ReflectionMap,
};

use thiserror::Error;
use tokenizer::{Token, TokenizeError};

/// The set of errors which might be generated.
#[derive(Error, Debug)]
pub enum ObjError {
    /// A tokenization error, typically something
    /// in the file is not as the parser expects it.
    #[error("Tokenize Error: `{0}`")]
    Tokenize(#[from] TokenizeError),

    /// The result of an error constructing a `Model`
    /// from the token stream.
    #[error("Model Error: `{0}`")]
    ModelParse(#[from] ModelError),

    /// The result of an error constructing a `Material`
    /// collection from the token stream.
    #[error("Material Error: `{0}`")]
    MaterialParse(#[from] MaterialError),

    /// An unexpected token was encountered in the token stream.
    #[error("Unexpected token encountered: `{0:#?}`")]
    UnexpectedToken(Token),

    /// The specification for obj/mtl files has some settings
    /// either being "on" or "off". If there is an issue
    /// parsing those values, this error will occur.
    #[error("Unexpected on/off value encountered: `{0}`")]
    InvalidOnOffValue(String),
}

/// Takes the content of an obj file and parses it.
///
/// # Arguments  
/// * input - The content of the obj file as a string
///
/// # Returns  
/// Returns a `Result` of either ObjError on parse errors
/// or a constructed `Model`.
pub fn load_obj(input: &str) -> Result<Model, ObjError> {
    match tokenizer::parse_obj(input) {
        Ok(tokens) => Ok(model::parse(&tokens)?),
        Err(e) => Err(e.into()),
    }
}

/// Takes the content of an mtl file and parses it.
///
/// # Arguments  
/// * input - The content of the mtl file as a string
///
/// # Returns  
/// Returns a `Result` of either ObjError on parse errors
/// or a collection of `Material`.
pub fn load_mtl(input: &str) -> Result<Vec<Material>, ObjError> {
    match tokenizer::parse_mtl(input) {
        Ok(tokens) => Ok(material::parse(&tokens)?),
        Err(e) => Err(e.into()),
    }
}

fn get_token_float(token: &Token) -> Result<f32, ObjError> {
    if let Token::Float(f) = token {
        Ok(*f)
    } else if let Token::Int(i) = token {
        Ok(*i as f32)
    } else {
        Err(ObjError::UnexpectedToken(token.clone()))
    }
}

fn get_opt_token_float_opt(token: &Option<Token>) -> Result<Option<f32>, ObjError> {
    if let Some(t) = token {
        if let Token::Float(f) = t {
            Ok(Some(*f))
        } else if let Token::Int(i) = t {
            Ok(Some(*i as f32))
        } else {
            Err(ObjError::UnexpectedToken(t.clone()))
        }
    } else {
        Ok(None)
    }
}

fn get_token_int(token: &Token) -> Result<i32, ObjError> {
    if let Token::Int(i) = token {
        Ok(*i)
    } else {
        Err(ObjError::UnexpectedToken(token.clone()))
    }
}

fn get_token_string(token: &Token) -> Result<String, ObjError> {
    if let Token::String(s) = token {
        Ok(s.clone())
    } else if let Token::Int(i) = token {
        Ok(i.to_string())
    } else if let Token::Float(f) = token {
        Ok(f.to_string())
    } else {
        Err(ObjError::UnexpectedToken(token.clone()))
    }
}

fn get_on_off_from_str(token: &Token) -> Result<bool, ObjError> {
    let s = get_token_string(&token)?;
    match s.as_str() {
        "on" => Ok(true),
        "off" => Ok(false),
        _ => Err(ObjError::InvalidOnOffValue(s.clone())),
    }
}
