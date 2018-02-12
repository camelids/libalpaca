extern crate alpaca;

use std::fs::File;
use std::io::Read;

use alpaca::parsing::parse_html_object_refs;
use alpaca::objects::{Object, ObjectKind, ObjectReference};

// TODO: The SD page was download via
// `wget -p -k http://source.demo.securedrop.club/`. The TP page via
// `torify wget -p -k http://expyuzz4wqqyqhjn.onion/`. We should use tbselenium
// to re-download these pages and also facebookcorewwwi.onion .

#[test]
fn test_parse_securedrop_homepage() {
    let root_path = "tests/fixtures/securedrop/";
    let mut sd_html_content = Vec::new();
    let mut sd_file = File::open(root_path.to_string() + "index.html").unwrap();
    assert!(sd_file.read_to_end(&mut sd_html_content).is_ok());
    let mut sd_html = Object {
        kind: ObjectKind::HTML,
        content: sd_html_content,
        position: None,
        target_size: None,
    };

    // There are 8 images and 1 CSS stylesheet on the SD homepage.
    // TODO: technically there are 9 if you count the favicon. Something to look
    // into collecting data on (separate distribution?).
    let object_refs = parse_html_object_refs(&mut sd_html, root_path);
    assert_eq!(object_refs.len(), 9);

    // Check that the SD logo was found and its attributes correctly identified.
    let sd_logo = ObjectReference {
        kind: ObjectKind::IMG,
        path: String::from("static/i/logo.png"),
        node_index: Some(45),
        size: Some(44_138),
        target_size: None,
    };
    assert!(object_refs.contains(&sd_logo));

    // Check that the SD stylesheet was found.
    let sd_stylesheet = ObjectReference {
        kind: ObjectKind::CSS,
        path: String::from("static/css/source.css"),
        node_index: Some(6),
        size: Some(62_602),
        target_size: None,
    };
    assert!(object_refs.contains(&sd_stylesheet));
}

#[test]
fn test_parse_tor_project_homepage() {
    let root_path = "tests/fixtures/torproject/";
    let mut tp_html_content = Vec::new();
    let mut tp_file = File::open(root_path.to_string() + "index.html").unwrap();
    assert!(tp_file.read_to_end(&mut tp_html_content).is_ok());
    let mut tp_html = Object {
        kind: ObjectKind::HTML,
        content: tp_html_content,
        position: None,
        target_size: None,
    };

    // There are 15 images and 1 CSS stylesheet on the SD homepage.
    let object_refs = parse_html_object_refs(&mut tp_html, root_path);
    assert_eq!(object_refs.len(), 16);

    // Check that the Internet Defense logo was found and its size read correctly
    let tp_logo = ObjectReference {
        kind: ObjectKind::IMG,
        path: String::from("images/InternetDefenseLeague-footer-badge.png"),
        node_index: Some(626),
        size: Some(7_253),
        target_size: None,
    };
    assert!(object_refs.contains(&tp_logo));

    // Check that the master CSS stylesheet was found.
    // TODO: The site actually loads 4 CSS stylesheets! We're going to need to
    // change our algorithm and possibly data collection to deal with the CSS
    // @import rule.
    let tp_stylesheet = ObjectReference {
        kind: ObjectKind::CSS,
        path: String::from("css/master.min.css"),
        node_index: Some(22),
        size: Some(96),
        target_size: None,
    };
    assert!(object_refs.contains(&tp_stylesheet));
}
