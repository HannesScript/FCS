use core::fmt::Error;

// inits (mounts) th SD with exFAT
pub fn sd_init() -> Option<Error> {
    return None;
}

// creates new file
pub fn sd_create_new_file(_path: &str, _content: &str) {
    // ...
}

// appends to file
pub fn sd_append_to_file(_path: &str, _line: &str) {
    // ...
}
