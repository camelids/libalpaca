use rand::{Rng, weak_rng};

use pad::*;
use objects::*;
use parsing::{parse_target_size, parse_objects};
use distribution::{sample_object_count, sample_html_size, sample_object_sizes};

const PAGE_SAMPLE_LIMIT: u8 = 10;


/// Do ALPaCA's morphing.
///
/// If the input object is an HTML page, it samples a new page, changes the
/// references to its objects accordingly, and pads it; if it is a different
/// type of object, it returns the object padded to the specified size.
#[no_mangle]
pub extern fn morph_object(object: &[u8], request: &str) -> *const u8 {
    let mut object = Object::from(object, request);

    let target_size = if object.kind == ObjectKind::HTML {
        morph_html(&mut object).expect("Failed morphing page")
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
fn morph_html(html: &mut Object) -> Result<usize, ()> {
    let mut objects = parse_objects(html);
    objects.sort_unstable_by(|ref o1, ref o2|
                                o1.content.len().cmp(&o2.content.len()));
    // Minimum characteristics.
    let min_count = objects.len();

    let mut rng = weak_rng();

    // Try morphing for PAGE_SAMPLE_LIMIT times.
    let mut success = false;
    for _ in 0..PAGE_SAMPLE_LIMIT {
        if let Ok(_) = morph_from_distribution(&mut rng, &mut objects,
                                               min_count) {
            success = true;
            break;
        }
    }

    if !success {
        return Err(());
    }

    insert_objects_refs(html, &objects)?;

    // Return the target HTML page size.
    let html_min_size = html.content.len();
    sample_html_size(&mut rng, html_min_size)
}

fn morph_from_distribution<R: Rng>(rng: &mut R, objects: &mut Vec<Object>,
        min_count: usize) -> Result<(), ()> {

    // Sample target number of objects (count) and target sizes for morphed
    // objects.
    let target_count = sample_object_count(rng, min_count)?;
    let mut target_sizes = sample_object_sizes(rng, target_count)?;

    // Match target sizes to objects.
    // We will consider each target_size and decide whether to use it to pad
    // an object or to create a new object.
    // NOTE: We append newly created objects to the array objects.
    // NOTE: array objects is initially sorted.
    target_sizes.sort();

    let n = objects.len();              // Keep track of initial number of objects.
    let mut i = 0;                      // Pointing at next object to morph.
    for s in target_sizes {
        if (i < n) && (s >= objects[i].content.len()) {
            // Pad i-th object to size s.
            objects[i].target_size = Some(s);
            i += 1;
        }
        else {
            // Create new padding object.
            let o = Object { kind: ObjectKind::Alpaca,
                             content: Vec::new(),
                             position: None,
                             target_size: Some(s)};
            objects.push(o);
        }
    }

    // No proper padding was found for some object.
    if i < n {
        // Need to remove padding objects.
        objects.truncate(n);

        return Err(());
    }

    Ok(())
}

#[allow(unused)]
fn insert_objects_refs(html: &mut Object, objects: &Vec<Object>) -> Result<(), ()> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{SeedableRng, XorShiftRng};

    fn generate_objects() -> Vec<Object> {
        let object_sizes: Vec<usize> = vec![400, 2000, 1000, 100];

        object_sizes.iter()
                    .map(|s| Object { kind: ObjectKind::Unknown,
                                      content: vec![0u8; *s],
                                      position: None,
                                      target_size: None })
                    .collect()
    }

    fn init_seeded_rng() -> XorShiftRng {
        let s: [u32; 4] = [0, 1, 2, 3];

        XorShiftRng::from_seed(s)
    }

    #[test]
    fn test_morph_objects_from_distribution() {

        let mut objects = generate_objects();
        let mut rng = init_seeded_rng();

        let min_count = objects.len();

        morph_from_distribution(&mut rng, &mut objects, min_count)?;

        let expected_sizes = vec![1048, 2167, 3824, 4230, 1131, 1215, 1529,
            1897, 4260, 5343, 5373, 8315, 8513, 10687, 12617, 12807, 13867,
            14644, 24146];
        let new_sizes = objects.iter()
                               .map(|o| o.target_size.expect("Need Some"))
                               .collect::<Vec<_>>();
        println!("expected sizes: {:?}", new_sizes);
        assert!(new_sizes == expected_sizes);
    }
    // TODO: I migrated the following `test_pad_object_*` tests from the pad
    // module, where once lived the pub extern fn `pad_object`, which was later
    // replaced `morph_object` here. Since testing `morph_object' requires we
    // first implement `parsing::parse_object_kind`, I'm commenting these tests
    // out until we've done that.
    //
    // #[test]
    // fn test_pad_object_html() {
    //     let mut rng = weak_rng();
    //     let raw_len = Range::new(0, 50).ind_sample(&mut rng);
    //     let raw = sample(&mut rng, 46..127, raw_len);
    //     assert_eq!(raw.len(), raw_len);
    //     let comment_syntax_size = HTML_COMMENT_START_SIZE
    //         + HTML_COMMENT_END_SIZE;
    //     let pad_len = Range::new(comment_syntax_size, 50)
    //         .ind_sample(&mut rng);
    //     let target_size = raw_len + pad_len;
    //     let obj_ptr = morph_object(&raw, "html", target_size);
    //     unsafe {
    //         for i in 0..raw_len {
    //             assert_eq!(raw[i], *obj_ptr.offset(i as isize));
    //         }
    //         for i in 0..HTML_COMMENT_START_SIZE {
    //             assert_eq!(HTML_COMMENT_START.as_bytes()[i],
    //             *obj_ptr.offset(raw_len as isize + i as isize));
    //         }
    //         for i in 0..HTML_COMMENT_END_SIZE {
    //             assert_eq!(HTML_COMMENT_END.as_bytes()[i],
    //             *obj_ptr.offset(target_size as isize
    //                             - HTML_COMMENT_END_SIZE as isize
    //                             + i as isize));
    //         }
    //     }
    // }

    // #[test]
    // fn test_pad_object_css() {
    //     let mut rng = weak_rng();
    //     let raw_len = Range::new(0, 50).ind_sample(&mut rng);
    //     let raw = sample(&mut rng, 43..127, raw_len);
    //     assert_eq!(raw.len(), raw_len);
    //     let comment_syntax_size = CSS_COMMENT_START_SIZE
    //         + CSS_COMMENT_END_SIZE;
    //     let pad_len = Range::new(comment_syntax_size, 50)
    //         .ind_sample(&mut rng);
    //     let target_size = raw_len + pad_len;
    //     let obj_ptr = pad_object(&raw, "css", target_size);
    //     unsafe {
    //         for i in 0..raw_len {
    //             assert_eq!(raw[i], *obj_ptr.offset(i as isize));
    //         }
    //         for i in 0..CSS_COMMENT_START_SIZE {
    //             assert_eq!(CSS_COMMENT_START.as_bytes()[i],
    //                        *obj_ptr.offset(raw_len as isize + i as isize));
    //         }
    //         for i in 0..CSS_COMMENT_END_SIZE {
    //             assert_eq!(CSS_COMMENT_END.as_bytes()[i],
    //                        *obj_ptr.offset(target_size as isize
    //                                    - CSS_COMMENT_END_SIZE as isize
    //                                    + i as isize));
    //         }
    //     }
    // }

    // #[test]
    // fn test_pad_object_alpaca() {
    //     let mut rng = weak_rng();
    //     let raw_len = Range::new(0, 50).ind_sample(&mut rng);
    //     let raw = rng.gen_iter::<u8>().take(raw_len).collect::<Vec<u8>>();
    //     assert_eq!(raw.len(), raw_len);
    //     let pad_len = Range::new(0, 50).ind_sample(&mut rng);
    //     let target_size = raw_len + pad_len;
    //     let obj_ptr = pad_object(&raw, "alpaca", target_size);
    //     unsafe {
    //         for i in 0..raw_len {
    //             assert_eq!(raw[i], *obj_ptr.offset(i as isize));
    //         }
    //     }
    // }

    // #[should_panic]
    // #[test]
    // fn test_pad_object_too_small() {
    //     let mut rng = weak_rng();
    //     let raw_len = Range::new(1, 50).ind_sample(&mut rng);
    //     let raw = rng.gen_iter::<u8>().take(raw_len).collect::<Vec<u8>>();
    //     assert_eq!(raw.len(), raw_len);
    //     let obj_ptr = pad_object(&raw, "alpaca", raw_len - 1);
    // }
}
