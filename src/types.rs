
pub type ObjectType = usize;

// Object types
pub const ALPACA_T: ObjectType = 0;
pub const HTML_T: ObjectType = 1;
pub const CSS_T: ObjectType = 2;
pub const IMG_T: ObjectType = 3;
pub const UNKNOWN_T: ObjectType = 4;

pub extern fn parse_object_type(object: &[u8], request: &str) -> ObjectType {
    unimplemented!();
}
