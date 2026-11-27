//! Golden tests: the Rust generators must reproduce the original Python
//! tool's payload bytes exactly. Goldens were captured from lure 1.x
//! (see docs/ANALYSIS.md — "Behavior parity").

use hash_bait::payloads::{render, render_all, Kind};

const GOLDEN_URL: &str = include_str!("golden/@lure.url");
const GOLDEN_SCF: &str = include_str!("golden/@lure.scf");
const GOLDEN_XML: &str = include_str!("golden/@lure.xml");
const GOLDEN_DOCS_URL: &str = include_str!("golden/@docs.url");
const GOLDEN_PLAIN_SCF: &str = include_str!("golden/plain.scf");

#[test]
fn url_matches_original() {
    let (name, body) = render(Kind::Url, "10.10.14.3", "@lure");
    assert_eq!(name, "@lure.url");
    assert_eq!(body, GOLDEN_URL);
}

#[test]
fn scf_matches_original() {
    let (name, body) = render(Kind::Scf, "10.10.14.3", "@lure");
    assert_eq!(name, "@lure.scf");
    assert_eq!(body, GOLDEN_SCF);
}

#[test]
fn xml_matches_original() {
    let (name, body) = render(Kind::Xml, "10.10.14.3", "@lure");
    assert_eq!(name, "@lure.xml");
    assert_eq!(body, GOLDEN_XML);
}

#[test]
fn custom_basename_matches_original() {
    let (_, body) = render(Kind::Url, "192.168.45.200", "@docs");
    assert_eq!(body, GOLDEN_DOCS_URL);
    let (_, body) = render(Kind::Scf, "1.2.3.4", "plain");
    assert_eq!(body, GOLDEN_PLAIN_SCF);
}

#[test]
fn render_all_keeps_canonical_order() {
    let files = render_all(&Kind::ALL, "10.10.14.3", "@bait");
    let names: Vec<&str> = files.iter().map(|(n, _)| n.as_str()).collect();
    assert_eq!(names, ["@bait.url", "@bait.scf", "@bait.xml"]);
}
