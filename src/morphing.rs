use types::*;
use pad::pad_object;

struct Object {
    size: usize,
    path: &'static str,
    position: usize,
}

/// Do ALPaCA's morphing.
///
/// If the input object is an HTML page, it samples a new page, changes the
/// references to its objects accordingly, and pads it; if it is a different
/// type of object, it returns the object padded to the specified size.
#[no_mangle]
pub extern fn morph_object(object: &[u8], request: &str) -> *const u8 {
    let object_type = parse_object_type(object, request);

    if object_type == HTML_T {
        morph_html(object)
    }
    else {
        let target_size = parse_target_size(request);
        pad_object(object, object_type, target_size)
    }.as_ptr()
}

fn parse_target_size(request: &str) -> usize {
    unimplemented!();
}

/// Samples a new page's characteristics from a distribution,
/// and morphs it accordingly.
///
/// This function:
/// 1. samples new page and objects' sizes from a distribution
/// 2. appends the desired size to the objects' references in the HTML
/// 3. pads the HTML page to the chosen size.
///
/// # Arguments
///
/// `html` - HTML page.
fn morph_html(html: &[u8]) -> Vec<u8> {
    unimplemented!();
}

fn parse_objects(html: &str) -> Vec<Object> {
    unimplemented!();
}
