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
pub fn pad_object(raw: &[u8], object_type: ObjectType, target_size: usize) -> Vec<u8> {

    let mut object;

    if object_type != ALPACA_T {
        object = raw.to_vec();
        object.reserve_exact(target_size - raw.len());
    }
    else {
        object = Vec::with_capacity(target_size); // New "padding object"
    }

    match object_type {
        HTML_T => pad_html(object, target_size),
        CSS_T => pad_css(object, target_size),
        _ => pad_binary(object, target_size),
    }
}

fn pad_html(raw: Vec<u8>, target_size: usize) -> Vec<u8> {
    // XXX: better to return new vec in this case?
    // XXX: maybe just append to the end
    unimplemented!();
}

fn pad_css(raw: Vec<u8>, target_size: usize) -> Vec<u8>{
    //Pcg32::new_unseeded()
    //      .gen_ascii_chars();
    unimplemented!();
}

fn pad_binary(mut raw: Vec<u8>, target_size: usize) -> Vec<u8> {

    let pad_len = target_size - raw.len();

    assert!(pad_len >= 0);

    raw.extend(Pcg32::new_unseeded()
                     .gen_iter::<u8>()
                     .take(pad_len));
    raw
}
