extern crate proptest;
extern crate wavefront_obj;

use wavefront_obj::obj;

#[test]
fn decode_obj_with_vert_color() {
  let input = include_str!("obj_with_vert_color.obj");
  let result = obj::parse(input).unwrap();
}
