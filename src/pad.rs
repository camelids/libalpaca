use rand::Rng;
use pcg_rand::Pcg32;

use types::*;

/// Pads an object up to a given size.
///
/// Padding varies with respect to the object's type (we assume the provided
/// extension indicates its correct type).
/// In HTML and CSS objects, padding is added within a comment.
/// In other (binary) objects we limit ourselves to appending random bytes.
///
/// # Arguments
///
/// * `raw` - Object to pad.
/// * `extension` - The object's extension (e.g., "png").
/// * `target_size` - The target size.
#[no_mangle]
pub extern fn pad_object(raw: &[u8], object_type: &str, target_size: usize) -> *const u8 {

    let mut object = raw.to_vec();

    match object_type {
        HTML_T => pad_html(&mut object, target_size),
        CSS_T => pad_css(&mut object, target_size),
        _ => pad_binary(&mut object, target_size),
    };
    
    object.as_ptr()
}

fn pad_html(raw: &mut Vec<u8>, target_size: usize) {
    // XXX: better to return new vec in this case?
    unimplemented!();
}

fn pad_css(raw: &mut Vec<u8>, target_size: usize) {
    //Pcg32::new_unseeded()
    //      .gen_ascii_chars();
    unimplemented!();
}

fn pad_binary(raw: &mut Vec<u8>, target_size: usize) {

    let pad_len = target_size - raw.len();

    assert!(pad_len >= 0);

    raw.extend(Pcg32::new_unseeded()
                     .gen_iter::<u8>()
                     .take(pad_len));
}
