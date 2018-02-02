use rand::Rng;
use rand::os::OsRng;

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
    let html_min_size = html.content.len();

    let mut rng = OsRng::new()
                        .expect("Failed to initialize system RNG");

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

    insert_objects_refs(html, &objects);

    // Return the target HTML page size.
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

fn insert_objects_refs(html: &mut Object, objects: &Vec<Object>) -> Result<(), ()> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng, StdRng, SeedableRng};

    fn generate_objects() -> Vec<Object> {
        let object_sizes: Vec<usize> = vec![400, 2000, 1000, 100];

        object_sizes.iter()
                    .map(|s| Object { kind: ObjectKind::Unknown,
                                      content: vec![0u8; *s],
                                      position: None,
                                      target_size: None })
                    .collect()
    }

    fn init_seeded_rng() -> StdRng {
        let s: Vec<usize> = vec![0, 0];

        SeedableRng::from_seed(&s[..])
    }

    #[test]
    fn test_morph_objects_from_distribution() {

        let mut objects = generate_objects();
        let mut rng = init_seeded_rng();

        let min_count = objects.len();
        let obj_min_size = objects.iter()
                                  .map(|o| o.content.len())
                                  .min()
                                  .expect("No objects in this page");

        morph_from_distribution(&mut rng, &mut objects, min_count, obj_min_size);

        let expected_sizes = vec![589, 2273, 5395, 8171, 1128, 1664, 11858];
        let new_sizes = objects.iter()
                               .map(|o| o.target_size.expect("Need Some"))
                               .collect::<Vec<_>>();
        assert!(new_sizes == expected_sizes);
    }
}
