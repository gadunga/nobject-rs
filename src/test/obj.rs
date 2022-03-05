use crate::model::ModelElement;
use crate::{
    model,
    model::{Face, FaceElement, Vertex},
    tokenizer::{parse_obj, Token},
    Line, LineElement, Material, Point, Texture,
};

#[test]
fn parse_simple_comment() {
    let vert = "# whatever this is a comment";
    let res = parse_obj(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 0);
}

#[test]
fn parse_simple_comment_2() {
    let vert = "#whatever this is a comment";
    let res = parse_obj(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 0);
}

#[test]
fn parse_line_token_with_comment() {
    let vert = "l # blah blah blah";
    let res = parse_obj(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0], Token::Line);
}

#[test]
fn parse_vertex() {
    let vert = "v 0.123 0.234 0.345 1.0";
    let res = parse_obj(vert);
    dbg!(&res);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0], Token::Vertex);
    assert_eq!(tokens[1], Token::Float(0.123));
    assert_eq!(tokens[2], Token::Float(0.234));
    assert_eq!(tokens[3], Token::Float(0.345));
    assert_eq!(tokens[4], Token::Float(1.0));
}

#[test]
fn parse_vertex_multiline_test() {
    let vert = "v 0.123 0.234 0.345 1.0\nv 1.123 1.234 1.345 1.0";
    let res = parse_obj(vert);

    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 10);
    assert_eq!(tokens[0], Token::Vertex);
    assert_eq!(tokens[1], Token::Float(0.123));
    assert_eq!(tokens[2], Token::Float(0.234));
    assert_eq!(tokens[3], Token::Float(0.345));
    assert_eq!(tokens[4], Token::Float(1.0));

    assert_eq!(tokens[5], Token::Vertex);
    assert_eq!(tokens[6], Token::Float(1.123));
    assert_eq!(tokens[7], Token::Float(1.234));
    assert_eq!(tokens[8], Token::Float(1.345));
    assert_eq!(tokens[9], Token::Float(1.0));
}

#[test]
fn parse_vertex_normal() {
    let vert = "vn 0.123 0.234 0.345";
    let res = parse_obj(vert);
    dbg!(&res);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0], Token::VertexNormal);
    assert_eq!(tokens[1], Token::Float(0.123));
    assert_eq!(tokens[2], Token::Float(0.234));
    assert_eq!(tokens[3], Token::Float(0.345));
}

#[test]
fn parse_vertex_texture() {
    let vert = "vt 0.500 1";
    let res = parse_obj(vert);
    dbg!(&res);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0], Token::VertexTexture);
    assert_eq!(tokens[1], Token::Float(0.500));
    assert_eq!(tokens[2], Token::Int(1));

    let res = model::parse_vertex_texture(&tokens);
    dbg!(&res);
    assert!(res.is_ok());
    let (extra, texture) = res.ok().unwrap();
    assert_eq!(extra.len(), 0);
    let expected = Texture {
        u: 0.500,
        v: Some(1.0),
        w: None,
    };
    assert_eq!(texture, expected);
}

#[test]
fn parse_vertex_texture2() {
    let vert = "vt 0.500 1 0.75";
    let res = parse_obj(vert);
    dbg!(&res);
    assert!(res.is_ok());
    let tokens = res.unwrap();

    let res = model::parse_vertex_texture(&tokens);
    dbg!(&res);
    assert!(res.is_ok());
    let (extra, texture) = res.ok().unwrap();
    assert_eq!(extra.len(), 0);
    let expected = Texture {
        u: 0.500,
        v: Some(1.0),
        w: Some(0.75),
    };
    assert_eq!(texture, expected);
}

#[test]
fn parse_face() {
    let vert = "f 1 2 3";
    let res = parse_obj(vert);
    dbg!(&res);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0], Token::Face);
    assert_eq!(tokens[1], Token::Int(1));
    assert_eq!(tokens[2], Token::Int(2));
    assert_eq!(tokens[3], Token::Int(3));

    let res = model::parse_face(&tokens);
    dbg!(&res);
    assert!(res.is_ok());
    let (extra, face) = res.ok().unwrap();
    assert_eq!(extra.len(), 0);
    assert_eq!(face.elements.len(), 3);
    let expected = Face {
        elements: vec![
            FaceElement {
                vertex_index: 1,
                texture_index: None,
                normal_index: None,
            },
            FaceElement {
                vertex_index: 2,
                texture_index: None,
                normal_index: None,
            },
            FaceElement {
                vertex_index: 3,
                texture_index: None,
                normal_index: None,
            },
        ],
        smoothing_group: 0,
    };
    assert_eq!(face, expected);
}

#[test]
fn parse_face_1() {
    let vert = "f 1/2 2/3 3/4";
    let res = parse_obj(vert);
    dbg!(&res);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 10);
    assert_eq!(tokens[0], Token::Face);
    assert_eq!(tokens[1], Token::Int(1));
    assert_eq!(tokens[2], Token::Slash);
    assert_eq!(tokens[3], Token::Int(2));
    assert_eq!(tokens[4], Token::Int(2));
    assert_eq!(tokens[5], Token::Slash);
    assert_eq!(tokens[6], Token::Int(3));
    assert_eq!(tokens[7], Token::Int(3));
    assert_eq!(tokens[8], Token::Slash);
    assert_eq!(tokens[9], Token::Int(4));

    let res = model::parse_face(&tokens);
    dbg!(&res);
    let (extra, face) = res.ok().unwrap();
    assert_eq!(extra.len(), 0);
    assert_eq!(face.elements.len(), 3);
    let expected = Face {
        elements: vec![
            FaceElement {
                vertex_index: 1,
                texture_index: Some(2),
                normal_index: None,
            },
            FaceElement {
                vertex_index: 2,
                texture_index: Some(3),
                normal_index: None,
            },
            FaceElement {
                vertex_index: 3,
                texture_index: Some(4),
                normal_index: None,
            },
        ],
        smoothing_group: 0,
    };
    assert_eq!(face, expected);
}

#[test]
fn parse_face_2() {
    let vert = "f 1/2/3 2/3/4 3/4/5";
    let res = parse_obj(vert);
    dbg!(&res);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 16);
    assert_eq!(tokens[0], Token::Face);
    assert_eq!(tokens[1], Token::Int(1));
    assert_eq!(tokens[2], Token::Slash);
    assert_eq!(tokens[3], Token::Int(2));
    assert_eq!(tokens[4], Token::Slash);
    assert_eq!(tokens[5], Token::Int(3));
    assert_eq!(tokens[6], Token::Int(2));
    assert_eq!(tokens[7], Token::Slash);
    assert_eq!(tokens[8], Token::Int(3));
    assert_eq!(tokens[9], Token::Slash);
    assert_eq!(tokens[10], Token::Int(4));
    assert_eq!(tokens[11], Token::Int(3));
    assert_eq!(tokens[12], Token::Slash);
    assert_eq!(tokens[13], Token::Int(4));
    assert_eq!(tokens[14], Token::Slash);
    assert_eq!(tokens[15], Token::Int(5));

    let res = model::parse_face(&tokens);
    dbg!(&res);
    let (extra, face) = res.ok().unwrap();
    assert_eq!(extra.len(), 0);
    assert_eq!(face.elements.len(), 3);
    let expected = Face {
        elements: vec![
            FaceElement {
                vertex_index: 1,
                texture_index: Some(2),
                normal_index: Some(3),
            },
            FaceElement {
                vertex_index: 2,
                texture_index: Some(3),
                normal_index: Some(4),
            },
            FaceElement {
                vertex_index: 3,
                texture_index: Some(4),
                normal_index: Some(5),
            },
        ],
        smoothing_group: 0,
    };
    assert_eq!(face, expected);
}

#[test]
fn parse_face_3() {
    let vert = "f 1//2 2//3 3//4";
    let res = parse_obj(vert);
    dbg!(&res);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 13);
    assert_eq!(tokens[0], Token::Face);
    assert_eq!(tokens[1], Token::Int(1));
    assert_eq!(tokens[2], Token::Slash);
    assert_eq!(tokens[3], Token::Slash);
    assert_eq!(tokens[4], Token::Int(2));
    assert_eq!(tokens[5], Token::Int(2));
    assert_eq!(tokens[6], Token::Slash);
    assert_eq!(tokens[7], Token::Slash);
    assert_eq!(tokens[8], Token::Int(3));
    assert_eq!(tokens[9], Token::Int(3));
    assert_eq!(tokens[10], Token::Slash);
    assert_eq!(tokens[11], Token::Slash);
    assert_eq!(tokens[12], Token::Int(4));

    let res = model::parse_face(&tokens);
    dbg!(&res);
    let (extra, face) = res.ok().unwrap();
    assert_eq!(extra.len(), 0);
    assert_eq!(face.elements.len(), 3);
    let expected = Face {
        elements: vec![
            FaceElement {
                vertex_index: 1,
                texture_index: None,
                normal_index: Some(2),
            },
            FaceElement {
                vertex_index: 2,
                texture_index: None,
                normal_index: Some(3),
            },
            FaceElement {
                vertex_index: 3,
                texture_index: None,
                normal_index: Some(4),
            },
        ],
        smoothing_group: 0,
    };
    assert_eq!(face, expected);
}

#[test]
fn parse_face_4() {
    //According to http://paulbourke.net/dataformats/obj/ this is illegal syntax
    //As a leniant parser it should parse ok
    let vert = "f 1/1/1 2/2/2 3//3 4//4";
    let res = parse_obj(vert);
    dbg!(&res);
    assert!(res.is_ok());
    let tokens = res.unwrap();

    let res = model::parse_face(&tokens);
    dbg!(&res);
    let (extra, face) = res.ok().unwrap();
    assert_eq!(extra.len(), 0);
    assert_eq!(face.elements.len(), 4);
    let expected = Face {
        elements: vec![
            FaceElement {
                vertex_index: 1,
                texture_index: Some(1),
                normal_index: Some(1),
            },
            FaceElement {
                vertex_index: 2,
                texture_index: Some(2),
                normal_index: Some(2),
            },
            FaceElement {
                vertex_index: 3,
                texture_index: None,
                normal_index: Some(3),
            },
            FaceElement {
                vertex_index: 4,
                texture_index: None,
                normal_index: Some(4),
            },
        ],
        smoothing_group: 0,
    };
    assert_eq!(face, expected);
}

#[test]
#[ignore]
fn parse_face_trailing_slash() {
    //This could be an edge case but it is my interpretation that this is legal
    let vert = "f 1/ 2/ 3/";
    let res = parse_obj(vert);
    dbg!(&res);
    assert!(res.is_ok());
    let tokens = res.unwrap();

    let res = model::parse_face(&tokens);
    dbg!(&res);
    let (extra, face) = res.ok().unwrap();
    assert_eq!(extra.len(), 0);
    assert_eq!(face.elements.len(), 3);
    let expected = Face {
        elements: vec![
            FaceElement {
                vertex_index: 1,
                texture_index: None,
                normal_index: None,
            },
            FaceElement {
                vertex_index: 2,
                texture_index: None,
                normal_index: None,
            },
            FaceElement {
                vertex_index: 3,
                texture_index: None,
                normal_index: None,
            },
        ],
        smoothing_group: 0,
    };
    assert_eq!(face, expected);
}

#[test]
#[ignore]
fn parse_face_trailing_slash_slash() {
    //This could be an edge case but it is my interpretation that this is legal
    let vert = "f 1// 2// 3//";
    let res = parse_obj(vert);
    dbg!(&res);
    assert!(res.is_ok());
    let tokens = res.unwrap();

    let res = model::parse_face(&tokens);
    dbg!(&res);
    let (extra, face) = res.ok().unwrap();
    assert_eq!(extra.len(), 0);
    assert_eq!(face.elements.len(), 3);
    let expected = Face {
        elements: vec![
            FaceElement {
                vertex_index: 1,
                texture_index: None,
                normal_index: None,
            },
            FaceElement {
                vertex_index: 2,
                texture_index: None,
                normal_index: None,
            },
            FaceElement {
                vertex_index: 3,
                texture_index: None,
                normal_index: None,
            },
        ],
        smoothing_group: 0,
    };
    assert_eq!(face, expected);
}

#[test]
fn parse_point() {
    let vert = "p 1 2 3";
    let res = parse_obj(vert);
    dbg!(&res);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0], Token::Point);
    assert_eq!(tokens[1], Token::Int(1));
    assert_eq!(tokens[2], Token::Int(2));
    assert_eq!(tokens[3], Token::Int(3));

    let res = model::parse_point(&tokens);
    dbg!(&res);
    assert!(res.is_ok());
    let (extra, point) = res.ok().unwrap();
    assert_eq!(extra.len(), 0);
    assert_eq!(point.elements.len(), 3);
    let expected = Point {
        elements: vec![1, 2, 3],
    };
    assert_eq!(point, expected);
}

#[test]
fn parse_line() {
    let vert = "l 1 2 3";
    let res = parse_obj(vert);
    dbg!(&res);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0], Token::Line);
    assert_eq!(tokens[1], Token::Int(1));
    assert_eq!(tokens[2], Token::Int(2));
    assert_eq!(tokens[3], Token::Int(3));

    let res = model::parse_line(&tokens);
    dbg!(&res);
    assert!(res.is_ok());
    let (extra, line) = res.ok().unwrap();
    assert_eq!(extra.len(), 0);
    assert_eq!(line.elements.len(), 3);
    let expected = Line {
        elements: vec![
            LineElement {
                vertex_index: 1,
                texture_index: None,
            },
            LineElement {
                vertex_index: 2,
                texture_index: None,
            },
            LineElement {
                vertex_index: 3,
                texture_index: None,
            },
        ],
    };
    assert_eq!(line, expected);
}

#[test]
fn parse_line_texture_struct() {
    let vert = "l 1/4 2/5 3/6";
    let res = parse_obj(vert);
    dbg!(&res);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    let res = model::parse_line(&tokens);
    dbg!(&res);
    assert!(res.is_ok());
    let (extra, line) = res.ok().unwrap();
    assert_eq!(extra.len(), 0);
    assert_eq!(line.elements.len(), 3);
    let expected = Line {
        elements: vec![
            LineElement {
                vertex_index: 1,
                texture_index: Some(4),
            },
            LineElement {
                vertex_index: 2,
                texture_index: Some(5),
            },
            LineElement {
                vertex_index: 3,
                texture_index: Some(6),
            },
        ],
    };
    assert_eq!(line, expected);
}

#[test]
#[ignore]
fn parse_line_trailing_slash_struct() {
    //This could be an edge case but it is my interpretation that this is legal
    let vert = "l 1/ 2/ 3/";
    let res = parse_obj(vert);
    dbg!(&res);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    let res = model::parse_line(&tokens);
    dbg!(&res);
    assert!(res.is_ok());
    let (extra, line) = res.ok().unwrap();
    assert_eq!(extra.len(), 0);
    assert_eq!(line.elements.len(), 3);
    let expected = Line {
        elements: vec![
            LineElement {
                vertex_index: 1,
                texture_index: None,
            },
            LineElement {
                vertex_index: 2,
                texture_index: None,
            },
            LineElement {
                vertex_index: 3,
                texture_index: None,
            },
        ],
    };
    assert_eq!(line, expected);
}

#[test]
fn simple_material() {
    let vert = "mtllib some_mtl_file.mtl";
    let res = parse_obj(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    dbg!(&tokens);
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], Token::MaterialLib);
    assert_eq!(tokens[1], Token::String("some_mtl_file.mtl".to_string()));

    let res = model::parse_mat_lib(&tokens);
    dbg!(&res);
    assert!(res.is_ok());
    let (extra, model) = res.ok().unwrap();
    assert_eq!(extra.len(), 0);

    assert_eq!(
        model,
        ModelElement::MaterialLib(vec!["some_mtl_file.mtl".to_string()])
    );
}

#[test]
fn simple_group() {
    let vert = "g some_group";
    let res = parse_obj(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    dbg!(&tokens);
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], Token::Group);
    assert_eq!(tokens[1], Token::String("some_group".to_string()));

    let res = model::parse_group(&tokens);
    dbg!(&res);
    assert!(res.is_ok());
    let (extra, model) = res.ok().unwrap();
    assert_eq!(extra.len(), 0);

    assert_eq!(model, ModelElement::Group(vec!["some_group".to_string()]));
}

#[test]
fn simple_object() {
    let vert = "o some_object";
    let res = parse_obj(vert);
    assert!(res.is_ok());
    let tokens = res.unwrap();
    dbg!(&tokens);
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0], Token::Object);
    assert_eq!(tokens[1], Token::String("some_object".to_string()));

    let res = model::parse_obj_name(&tokens);
    dbg!(&res);
    assert!(res.is_ok());
    let (extra, model) = res.ok().unwrap();
    assert_eq!(extra.len(), 0);

    assert_eq!(model, ModelElement::ObjName("some_object".to_string()));
}

#[test]
fn cube_test() {
    let input = "#	                Vertices: 8
    #	                  Points: 0
    #	                   Lines: 0
    #	                   Faces: 6
    #	               Materials: 1
    
    o 1
    
    # Vertex list
    
    v -0.5 -0.5 0.5
    v -0.5 -0.5 -0.5
    v -0.5 0.5 -0.5
    v -0.5 0.5 0.5
    v 0.5 -0.5 0.5
    v 0.5 -0.5 -0.5
    v 0.5 0.5 -0.5
    v 0.5 0.5 0.5
    
    # Point/Line/Face list
    
    usemtl Default
    f 4 3 2 1
    f 2 6 5 1
    f 3 7 6 2
    f 8 7 3 4
    f 5 8 4 1
    f 6 7 8 5
    
    # End of file
    ";

    let res = crate::load_obj(&input).unwrap();
    dbg!(&res);
    assert_eq!(res.vertices.len(), 8);
    assert_eq!(
        res.vertices[0],
        Vertex {
            x: -0.5,
            y: -0.5,
            z: 0.5,
            w: None,
        }
    );
    assert_eq!(
        res.vertices[1],
        Vertex {
            x: -0.5,
            y: -0.5,
            z: -0.5,
            w: None,
        }
    );
    assert_eq!(
        res.vertices[2],
        Vertex {
            x: -0.5,
            y: 0.5,
            z: -0.5,
            w: None,
        }
    );
    assert_eq!(
        res.vertices[3],
        Vertex {
            x: -0.5,
            y: 0.5,
            z: 0.5,
            w: None,
        }
    );
    assert_eq!(
        res.vertices[4],
        Vertex {
            x: 0.5,
            y: -0.5,
            z: 0.5,
            w: None,
        }
    );
    assert_eq!(
        res.vertices[5],
        Vertex {
            x: 0.5,
            y: -0.5,
            z: -0.5,
            w: None,
        }
    );
    assert_eq!(
        res.vertices[6],
        Vertex {
            x: 0.5,
            y: 0.5,
            z: -0.5,
            w: None,
        }
    );
    assert_eq!(
        res.vertices[7],
        Vertex {
            x: 0.5,
            y: 0.5,
            z: 0.5,
            w: None,
        }
    );

    let group = &res.groups["default"];
    assert_eq!(group.material_name, "Default".to_string());
    assert_eq!(res.normals.len(), 0);
    assert_eq!(res.faces.len(), 1);
    let face_group = &res.faces["default"];
    assert_eq!(face_group.len(), 6);
    assert_eq!(
        face_group[0],
        Face {
            elements: vec![
                FaceElement {
                    vertex_index: 4,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 3,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 2,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 1,
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    );
    assert_eq!(
        face_group[1],
        Face {
            elements: vec![
                FaceElement {
                    vertex_index: 2,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 6,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 5,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 1,
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    );
    assert_eq!(
        face_group[2],
        Face {
            elements: vec![
                FaceElement {
                    vertex_index: 3,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 7,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 6,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 2,
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    );
    assert_eq!(
        face_group[3],
        Face {
            elements: vec![
                FaceElement {
                    vertex_index: 8,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 7,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 3,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 4,
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    );
    assert_eq!(
        face_group[4],
        Face {
            elements: vec![
                FaceElement {
                    vertex_index: 5,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 8,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 4,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 1,
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    );
    assert_eq!(
        face_group[5],
        Face {
            elements: vec![
                FaceElement {
                    vertex_index: 6,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 7,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 8,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 5,
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    );
}

#[test]
fn cube_test_interspersed() {
    let input = "#	                Vertices: 8
    #	                  Points: 0
    #	                   Lines: 0
    #	                   Faces: 6
    #	               Materials: 1

    o 1

    usemtl Default
    # Vertex list

    v -0.5 -0.5 0.5
    v -0.5 -0.5 -0.5
    v -0.5 0.5 -0.5
    v -0.5 0.5 0.5
    f 4 3 2 1

    v 0.5 -0.5 0.5
    v 0.5 -0.5 -0.5
    f 2 6 5 1

    v 0.5 0.5 -0.5
    f 3 7 6 2
    v 0.5 0.5 0.5

    f 8 7 3 4
    f 5 8 4 1
    f 6 7 8 5

    # End of file
    ";

    let res = crate::load_obj(&input).unwrap();
    dbg!(&res);
    assert_eq!(res.vertices.len(), 8);
    assert_eq!(
        res.vertices[0],
        Vertex {
            x: -0.5,
            y: -0.5,
            z: 0.5,
            w: None,
        }
    );
    assert_eq!(
        res.vertices[1],
        Vertex {
            x: -0.5,
            y: -0.5,
            z: -0.5,
            w: None,
        }
    );
    assert_eq!(
        res.vertices[2],
        Vertex {
            x: -0.5,
            y: 0.5,
            z: -0.5,
            w: None,
        }
    );
    assert_eq!(
        res.vertices[3],
        Vertex {
            x: -0.5,
            y: 0.5,
            z: 0.5,
            w: None,
        }
    );
    assert_eq!(
        res.vertices[4],
        Vertex {
            x: 0.5,
            y: -0.5,
            z: 0.5,
            w: None,
        }
    );
    assert_eq!(
        res.vertices[5],
        Vertex {
            x: 0.5,
            y: -0.5,
            z: -0.5,
            w: None,
        }
    );
    assert_eq!(
        res.vertices[6],
        Vertex {
            x: 0.5,
            y: 0.5,
            z: -0.5,
            w: None,
        }
    );
    assert_eq!(
        res.vertices[7],
        Vertex {
            x: 0.5,
            y: 0.5,
            z: 0.5,
            w: None,
        }
    );

    let group = &res.groups["default"];
    assert_eq!(group.material_name, "Default".to_string());
    assert_eq!(res.normals.len(), 0);
    assert_eq!(res.faces.len(), 1);
    let face_group = &res.faces["default"];
    assert_eq!(face_group.len(), 6);
    assert_eq!(
        face_group[0],
        Face {
            elements: vec![
                FaceElement {
                    vertex_index: 4,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 3,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 2,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 1,
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    );
    assert_eq!(
        face_group[1],
        Face {
            elements: vec![
                FaceElement {
                    vertex_index: 2,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 6,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 5,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 1,
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    );
    assert_eq!(
        face_group[2],
        Face {
            elements: vec![
                FaceElement {
                    vertex_index: 3,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 7,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 6,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 2,
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    );
    assert_eq!(
        face_group[3],
        Face {
            elements: vec![
                FaceElement {
                    vertex_index: 8,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 7,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 3,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 4,
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    );
    assert_eq!(
        face_group[4],
        Face {
            elements: vec![
                FaceElement {
                    vertex_index: 5,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 8,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 4,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 1,
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    );
    assert_eq!(
        face_group[5],
        Face {
            elements: vec![
                FaceElement {
                    vertex_index: 6,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 7,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 8,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 5,
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    );
}

#[test]
#[ignore]
fn cube_test_minus() {
    let input = "#	                Vertices: 8
    #	                  Points: 0
    #	                   Lines: 0
    #	                   Faces: 6
    #	               Materials: 1

    o 1

    # Vertex list

    v -0.5 -0.5 0.5
    v -0.5 -0.5 -0.5
    v -0.5 0.5 -0.5
    v -0.5 0.5 0.5
    v 0.5 -0.5 0.5
    v 0.5 -0.5 -0.5
    v 0.5 0.5 -0.5
    v 0.5 0.5 0.5

    # Point/Line/Face list

    usemtl Default
    # f 4 3 2 1
    # f 2 6 5 1
    # f 3 7 6 2
    # f 8 7 3 4
    # f 5 8 4 1
    # f 6 7 8 5

    f -5 -6 -7 -8
    f -7 -3 -4 -8
    f -6 -2 -3 -7
    f -1 -2 -6 -5
    f -4 -1 -5 -8
    f -3 -2 -1 -4

    # End of file
    ";

    let res = crate::load_obj(&input).unwrap();
    dbg!(&res);
    assert_eq!(res.vertices.len(), 8);
    assert_eq!(
        res.vertices[0],
        Vertex {
            x: -0.5,
            y: -0.5,
            z: 0.5,
            w: None,
        }
    );
    assert_eq!(
        res.vertices[1],
        Vertex {
            x: -0.5,
            y: -0.5,
            z: -0.5,
            w: None,
        }
    );
    assert_eq!(
        res.vertices[2],
        Vertex {
            x: -0.5,
            y: 0.5,
            z: -0.5,
            w: None,
        }
    );
    assert_eq!(
        res.vertices[3],
        Vertex {
            x: -0.5,
            y: 0.5,
            z: 0.5,
            w: None,
        }
    );
    assert_eq!(
        res.vertices[4],
        Vertex {
            x: 0.5,
            y: -0.5,
            z: 0.5,
            w: None,
        }
    );
    assert_eq!(
        res.vertices[5],
        Vertex {
            x: 0.5,
            y: -0.5,
            z: -0.5,
            w: None,
        }
    );
    assert_eq!(
        res.vertices[6],
        Vertex {
            x: 0.5,
            y: 0.5,
            z: -0.5,
            w: None,
        }
    );
    assert_eq!(
        res.vertices[7],
        Vertex {
            x: 0.5,
            y: 0.5,
            z: 0.5,
            w: None,
        }
    );

    let group = &res.groups["default"];
    assert_eq!(group.material_name, "Default".to_string());
    assert_eq!(res.normals.len(), 0);
    assert_eq!(res.faces.len(), 1);
    let face_group = &res.faces["default"];
    assert_eq!(face_group.len(), 6);
    assert_eq!(
        face_group[0],
        Face {
            elements: vec![
                FaceElement {
                    vertex_index: 4,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 3,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 2,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 1,
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    );
    assert_eq!(
        face_group[1],
        Face {
            elements: vec![
                FaceElement {
                    vertex_index: 2,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 6,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 5,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 1,
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    );
    assert_eq!(
        face_group[2],
        Face {
            elements: vec![
                FaceElement {
                    vertex_index: 3,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 7,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 6,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 2,
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    );
    assert_eq!(
        face_group[3],
        Face {
            elements: vec![
                FaceElement {
                    vertex_index: 8,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 7,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 3,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 4,
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    );
    assert_eq!(
        face_group[4],
        Face {
            elements: vec![
                FaceElement {
                    vertex_index: 5,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 8,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 4,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 1,
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    );
    assert_eq!(
        face_group[5],
        Face {
            elements: vec![
                FaceElement {
                    vertex_index: 6,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 7,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 8,
                    ..Default::default()
                },
                FaceElement {
                    vertex_index: 5,
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    );
}
