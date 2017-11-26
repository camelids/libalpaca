use rand::weak_rng;

use types::*;


/// Do ALPaCA's morphing.
///
/// If the input object is an HTML page, it samples a new page, changes the
/// references to its objects accordingly, and pads it; if it is a different
/// type of object, it returns the object padded to the specified size.
#[no_mangle]
pub extern fn morph_object(object: &[u8], request: &str) -> *const u8 {
    let mut object = Object::from(object, request);

    if object.kind == ObjectKind::HTML {
        morph_html(&mut object);
    }
    else {
        let target_size = parse_target_size(request);
        let mut rng = weak_rng();

        object.pad(target_size, &mut rng);
    }

    object.as_ptr()
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
fn morph_html(html: &mut Object) {
    unimplemented!();
}

fn parse_objects(html: &str) -> Vec<Object> {
    unimplemented!();
}
