mod mtl;
mod obj;

use super::*;

#[test]
fn parse_double_comment_test() {
    let content = "#  Stanford Bunny
    #  Normals but no textures
    
    v 0.1102022 0.74011 1.132398
    vn -1 0.000157759 5.71832e-005
    f 11250//11250 4406//4406 31248//31248
    f 9238//9238 25314//25314 21852//21852";
    let model = load_obj(&content).unwrap();
    assert_eq!(model.vertices.len(), 1);
    assert_eq!(model.vertices[0].x, 0.1102022);
    assert_eq!(model.vertices[0].y, 0.74011);
    assert_eq!(model.vertices[0].z, 1.132398);
    assert_eq!(model.normals.len(), 1);
    assert_eq!(model.normals[0].x, -1.0);
    assert_eq!(model.normals[0].y, 0.000157759);
    assert_eq!(model.normals[0].z, 0.0000571832);
    let faces = &model.faces["default"];
    assert_eq!(faces.len(), 2);
    let face = &faces[0];
    assert_eq!(
        face.elements[0],
        FaceElement {
            vertex_index:  11250,
            normal_index:  Some(11250),
            texture_index: None,
        }
    );
    assert_eq!(
        face.elements[1],
        FaceElement {
            vertex_index:  4406,
            normal_index:  Some(4406),
            texture_index: None,
        }
    );
    assert_eq!(
        face.elements[2],
        FaceElement {
            vertex_index:  31248,
            normal_index:  Some(31248),
            texture_index: None,
        }
    );
    let face = &faces[1];
    assert_eq!(
        face.elements[0],
        FaceElement {
            vertex_index:  9238,
            normal_index:  Some(9238),
            texture_index: None,
        }
    );
    assert_eq!(
        face.elements[1],
        FaceElement {
            vertex_index:  25314,
            normal_index:  Some(25314),
            texture_index: None,
        }
    );
    assert_eq!(
        face.elements[2],
        FaceElement {
            vertex_index:  21852,
            normal_index:  Some(21852),
            texture_index: None,
        }
    );
}
