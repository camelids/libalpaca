use parsing::parse_object_kind;

#[derive(PartialEq)]
pub enum ObjectKind {
    Alpaca,
    HTML,
    CSS,
    IMG,
    Unknown,
}

pub struct Object {
    pub kind: ObjectKind,
    pub content: Vec<u8>,
    pub position: Option<usize>,
    pub target_size: Option<usize>,
}

impl Object {
    pub fn from(raw: &[u8], request: &str) -> Object {
        Object {
            kind: parse_object_kind(raw, request),
            content: raw.to_vec(),
            position: None,
            target_size: None,
        }
    }

    pub fn as_ptr(self) -> *const u8 {
        self.content.as_ptr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::distributions::{IndependentSample, Range};
    use rand::{Rng, weak_rng};

    fn test_object_from_and_as_ptr_jpg() {
        let mut rng = weak_rng();
        let raw_len = Range::new(0, 50).ind_sample(&mut rng);
        let raw = rng.gen_iter::<u8>().take(raw_len).collect::<Vec<u8>>();
        let mut object = Object::from(&raw, "jpg");
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
