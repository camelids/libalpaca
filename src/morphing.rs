struct Object {
    size: usize,
    path: &'static str,
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
#[no_mangle]
pub extern fn morph_page(html: &str) -> *const u8 {
    unimplemented!();
}

fn parse_objects(html: &str) -> Vec<Object> {
    unimplemented!();
}
