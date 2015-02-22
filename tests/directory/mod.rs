use std::io::Read;
use std::path::Path;

use physfs;
use physfs::{File, Mode};
use super::TEST_LOCK;

#[test]
fn read_file_from_directory() {
    let _g = TEST_LOCK.lock();

    match physfs::init() {
        Err(e) => panic!(e),
        Ok(_) => {}
    };

    assert!(physfs::is_init());

    match physfs::mount(Path::new(super::PATH_TO_HERE), "/test/".to_string(), true) {
        Err(e) => panic!(e),
        _ => {}
    }

    let mut file = match File::open("/test/directory/read.txt".to_string(), Mode::Read) {
        Ok(f) => f,
        Err(e) => panic!(e)
    };

    let mut bytes = [0u8; 32];
    let buf = bytes.as_mut_slice();

    match file.read(buf) {
        Err(e) => panic!(e),
        _ => {}
    }

    let mut contents = String::new();
    for byte in buf.iter() {
        if *byte == 0 { break }
        contents.push(*byte as char);
    }

    assert!(contents.as_slice() == "Read from me.");

    match file.close() {
        Err(e) => panic!(e),
        Ok(_) => {}
    };

    physfs::deinit();

    assert!(!physfs::is_init());
}

