use pad::*;
use objects::*;
use parsing::{parse_target_size, parse_objects};


/// Do ALPaCA's morphing.
///
/// If the input object is an HTML page, it samples a new page, changes the
/// references to its objects accordingly, and pads it; if it is a different
/// type of object, it returns the object padded to the specified size.
#[no_mangle]
pub extern fn morph_object(object: &[u8], request: &str) -> *const u8 {
    let mut object = Object::from(object, request);

    let target_size = if object.kind == ObjectKind::HTML {
        morph_html(&mut object)
    }
    else {
        parse_target_size(request)
    };

    object.pad(target_size);

    object.as_ptr()
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
fn morph_html(html: &mut Object) -> usize {
    unimplemented!();
}
