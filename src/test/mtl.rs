use crate::{
    material::{
        BumpMap,
        ColorCorrectedMap,
        ColorType,
        DisolveType,
        Material,
        NonColorCorrectedMap,
        ReflectionMap,
    },
    tokenizer::{
        parse_mtl,
        Token,
    },
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

macro_rules! parse_material_test {
    ($name:ident, $val:expr, $exp:expr) => {
        #[test]
        fn $name() {
            let res = crate::load_mtl(&$val).unwrap();
            assert_eq!(res.len(), 1);
            let mat = res.first().unwrap();
            assert_eq!(mat, &$exp);
        }
    };
}

parse_material_test!(
    neon_green,
    "newmtl neon_green
Kd 0.0000 1.0000 0.0000
illum 0",
    Material {
        name: "neon_green".to_string(),
        diffuse: Some(ColorType::Rgb(0.0, 1.0, 0.0)),
        illumination_mode: Some(0),
        ..Default::default()
    }
);

parse_material_test!(
    frosted_window,
    "newmtl frost_wind
    Ka 0.2 0.2 0.2
    Kd 0.6 0.6 0.6
    Ks 0.1 0.1 0.1
    d 1
    Ns 200
    illum 2
    map_d -mm 0.200 0.800 window.mps",
    Material {
        name: "frost_wind".to_string(),
        ambient: Some(ColorType::Rgb(0.2, 0.2, 0.2)),
        diffuse: Some(ColorType::Rgb(0.6, 0.6, 0.6)),
        specular: Some(ColorType::Rgb(0.1, 0.1, 0.1)),
        disolve: Some(DisolveType::Alpha(1.0)),
        specular_exponent: Some(200.0),
        illumination_mode: Some(2),
        disolve_map: Some(NonColorCorrectedMap {
            texture_range: Some((0.2, 0.8)),
            file_name: "window.mps".into(),
            ..Default::default()
        }),
        ..Default::default()
    }
);

parse_material_test!(
    flat_green_test,
    "newmtl flat_green
 Ka 0.0000 1.0000 0.0000
 Kd 0.0000 1.0000 0.0000
 illum 1",
    Material {
        name: "flat_green".into(),
        ambient: Some(ColorType::Rgb(0.0, 1.0, 0.0)),
        diffuse: Some(ColorType::Rgb(0.0, 1.0, 0.0)),
        illumination_mode: Some(1),
        ..Default::default()
    }
);

parse_material_test!(
    pine_wood_test,
    "newmtl pine_wood
 Ka spectral ident.rfl 1
 Kd spectral ident.rfl 1
 illum 1
 map_Ka pine.mpc
 map_Kd pine.mpc",
    Material {
        name: "pine_wood".into(),
        ambient: Some(ColorType::Spectral("ident.rfl".into(), 1.0)),
        diffuse: Some(ColorType::Spectral("ident.rfl".into(), 1.0)),
        illumination_mode: Some(1),
        texture_map_ambient: Some(ColorCorrectedMap {
            file_name: "pine.mpc".into(),
            ..Default::default()
        }),
        texture_map_diffuse: Some(ColorCorrectedMap {
            file_name: "pine.mpc".into(),
            ..Default::default()
        }),
        ..Default::default()
    }
);

parse_material_test!(
    tin_test,
    "newmtl tin
 Ka spectral tin.rfl
 Kd spectral tin.rfl
 Ks spectral tin.rfl
 Ns 200
 illum 3",
    Material {
        name: "tin".into(),
        ambient: Some(ColorType::Spectral("tin.rfl".into(), 1.0)),
        diffuse: Some(ColorType::Spectral("tin.rfl".into(), 1.0)),
        specular: Some(ColorType::Spectral("tin.rfl".into(), 1.0)),
        illumination_mode: Some(3),
        specular_exponent: Some(200.0),
        ..Default::default()
    }
);

parse_material_test!(
    bump_leath_test,
    "newmtl bumpy_leath
 Ka spectral ident.rfl 1
 Kd spectral ident.rfl 1
 Ks spectral ident.rfl 1
 illum 2
 map_Ka brown.mpc
 map_Kd brown.mpc
 map_Ks brown.mpc
 bump -bm 2.000 leath.mpb",
    Material {
        name: "bumpy_leath".into(),
        ambient: Some(ColorType::Spectral("ident.rfl".into(), 1.0)),
        diffuse: Some(ColorType::Spectral("ident.rfl".into(), 1.0)),
        specular: Some(ColorType::Spectral("ident.rfl".into(), 1.0)),
        illumination_mode: Some(2),
        texture_map_ambient: Some(ColorCorrectedMap {
            file_name: "brown.mpc".into(),
            ..Default::default()
        }),
        texture_map_diffuse: Some(ColorCorrectedMap {
            file_name: "brown.mpc".into(),
            ..Default::default()
        }),
        texture_map_specular: Some(ColorCorrectedMap {
            file_name: "brown.mpc".into(),
            ..Default::default()
        }),
        bump_map: Some(BumpMap {
            bump_multiplier: Some(2.0),
            map_settings:    Some(NonColorCorrectedMap {
                file_name: "leath.mpb".into(),
                ..Default::default()
            }),
        }),
        ..Default::default()
    }
);

parse_material_test!(
    logo_test,
    "newmtl logo
 Ka spectral ident.rfl 1
 Kd spectral ident.rfl 1
 Ks spectral ident.rfl 1
 illum 2
 map_Ka -s 1.200 1.200 0.000 logo.mpc
 map_Kd -s 1.200 1.200 0.000 logo.mpc
 map_Ks -s 1.200 1.200 0.000 logo.mpc",
    Material {
        name: "logo".into(),
        ambient: Some(ColorType::Spectral("ident.rfl".into(), 1.0)),
        diffuse: Some(ColorType::Spectral("ident.rfl".into(), 1.0)),
        specular: Some(ColorType::Spectral("ident.rfl".into(), 1.0)),
        illumination_mode: Some(2),
        texture_map_ambient: Some(ColorCorrectedMap {
            file_name: "logo.mpc".into(),
            scale: Some((1.2, Some(1.2), Some(0.0))),
            ..Default::default()
        }),
        texture_map_diffuse: Some(ColorCorrectedMap {
            file_name: "logo.mpc".into(),
            scale: Some((1.2, Some(1.2), Some(0.0))),
            ..Default::default()
        }),
        texture_map_specular: Some(ColorCorrectedMap {
            file_name: "logo.mpc".into(),
            scale: Some((1.2, Some(1.2), Some(0.0))),
            ..Default::default()
        }),
        ..Default::default()
    }
);

parse_material_test!(
    reflection_material,
    "newmtl reflection
 ka 0 0 0
 kd 0 0 0
 ks .7 .7 .7
 illum 1
 refl -type sphere chrome.rla",
    Material {
        name: "reflection".into(),
        ambient: Some(ColorType::Rgb(0.0, 0.0, 0.0)),
        diffuse: Some(ColorType::Rgb(0.0, 0.0, 0.0)),
        specular: Some(ColorType::Rgb(0.7, 0.7, 0.7)),
        illumination_mode: Some(1),
        reflection_map: Some(ReflectionMap {
            reflection_type: "sphere".into(),
            map_settings:    Some(ColorCorrectedMap {
                file_name: "chrome.rla".into(),
                ..Default::default()
            }),
        }),
        ..Default::default()
    }
);

parse_material_test!(
    moon_test,
    "# Material Count: 1
    
    newmtl Moon
    Ns 96.078431
    Ka 1.000000 1.000000 1.000000
    Kd 0.640000 0.640000 0.640000
    Ks 0.500000 0.500000 0.500000
    Ke 0.000000 0.000000 0.000000
    Ni 1.000000
    d 1.000000
    map_Kd -o 0 0 0 Diffuse_2K.png
    bump -o 0 0 0 Bump_2K.png
    illum 2
    ",
    Material {
        name: "Moon".into(),
        ambient: Some(ColorType::Rgb(1.0, 1.0, 1.0)),
        diffuse: Some(ColorType::Rgb(0.64, 0.64, 0.64)),
        specular: Some(ColorType::Rgb(0.5, 0.5, 0.5)),
        emissive_coefficient: Some(ColorType::Rgb(0.0, 0.0, 0.0)),
        specular_exponent: Some(96.078431),
        index_of_refraction: Some(1.0),
        disolve: Some(DisolveType::Alpha(1.0)),
        texture_map_diffuse: Some(ColorCorrectedMap {
            offset: Some((0.0, Some(0.0), Some(0.0))),
            file_name: "Diffuse_2K.png".into(),
            ..Default::default()
        }),
        bump_map: Some(BumpMap {
            map_settings: Some(NonColorCorrectedMap {
                offset: Some((0.0, Some(0.0), Some(0.0))),
                file_name: "Bump_2K.png".into(),
                ..Default::default()
            }),
            ..Default::default()
        }),
        illumination_mode: Some(2),
        ..Default::default()
    }
);

parse_material_test!(
    sponza_mat_test,
    "newmtl Material__25
    Ns 7.843137
    Ka 0.000000 0.000000 0.000000
    Kd 0.470400 0.470400 0.470400
    Ks 0.000000 0.000000 0.000000
    Ni 1.000000
    d 0.000000
    illum 2
    map_Kd textures/lion.tga
    map_Disp textures/lion_ddn.tga
    map_Ka textures/lion.tga",
    Material {
        name: "Material__25".into(),
        ambient: Some(ColorType::Rgb(0.0, 0.0, 0.0)),
        diffuse: Some(ColorType::Rgb(0.470400, 0.470400, 0.470400)),
        specular: Some(ColorType::Rgb(0.0, 0.0, 0.0)),
        specular_exponent: Some(7.843137),
        index_of_refraction: Some(1.0),
        disolve: Some(DisolveType::Alpha(0.0)),
        texture_map_diffuse: Some(ColorCorrectedMap {
            file_name: "textures/lion.tga".into(),
            ..Default::default()
        }),
        texture_map_ambient: Some(ColorCorrectedMap {
            file_name: "textures/lion.tga".into(),
            ..Default::default()
        }),
        displacement_map: Some(NonColorCorrectedMap {
            file_name: "textures/lion_ddn.tga".into(),
            ..Default::default()
        }),
        illumination_mode: Some(2),
        ..Default::default()
    }
);
