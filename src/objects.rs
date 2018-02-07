//! Defines object data model used by libalpaca.
use parsing::parse_object_kind;

/// Defines our basic object types, each of which has a corresponding
/// unique (distribution, padding type) tuple.
#[derive(PartialEq)]
pub enum ObjectKind {
    /// Fake "padding" object
    Alpaca,
    /// HTML body
    HTML,
    /// CSS
    CSS,
    /// IMG: PNG, JPEG, etc.
    IMG,
    /// Used when our parser cannot determine the object type
    Unknown,
}

/// An object to be used in the morphing process.
pub struct Object {
    /// Type of the Object
    pub kind: ObjectKind,
    /// Content (Vector of bytes) of the Object
    pub content: Vec<u8>,
    /// Position in the HTML body
    pub position: Option<usize>,
    /// Size to pad the Object to
    pub target_size: Option<usize>,
}

impl Object {
    /// Construct an Object given an array of bytes and the HTML request str
    pub fn from(raw: &[u8], request: &str) -> Object {
        Object {
            kind: parse_object_kind(raw, request),
            content: raw.to_vec(),
            position: None,
            target_size: None,
        }
    }

    /// Returns a raw pointer to our Object's 'content' field's slice's buffer.
    /// "The caller must ensure that the slice outlives the pointer this
    /// function returns, or else it will end up pointing to garbage."
    pub fn as_ptr(self) -> *const u8 {
        self.content.as_ptr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::distributions::{IndependentSample, Range};
    use rand::{Rng, weak_rng};

    #[test]
    fn test_object_from_and_as_ptr_jpg() {
        let mut rng = weak_rng();
        let raw_len = Range::new(0, 50).ind_sample(&mut rng);
        let raw = rng.gen_iter::<u8>().take(raw_len).collect::<Vec<u8>>();
        let object = Object {
            kind: ObjectKind::IMG,
            content: raw.to_vec(),
            position: None,
            target_size: None,
        };
        assert_eq!(object.content.len(), raw_len);
        assert!(match object.kind {
            ObjectKind::IMG => true,
            _               => false,
        });

        let obj_ptr = object.as_ptr();
        unsafe {
            for i in 0..raw_len {
                assert_eq!(raw[i], *obj_ptr.offset(i as isize));
            }
        }
    }
}
