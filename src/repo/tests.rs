use crate::{Repo, Url};
use std::path::{Path, PathBuf};
use test_case::test_case;

#[test_case(
    "https://example.com/foo/bar.pdf",
    "BASE/https/example.com/foo/bar.pdf"
)]
#[test_case(
    "https://example.com:12345/foo/bar.pdf",
    "BASE/https_12345/example.com/foo/bar.pdf"
)]
#[test_case("file:///foo/bar.pdf", "BASE/file/foo/bar.pdf")]
#[test_case("data:text/plain,Stuff", "FIXME")]
fn url_path(input: &str, expected: &str) {
    let repodir: PathBuf = "BASE".parse().unwrap();
    let url: Url = input.parse().unwrap();
    let actual = Repo::from(repodir).get_url_path(&url);
    let expected = Path::new(expected);
    assert_eq!(expected, actual);
}
