# Overview

`nobject-rs` is a library for parsing wavefront .obj and .mtl content.
To this end, the crate exposes two methos:  
* `load_obj`
* `load_mtl`

Both methods take the content of the respective files (.obj and .mtl),
parse and then return a result with either some kind of parse error, or
a struct containing the data.  

Note that this crate leaves the responsibility of file I/O to the consuming
application. For example, it's possible to specify file names as attributes
in the material, or file names as material libraries in the obj file. This
library will NOT attempt to open and parse those files. It is left to the
consuming application/library to take the file information from the results
of the parse methods, find and open the appropriate files, and then pass on
the contents to be parsed.

# Reference

Parsing is done based on the specification for Obj's and Mtl's found at:
* [Obj]( http://paulbourke.net/dataformats/obj/)
* [Mtl](http://paulbourke.net/dataformats/mtl/)

# Examples

## Obj parsing
```rust
fn main() {
    let input =
    "
    o 1
    v -0.5 -0.5 0.5
    v -0.5 -0.5 -0.5
    v -0.5 0.5 -0.5
    v -0.5 0.5 0.5
    v 0.5 -0.5 0.5
    v 0.5 -0.5 -0.5
    v 0.5 0.5 -0.5
    v 0.5 0.5 0.5
    
    usemtl Default
    f 4 3 2 1
    f 2 6 5 1
    f 3 7 6 2
    f 8 7 3 4
    f 5 8 4 1
    f 6 7 8 5
    ";

    let res = nobject_rs::load_obj(&input).unwrap();
    let group = &res.groups["default"];
    let face_group = &res.faces["default"];
    assert_eq!(res.vertices.len(), 8);
    assert_eq!(group.material_name, "Default".to_string());
    assert_eq!(res.normals.len(), 0);
    assert_eq!(res.faces.len(), 1);
    assert_eq!(face_group.len(), 6);;
}
```

## Mtl parsing
```rust
fn main() {
    let input =
    "newmtl frost_wind
    Ka 0.2 0.2 0.2
    Kd 0.6 0.6 0.6
    Ks 0.1 0.1 0.1
    d 1
    Ns 200
    illum 2
    map_d -mm 0.200 0.800 window.mps";

    let res = nobject_rs::load_mtl(&input).unwrap();
    assert_eq!(res.len(), 1);
}
```