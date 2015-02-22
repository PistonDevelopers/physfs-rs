#![feature(core)]
#![feature(io)]
#![feature(path)]
#![feature(std_misc)]

extern crate physfs;

use std::sync::{StaticMutex, MUTEX_INIT};

mod directory;

/// For running only one test at a time
static TEST_LOCK: StaticMutex = MUTEX_INIT;

// from project_root
static PATH_TO_HERE: &'static str = "tests/";

#[test]
fn test_init_physfs() {
    let _g = TEST_LOCK.lock();

    match physfs::init() {
        Err(e) => panic!(e),
        Ok(_) => {}
    };

    assert!(physfs::is_init());

    physfs::deinit();

    assert!(!physfs::is_init());
}

