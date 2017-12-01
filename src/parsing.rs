//! Contains parsing routines
use objects::{Object, ObjectKind};

/// Parses the object's kind from its raw representation and
/// the associated request.
///
/// XXX: if we realise that `request` is not needed to determine
/// the object's kind, we can remove it from here and from
/// `objects::Object::from()`.
pub fn parse_object_kind(raw: &[u8], request: &str) -> ObjectKind {
    unimplemented!();
}

/// Parses the target size of an object from its HTTP request.
pub fn parse_target_size(request: &str) -> usize {
    unimplemented!();
}

/// Parses the objects contained in an HTML page.
pub fn parse_objects(html: &Object) -> Vec<Object> {
    unimplemented!();
}
