//! Contains padding functions for different resource types.
use rand::{weak_rng, Rng};
use rand::distributions::{IndependentSample, Range};
use std::iter::Extend;

use objects::*;

static CSS_COMMENT_START: &'static str = "/*";
const CSS_COMMENT_START_SIZE: usize = 2;
static CSS_COMMENT_END: &'static str = "*/";
const CSS_COMMENT_END_SIZE: usize = 2;
static HTML_COMMENT_START: &'static str = "<!--";
const HTML_COMMENT_START_SIZE: usize = 4;
static HTML_COMMENT_END: &'static str = "-->";
const HTML_COMMENT_END_SIZE: usize = 3;

/// When Paddable is implemented for a data type, we can pad objects of that
/// data type.
pub trait Paddable {
    /// Pads an object up to a given size.
    ///
    /// # Arguments
    ///
    /// * `target_size` - The target size.
    fn pad(&mut self, usize);
}

impl Paddable for Object {
    /// Pads an object up to a given size.
    ///
    /// Padding varies with respect to the object's type.
    /// In HTML and CSS objects, padding is added within a comment.
    /// In other (binary) objects it is done by appending random bytes.
    ///
    /// # Arguments
    ///
    /// * `target_size` - The target size.
    fn pad(&mut self, target_size: usize) {
        // Rust's type system guarantees pad_len will be >=0 because
        // target_size is unsigned. However, Rust panic!s in this case and in
        // the future we should do proper recovery/ error handling.
        let pad_len = target_size - self.content.len();
        let padding = match self.kind {
            ObjectKind::HTML => get_html_padding(pad_len),
            ObjectKind::CSS => get_css_padding(pad_len),
            _ => get_binary_padding(pad_len),
        };
        self.content.extend(padding);
    }
}

fn get_html_padding(pad_len: usize) -> Vec<u8> {
    // During HTML morphing we should ensure the target size is at least 7
    // bytes larger than the real HTML to account for the comment opening
    // and closing syntax.
    let pad_len = pad_len - HTML_COMMENT_START_SIZE - HTML_COMMENT_END_SIZE;
    let mut pad = Vec::from(HTML_COMMENT_START);
    // [46,127) contains only human-readable ascii characters, no
    // whitespace, and omits '-' to ensure the HTML comment cannot be ended
    // early by the random generation of the bytes corresponding to '-->'.
    add_random_chars_in_range(&mut pad, pad_len, 46, 127);
    pad.extend(Vec::from(HTML_COMMENT_END));
    pad
}

fn get_css_padding(pad_len: usize) -> Vec<u8> {
    // During the CSS morphing we should ensure the target size is at least
    // 4 bytes larger than the real CSS.
    let pad_len = pad_len - CSS_COMMENT_START_SIZE - CSS_COMMENT_END_SIZE;
    let mut pad = Vec::from(CSS_COMMENT_START);
    // [43,127) contains only human-readable ascii characters, no
    // whitespace, and omits '*' to ensure the CSS comment cannot be ended
    // early by the random generation of the bytes corresponding to '*/'.
    add_random_chars_in_range(&mut pad, pad_len, 43, 127);
    pad.extend(Vec::from(CSS_COMMENT_END));
    pad
}

/// Extends a Vec<u8> with random values (ASCII characters).
pub fn add_random_chars_in_range(pad: &mut Vec<u8>, pad_len: usize, lb: u8, ub: u8) {
    let acceptable_chars = Range::new(lb, ub);
    let mut rng = weak_rng();
    for _ in 0..pad_len {
        pad.push(acceptable_chars.ind_sample(&mut rng));
    }
}

fn get_binary_padding(pad_len: usize) -> Vec<u8> {
    let mut rng = weak_rng();
    rng.gen_iter::<u8>().take(pad_len).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use rand::distributions::{IndependentSample, Range};

    use std::str;

    #[test]
    fn test_pad_method_html() {
        let mut rng = weak_rng();
        let raw_len = Range::new(0, 50).ind_sample(&mut rng);
        let mut raw = Vec::new();
        add_random_chars_in_range(&mut raw, raw_len, 46, 127);
        let mut object = Object {
            kind: ObjectKind::HTML,
            content: raw.to_vec(),
            position: None,
            target_size: None,
        };
        assert_eq!(object.content.len(), raw_len);
        assert!(match object.kind {
            ObjectKind::HTML => true,
            _ => false,
        });

        let comment_syntax_size = HTML_COMMENT_START_SIZE + HTML_COMMENT_END_SIZE;
        let pad_len = Range::new(comment_syntax_size, 50).ind_sample(&mut rng);
        let target_size = raw_len + pad_len;
        object.pad(target_size);
        assert_eq!(object.content.len(), target_size);
        _test_html_padding(object.content[raw_len..].to_vec());
        // The original object has not changed.
        assert_eq!(object.content[..raw_len], raw[..])
    }

    fn _test_html_padding(padding: Vec<u8>) {
        let mut rng = weak_rng();
        let comment_syntax_size = HTML_COMMENT_START_SIZE + HTML_COMMENT_END_SIZE;
        let padding = if padding.len() == 0 {
            let pad_len = Range::new(comment_syntax_size, 50).ind_sample(&mut rng);
            let padding = get_html_padding(pad_len);
            assert_eq!(padding.len(), pad_len);
            padding
        } else {
            padding
        };
        // The padding starts with '<!--'.
        assert_eq!(
            &padding[..HTML_COMMENT_START_SIZE],
            HTML_COMMENT_START.as_bytes()
        );
        // The padding ends with '-->'.
        assert_eq!(
            &padding[padding.len() - HTML_COMMENT_END_SIZE..],
            HTML_COMMENT_END.as_bytes()
        );
        // Ensure '-->' is not present in the padding.
        let rand_padding = str::from_utf8(
            &padding[HTML_COMMENT_START_SIZE..padding.len() - HTML_COMMENT_END_SIZE],
        ).unwrap();
        assert!(!rand_padding.contains(HTML_COMMENT_END));
    }

    #[test]
    fn test_get_html_padding() {
        let padding = Vec::new();
        _test_html_padding(padding);
    }

    #[should_panic]
    #[test]
    fn test_get_html_padding_too_little() {
        let mut rng = weak_rng();
        let comment_syntax_size = HTML_COMMENT_START_SIZE + HTML_COMMENT_END_SIZE;
        let pad_len = Range::new(0, comment_syntax_size).ind_sample(&mut rng);
        get_html_padding(pad_len);
    }

    #[test]
    fn test_pad_method_css() {
        let mut rng = weak_rng();
        let raw_len = Range::new(0, 50).ind_sample(&mut rng);
        let mut raw = Vec::new();
        add_random_chars_in_range(&mut raw, raw_len, 43, 127);
        let mut object = Object {
            kind: ObjectKind::CSS,
            content: raw.to_vec(),
            position: None,
            target_size: None,
        };
        assert_eq!(object.content.len(), raw_len);
        assert!(match object.kind {
            ObjectKind::CSS => true,
            _ => false,
        });

        let comment_syntax_size = CSS_COMMENT_START_SIZE + CSS_COMMENT_END_SIZE;
        let pad_len = Range::new(comment_syntax_size, 50).ind_sample(&mut rng);
        let target_size = raw_len + pad_len;
        object.pad(target_size);
        assert_eq!(object.content.len(), target_size);
        _test_css_padding(object.content[raw_len..].to_vec());
        // The original object has not changed.
        assert_eq!(object.content[..raw_len], raw[..])
    }

    fn _test_css_padding(padding: Vec<u8>) {
        let mut rng = weak_rng();
        let comment_syntax_size = CSS_COMMENT_START_SIZE + CSS_COMMENT_END_SIZE;
        let padding = if padding.len() == 0 {
            let pad_len = Range::new(comment_syntax_size, 50).ind_sample(&mut rng);
            let padding = get_css_padding(pad_len);
            assert_eq!(padding.len(), pad_len);
            padding
        } else {
            padding
        };
        // The padding starts with '/*'.
        assert_eq!(
            &padding[..CSS_COMMENT_START_SIZE],
            CSS_COMMENT_START.as_bytes()
        );
        // The padding ends with '*/'.
        assert_eq!(
            &padding[padding.len() - CSS_COMMENT_END_SIZE..],
            CSS_COMMENT_END.as_bytes()
        );
        // Ensure '*/' is not present in the padding.
        let rand_padding = str::from_utf8(
            &padding[CSS_COMMENT_START_SIZE..padding.len() - CSS_COMMENT_END_SIZE],
        ).unwrap();
        assert!(!rand_padding.contains(CSS_COMMENT_END));
    }

    #[test]
    fn test_get_css_padding() {
        let padding = Vec::new();
        _test_css_padding(padding);
    }

    #[should_panic]
    #[test]
    fn test_get_css_padding_too_little() {
        let mut rng = weak_rng();
        let comment_syntax_size = CSS_COMMENT_START_SIZE + CSS_COMMENT_END_SIZE;
        let pad_len = Range::new(0, comment_syntax_size).ind_sample(&mut rng);
        get_css_padding(pad_len);
    }

    #[test]
    fn test_pad_method_png() {
        let mut rng = weak_rng();
        let raw_len = Range::new(0, 50).ind_sample(&mut rng);
        let raw = rng.gen_iter::<u8>().take(raw_len).collect::<Vec<u8>>();
        let mut object = Object {
            kind: ObjectKind::IMG,
            content: raw.to_vec(),
            position: None,
            target_size: None,
        };
        assert_eq!(object.content.len(), raw_len);
        assert!(match object.kind {
            ObjectKind::IMG => true,
            _ => false,
        });

        let pad_len = Range::new(0, 50).ind_sample(&mut rng);
        let target_size = raw_len + pad_len;
        object.pad(target_size);
        assert_eq!(object.content.len(), target_size);
        // The original object has not changed.
        assert_eq!(object.content[..raw_len], raw[..])
    }

    #[test]
    fn test_get_binary_padding() {
        let mut rng = weak_rng();
        let pad_len = Range::new(0, 50).ind_sample(&mut rng);
        let padding = get_binary_padding(pad_len);
        assert_eq!(padding.len(), pad_len);
    }

}
