use physfs::PhysFSContext;
use physfs::file;
use super::TEST_LOCK;
use std::io::Read;

#[test]
fn read_file_from_directory() {
    let _g = TEST_LOCK.lock();
    let con = match PhysFSContext::new() {
        Err(e) => panic!(e),
        Ok(con) => con
    };

    assert!(PhysFSContext::is_init());

    match con.mount(super::PATH_TO_HERE.to_string(), "/test/".to_string(), true) {
        Err(e) => panic!(e),
        _ => {}
    }

    let mut file = match file::File::open(&con, "/test/directory/read.txt".to_string(), file::Mode::Read) {
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
}

