#![feature(globs)]

extern crate physfs;

use physfs::*;

mod directory;

//from project_root
static path_to_here : &'static str = "tests/";

#[test]
fn test_create_physfs_context() {
    let con = PhysFSContext::new().unwrap();
    assert!(con.is_init());
}

#[test]
fn test_threaded_physfs_contexts() {
    for _ in range(0i, 10) {
        spawn(proc() {
            let con = PhysFSContext::new().unwrap();
            assert!(con.is_init())
        });
    }
}