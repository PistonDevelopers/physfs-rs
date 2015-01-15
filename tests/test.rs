extern crate physfs;

use std::thread::Thread;
use physfs::*;

mod directory;

//from project_root
static PATH_TO_HERE : &'static str = "tests/";

#[test]
fn test_create_physfs_context() {
    let con = PhysFSContext::new().unwrap();
    let _ = con;
    assert!(PhysFSContext::is_init());
}

#[test]
fn test_threaded_physfs_contexts() {
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
