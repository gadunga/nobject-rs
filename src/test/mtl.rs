use crate::tokenizer::{
    parse_mtl,
    Token,
};

#[test]
fn parse_simple_comment() {
    let vert = "# whatever this is a comment";
    let res = parse_mtl(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 0);
}

#[test]
fn parse_simple_comment_2() {
    let vert = "#whatever this is a comment";
    let res = parse_mtl(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 0);
}

#[test]
fn parse_texres() {
    let vert = "-texres 512";
    let res = parse_mtl(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], Token::OptionTextureResolution);
    assert_eq!(tokens[1], Token::Int(512));
}

#[test]
fn parse_turbulence_1() {
    let vert = "-t 1.2";
    let res = parse_mtl(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], Token::OptionTurbulence);
    assert_eq!(tokens[1], Token::Float(1.2));
}

#[test]
fn parse_turbulence_2() {
    let vert = "-t 1.2 1.3";
    let res = parse_mtl(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], Token::OptionTurbulence);
    assert_eq!(tokens[1], Token::Float(1.2));
    assert_eq!(tokens[2], Token::Float(1.3));
}

#[test]
fn parse_turbulence_3() {
    let vert = "-t 1.2 1.3 1.4";
    let res = parse_mtl(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0], Token::OptionTurbulence);
    assert_eq!(tokens[1], Token::Float(1.2));
    assert_eq!(tokens[2], Token::Float(1.3));
    assert_eq!(tokens[3], Token::Float(1.4));
}

#[test]
fn parse_scale_1() {
    let vert = "-s 1.2";
    let res = parse_mtl(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], Token::OptionScale);
    assert_eq!(tokens[1], Token::Float(1.2));
}

#[test]
fn parse_scale_2() {
    let vert = "-s 1.2 1.3";
    let res = parse_mtl(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], Token::OptionScale);
    assert_eq!(tokens[1], Token::Float(1.2));
    assert_eq!(tokens[2], Token::Float(1.3));
}

#[test]
fn parse_scale_3() {
    let vert = "-s 1.2 1.3 1.4";
    let res = parse_mtl(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0], Token::OptionScale);
    assert_eq!(tokens[1], Token::Float(1.2));
    assert_eq!(tokens[2], Token::Float(1.3));
    assert_eq!(tokens[3], Token::Float(1.4));
}

#[test]
fn parse_offset_1() {
    let vert = "-o 1.2";
    let res = parse_mtl(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], Token::OptionOffset);
    assert_eq!(tokens[1], Token::Float(1.2));
}

#[test]
fn parse_offset_2() {
    let vert = "-o 1.2 1.3";
    let res = parse_mtl(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], Token::OptionOffset);
    assert_eq!(tokens[1], Token::Float(1.2));
    assert_eq!(tokens[2], Token::Float(1.3));
}

#[test]
fn parse_offset_3() {
    let vert = "-o 1.2 1.3 1.4";
    let res = parse_mtl(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0], Token::OptionOffset);
    assert_eq!(tokens[1], Token::Float(1.2));
    assert_eq!(tokens[2], Token::Float(1.3));
    assert_eq!(tokens[3], Token::Float(1.4));
}

#[test]
fn parse_range() {
    let vert = "-mm 1.0 2.0";
    let res = parse_mtl(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], Token::OptionRange);
    assert_eq!(tokens[1], Token::Float(1.0));
    assert_eq!(tokens[2], Token::Float(2.0));
}

#[test]
fn parse_imfchan() {
    let vert = "-imfchan r";
    let res = parse_mtl(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], Token::OptionIMFChan);
    assert_eq!(tokens[1], Token::String("r".into()));
}

#[test]
fn parse_clamp() {
    let vert = "-clamp on";
    let res = parse_mtl(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], Token::OptionClamp);
    assert_eq!(tokens[1], Token::String("on".into()));
}

#[test]
fn parse_cc() {
    let vert = "-cc on";
    let res = parse_mtl(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], Token::OptionColorCorrect);
    assert_eq!(tokens[1], Token::String("on".into()));
}

#[test]
fn parse_boost() {
    let vert = "-boost 1.2";
    let res = parse_mtl(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], Token::OptionBoost);
    assert_eq!(tokens[1], Token::Float(1.2));
}

#[test]
fn parse_bm() {
    let vert = "-bm 1.2";
    let res = parse_mtl(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], Token::OptionBumpMultiplier);
    assert_eq!(tokens[1], Token::Float(1.2));
}

#[test]
fn parse_blendv() {
    let vert = "-blendv on";
    let res = parse_mtl(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], Token::OptionBlendV);
    assert_eq!(tokens[1], Token::String("on".into()));
}

#[test]
fn parse_blendu() {
    let vert = "-blendu on";
    let res = parse_mtl(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], Token::OptionBlendU);
    assert_eq!(tokens[1], Token::String("on".into()));
}

#[test]
fn parse_refl_1() {
    let vert = "refl -type sphere -mm 0.2 1.2 clouds.mpc";
    let res = parse_mtl(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    dbg!(&tokens);
    assert_eq!(tokens.len(), 7);
    assert_eq!(tokens[0], Token::ReflectionMap);
    assert_eq!(tokens[1], Token::ReflectionType);
    assert_eq!(tokens[2], Token::String("sphere".into()));
    assert_eq!(tokens[3], Token::OptionRange);
    assert_eq!(tokens[4], Token::Float(0.2));
    assert_eq!(tokens[5], Token::Float(1.2));
    assert_eq!(tokens[6], Token::String("clouds.mpc".into()));
}

#[test]
fn parse_tf_spectral() {
    let vert = "Tf spectral file.rfl 1.0";
    let res = parse_mtl(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    dbg!(&tokens);
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0], Token::TransmissionFactor);
    assert_eq!(tokens[1], Token::Spectral);
    assert_eq!(tokens[2], Token::String("file.rfl".into()));
    assert_eq!(tokens[3], Token::Float(1.0));
}
