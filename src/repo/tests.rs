use crate::Repo;
use std::path::PathBuf;
use test_case::test_case;

#[test_case(
    "https://example.com/foo/bar.pdf",
    Ok("BASE/https/example.com/foo/bar.pdf")
)]
#[test_case(
    "https://example.com:12345/foo/bar.pdf",
    Ok("BASE/https_12345/example.com/foo/bar.pdf")
)]
#[test_case("file:///foo/bar.pdf", Err("FIXME"))]
#[test_case("data:text/plain,Stuff", Err("FIXME"))]
#[test_case("%\\$ floof", Err("FIXME"))]
fn url_path(input: &str, expected: Result<&str, &str>) {
    let repodir: PathBuf = "BASE".parse().unwrap();
    let actual = input
        .parse()
        .map(|url| Repo::from(repodir).get_url_path(&url).display().to_string())
        .map_err(|e| e.to_string());

    assert_eq!(
        expected,
        actual.as_ref().map(|s| s.as_str()).map_err(|s| s.as_str())
    );
}
