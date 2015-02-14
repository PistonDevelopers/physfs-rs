#![feature(core)]
#![feature(io)]
#![feature(std_misc)]

extern crate physfs;

use std::thread::Thread;
use physfs::*;
use std::sync::{StaticMutex, MUTEX_INIT};

mod directory;

/// For running only one test at a time
static TEST_LOCK: StaticMutex = MUTEX_INIT;

// from project_root
static PATH_TO_HERE: &'static str = "tests/";

#[test]
fn test_create_physfs_context() {
    let _g = TEST_LOCK.lock();
    let con = PhysFSContext::new().unwrap();
    let _ = con;
    assert!(PhysFSContext::is_init());
}

#[test]
fn test_threaded_physfs_contexts() {
    let _g = TEST_LOCK.lock();
    let threads: Vec<_> = range(0is, 10).map(|_| {
        Thread::scoped(move || {
            let con = PhysFSContext::new().unwrap();
            let _ = con;
            assert!(PhysFSContext::is_init())
        })
    }).collect();

    for thread in threads.into_iter() {
        let _ = thread.join();
    }
}

