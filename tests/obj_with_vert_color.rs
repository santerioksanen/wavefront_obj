extern crate proptest;
extern crate wavefront_obj;

use wavefront_obj::obj::{self, Geometry, ObjSet, Object, Shape, TVertex, Vertex};

#[test]
fn decode_obj_with_vert_color() {
  let input = include_str!("obj_with_vert_color.obj");
  let result = obj::parse(input).unwrap();

  let expected = ObjSet {
    material_library: Some("obj_with_vert_color.mtl".to_string()),
    objects: vec![Object {
      name: "".to_string(),
      vertices: vec![
        Vertex {
          x: 1.0,
          y: -1.0,
          z: 2.0,
        },
        Vertex {
          x: 2.0,
          y: -2.0,
          z: 3.0,
        },
        Vertex {
          x: 3.0,
          y: -3.0,
          z: 4.0,
        },
      ],
      tex_vertices: vec![
        TVertex {
          u: 0.1,
          v: 0.2,
          w: 0.0,
        },
        TVertex {
          u: 0.2,
          v: 0.3,
          w: 0.0,
        },
        TVertex {
          u: 0.3,
          v: 0.4,
          w: 0.0,
        },
      ],
      normals: vec![],
      geometry: vec![Geometry {
        material_name: Some("obj_with_vert_color_u1_v1".to_string()),
        shapes: vec![Shape {
          primitive: obj::Primitive::Triangle(
            (2, Some(2), None),
            (0, Some(0), None),
            (1, Some(1), None),
          ),
          groups: vec!["default".to_string()],
          smoothing_groups: vec![],
        }],
      }],
    }],
  };

  assert_eq!(result, expected);
}
