#[cfg(test)]
mod test;
mod tokenizer;

mod model;
mod material;

use std::result::Result;

pub use model::{
    Model,
    ModelError,
};

pub use material::Material;

use thiserror::Error;
use tokenizer::TokenizeError;

#[derive(Error, Debug)]
pub enum ObjError {
    #[error("Tokenize Error: `{0}`")]
    Tokenize(#[from] TokenizeError),

    #[error("Model Error: `{0}`")]
    ModelParse(#[from] ModelError),
}

pub fn load_obj(input: &str) -> Result<Model, ObjError> {
    match tokenizer::parse_obj(input) {
        Ok(tokens) => Ok(model::parse(&tokens)?),
        Err(e) => Err(e.into()),
    }
}

pub fn load_mtl(_input: &str) -> Result<Material, ObjError> {
    Ok(
        Material{}
    )
}