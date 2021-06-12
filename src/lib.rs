#[cfg(test)]
mod test;
mod tokenizer;

#[macro_use]
mod macros;

mod material;
mod model;

use std::result::Result;

pub use model::{
    Model,
    ModelError,
};

pub use material::{
    Material,
    MaterialError,
};

use thiserror::Error;
use tokenizer::{
    Token,
    TokenizeError,
};

#[derive(Error, Debug)]
pub enum ObjError {
    #[error("Tokenize Error: `{0}`")]
    Tokenize(#[from] TokenizeError),

    #[error("Model Error: `{0}`")]
    ModelParse(#[from] ModelError),

    #[error("Material Error: `{0}`")]
    MaterialParse(#[from] MaterialError),

    #[error("Unexpected token encountered: `{0:#?}`")]
    UnexpectedToken(Token),

    #[error("Unexpected on/off value encountered: `{0}`")]
    InvalidOnOffValue(String),
}

pub fn load_obj(input: &str) -> Result<Model, ObjError> {
    match tokenizer::parse_obj(input) {
        Ok(tokens) => Ok(model::parse(&tokens)?),
        Err(e) => Err(e.into()),
    }
}

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
    } else {
        Err(ObjError::UnexpectedToken(token.clone()))
    }
}

fn get_on_off_from_str(token: &Token) -> Result<bool, ObjError> {
    let s = get_token_string(&token)?;
    match s.as_str() {
        "on" => Ok(true),
        "off" => Ok(true),
        _ => Err(ObjError::InvalidOnOffValue(s.clone())),
    }
}
