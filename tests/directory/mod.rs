use physfs::PhysFSContext;
use physfs::file;

#[test]
fn read_file_from_directory() {
    let con = match PhysFSContext::new() {
        Err(msg) => panic!(msg),
        Ok(con) => con
    };

    assert!(PhysFSContext::is_init());

    match con.mount(super::PATH_TO_HERE.to_string(), "/test/".to_string(), true) {
        Err(msg) => panic!(msg),
        _ => {}
    }

    let mut file = match file::File::open(&con, "/test/directory/read.txt".to_string(), file::Mode::Read) {
        Ok(f) => f,
        Err(msg) => panic!(msg)
    };

    let msg = file.read_to_string().unwrap_or_else(|err| panic!(err));

    assert!(msg.as_slice() == "Read from me.");
}
