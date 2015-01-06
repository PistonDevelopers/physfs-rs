#![feature(globs)]

extern crate physfs;

use physfs::*;

mod directory;

//from project_root
static PATH_TO_HERE : &'static str = "tests/";

#[test]
fn test_create_physfs_context() {
    let con = PhysFSContext::new().unwrap();
    assert!(PhysFSContext::is_init());
}

#[test]
fn test_threaded_physfs_contexts() {
    for _ in range(0i, 10) {
        spawn(proc() {
            let con = PhysFSContext::new().unwrap();
            assert!(PhysFSContext::is_init())
        });
    }
}