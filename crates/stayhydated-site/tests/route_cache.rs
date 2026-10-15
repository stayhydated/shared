use std::fs;
use std::io::ErrorKind;

use stayhydated_site::route_cache::{
    cleanup_generated_route_cache, cleanup_generated_route_cache_for_outputs,
    mark_generated_route_cache,
};

const MARKER: &str = ".generated-route-cache";

#[test]
fn invalid_directories_preserve_all_existing_content() {
    let temp = tempfile::tempdir().expect("tempdir");
    let public_dir = temp.path().join("public");
    let outside_dir = temp.path().join("outside");
    fs::create_dir_all(public_dir.join("demos")).expect("create demos");
    fs::create_dir_all(&outside_dir).expect("create outside directory");
    fs::write(outside_dir.join("keep.txt"), "outside").expect("write outside file");
    fs::write(public_dir.join("index.html"), "root").expect("write index");
    fs::write(public_dir.join("404.html"), "fallback").expect("write fallback");
    fs::write(public_dir.join("demos/index.html"), "demo").expect("write demo");
    mark_generated_route_cache(&public_dir, MARKER, "generated").expect("mark cache");

    for invalid in [
        ".",
        "..",
        "../outside",
        "demos/../outside",
        "nested/dir",
        "./demos",
        "demos/",
        r"..\outside",
        outside_dir.to_str().expect("UTF-8 temporary path"),
    ] {
        let error =
            cleanup_generated_route_cache(&public_dir, MARKER, ["demos", invalid], |_, _| true)
                .expect_err("invalid directory must be rejected");

        assert_eq!(error.kind(), ErrorKind::InvalidInput, "input: {invalid:?}");
        for (path, expected) in [
            (public_dir.join("index.html"), "root"),
            (public_dir.join("404.html"), "fallback"),
            (public_dir.join("demos/index.html"), "demo"),
            (outside_dir.join("keep.txt"), "outside"),
        ] {
            assert_eq!(
                fs::read_to_string(path).expect("read preserved file"),
                expected
            );
        }
    }
}

#[test]
fn cleanup_preserves_unmarked_outputs() {
    let temp = tempfile::tempdir().expect("tempdir");
    let public_dir = temp.path();
    fs::create_dir(public_dir.join("demos")).expect("create demos");
    fs::write(public_dir.join("index.html"), "root").expect("write index");
    fs::write(public_dir.join("demos/index.html"), "demo").expect("write demo");

    cleanup_generated_route_cache_for_outputs(public_dir, MARKER, ["demos/page"], |_, _| true)
        .expect("unmarked cleanup is a no-op");

    assert_eq!(
        fs::read_to_string(public_dir.join("index.html")).unwrap(),
        "root"
    );
    assert_eq!(
        fs::read_to_string(public_dir.join("demos/index.html")).unwrap(),
        "demo"
    );
}

#[test]
fn output_cleanup_rejects_parent_prefixes() {
    let temp = tempfile::tempdir().expect("tempdir");
    let public_dir = temp.path().join("public");
    mark_generated_route_cache(&public_dir, MARKER, "generated").expect("mark cache");
    fs::write(public_dir.join("index.html"), "root").expect("write index");

    let error = cleanup_generated_route_cache_for_outputs(
        &public_dir,
        MARKER,
        ["../outside/page"],
        |_, _| false,
    )
    .expect_err("parent prefix must be rejected");

    assert_eq!(error.kind(), ErrorKind::InvalidInput);
    assert_eq!(
        fs::read_to_string(public_dir.join("index.html")).unwrap(),
        "root"
    );
}

#[cfg(unix)]
#[test]
fn cleanup_does_not_follow_directory_symlinks() {
    let temp = tempfile::tempdir().expect("tempdir");
    let public_dir = temp.path().join("public");
    let outside_dir = temp.path().join("outside");
    fs::create_dir(&outside_dir).expect("create outside directory");
    fs::write(outside_dir.join("keep.txt"), "outside").expect("write outside file");
    mark_generated_route_cache(&public_dir, MARKER, "generated").expect("mark cache");
    std::os::unix::fs::symlink(&outside_dir, public_dir.join("demos")).expect("create symlink");

    cleanup_generated_route_cache(&public_dir, MARKER, ["demos"], |_, _| false)
        .expect("remove generated symlink");

    assert!(!public_dir.join("demos").exists());
    assert_eq!(
        fs::read_to_string(outside_dir.join("keep.txt")).unwrap(),
        "outside"
    );
}
