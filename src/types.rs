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
}

impl Object {
    pub fn from(raw: &[u8], content_type: &str) -> Object {
        let kind = match content_type {
            // Sample values for early development.
            "alpaca" => ObjectKind::Alpaca,
            "html"   => ObjectKind::HTML,
            "css"    => ObjectKind::CSS,
            "png"    => ObjectKind::IMG,
            "jpg"    => ObjectKind::IMG,
            _        => ObjectKind::Unknown,
        };
        Object {
            kind,
            content: raw.to_vec()
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
