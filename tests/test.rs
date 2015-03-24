#![feature(std_misc)]

extern crate physfs;

use std::thread;
use physfs::*;
use std::sync::{ StaticMutex, MUTEX_INIT };

mod directory;

/// For running only one test at a time
static TEST_LOCK: StaticMutex = MUTEX_INIT;

// from project_root
const PATH_TO_HERE: &'static str = "tests/";

#[test]
fn test_create_physfs_context() {
    let _g = TEST_LOCK.lock();
    let _c = PhysFSContext::new().unwrap();
    assert!(PhysFSContext::is_init());
}

#[test]
fn test_threaded_physfs_contexts() {
    let _g = TEST_LOCK.lock();
    let threads: Vec<_> = (0 .. 10).map(|_| {
        thread::scoped(|| {
            let _c = PhysFSContext::new().unwrap();
            assert!(PhysFSContext::is_init())
        })
    }).collect();

    for thread in threads {
        thread.join();
    }
}

