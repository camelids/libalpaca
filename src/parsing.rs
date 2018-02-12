//! Contains parsing routines
use std::fs;
use std::path::PathBuf;
use std::str;

use select::document::Document;
use select::predicate::{And, Attr, Name, Not, Or, Predicate};
use url::Url;

use objects::{Object, ObjectKind, ObjectReference};

/// Parses the object's kind from its raw representation and
/// the associated request.
///
/// XXX: if we realise that `request` is not needed to determine
/// the object's kind, we can remove it from here and from
/// `objects::Object::from()`.
#[allow(unused)]
pub fn parse_object_kind(raw: &[u8], request: &str) -> ObjectKind {
    unimplemented!();
}

/// Parses the target size of an object from its HTTP request.
#[allow(unused)]
pub fn parse_target_size(request: &str) -> usize {
    unimplemented!();
}

/// Parses the object references contained in an HTML page.
pub fn parse_html_object_refs(html: &mut Object, root_path: &str) -> Vec<ObjectReference> {
    // TODO: replace unwrap() below with better Result handling logic. Can we
    // be sure that html.content is valid UTF-8 bytes before we get here so
    // that `unwrap()` never panics?
    let document = Document::from(str::from_utf8(&html.content[..]).unwrap());
    let mut object_refs = Vec::new();
    // Find all the image object references
    append_object_refs_matching_predicate(
        &mut object_refs,
        &document,
        root_path,
        Name("img"),
        "src",
        ObjectKind::IMG,
    );
    // Find all CSS object references
    append_object_refs_matching_predicate(
        &mut object_refs,
        &document,
        root_path,
        // We're looking for (<link> tags with rel="stylesheet")
        // AND (type="text/css" OR no type attribute). When type is omitted,
        // we assume it is CSS, just as browsers do.
        And(
            And(Name("link"), Attr("rel", "stylesheet")),
            Or(Attr("type", "text/css"), Not(Attr("type", ()))),
        ),
        "href",
        ObjectKind::CSS,
    );
    object_refs
}

/// Appends object references matching a predicate to a Vec<ObjectReference>.
fn append_object_refs_matching_predicate<P: Predicate>(
    object_refs: &mut Vec<ObjectReference>,
    document: &Document,
    root_path: &str,
    predicate: P,
    rel_url_attr: &str,
    kind: ObjectKind,
) {
    for node in document.find(predicate) {
        let rel_url_result = node.attr(rel_url_attr);
        if rel_url_result.is_some() {
            let rel_url = rel_url_result.unwrap();
            let url = get_url(base_url, rel_url);
            if url.is_ok() {
                let path = get_path(root_path, url.path());
                let size = get_size(path);
                if size.is_ok() {
                    object_refs.push(ObjectReference {
                        kind: kind.clone(),
                        url: url.unwrap(),
                        path,
                        node_index: Some(node.index()),
                        size: Some(size.unwrap()),
                        target_size: None,
                    });
                }
            }
        }
    }
}

fn get_path(root_path: &str, rel_path: &str) -> PathBuf {
    PathBuf::from(root_path.to_string() + rel_path)
}

fn get_url(base_url: &str, rel_url: &str) -> Result<Url, ParseError> {
    let base_url = Url::parse(base_url);
    if base_url.is_ok() {
        base_url.join(rel_url)
    } else {
        base_url
    }
}

fn get_size(path: &PathBuf) -> Result<usize, ()> {
    let metadata = fs::metadata(root_path.to_string() + rel_path);
    match metadata {
        Ok(mdata) => Ok(mdata.len() as usize),
        // TODO: more specific error reporting.
        Err(_) => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_size_with_query_string() {}
}
