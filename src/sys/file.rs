use std::fs;

#[cfg(unix)]
use std::os::unix::io::AsRawFd;

#[cfg(unix)]
pub fn get_fd(file: &fs::File) -> libc::c_int {
    file.as_raw_fd()
}
