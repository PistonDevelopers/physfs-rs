#![feature(core)]
#![feature(io)]
#![feature(path)]

extern crate physfs;

mod directory;

// from project_root
static PATH_TO_HERE: &'static str = "tests/";

#[test]
fn test_init_physfs() {
    match physfs::init() {
        Err(e) => panic!(e),
        Ok(_) => {}
    };

    assert!(physfs::is_init());
}

