use super::file;
use crate::error;
use crate::model;
use libc::{c_void, sysconf, _SC_PAGESIZE};
use std::error::Error;
use std::ptr;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::{fs, os::unix::prelude::MetadataExt};

pub fn get_page_size() -> usize {
    let page_size = unsafe { sysconf(_SC_PAGESIZE) as usize };
    page_size
}

pub fn get_file_page_stat(path: &str) -> Result<model::PcStatus, Box<dyn Error>> {
    let file = fs::OpenOptions::new().read(true).write(true).open(path)?;
    let fd = file::get_fd(&file);

    let addr: *const u8 = ptr::null();
    let mtime = file.metadata()?.mtime() as u64;
    let size = file.metadata()?.size() as usize;

    // See more info at https://man7.org/linux/man-pages/man2/mmap.2.html
    // PROT_NONE: Pages may not be accessed.
    // MAP_SHARED: Share this mapping
    let mmap_ptr = unsafe {
        libc::mmap(
            addr as *mut c_void,
            size as libc::size_t,
            libc::PROT_NONE,
            libc::MAP_SHARED,
            fd,
            0,
        )
    };

    if mmap_ptr == libc::MAP_FAILED {
        return Err(Box::new(error::MapError::new()));
    }

    let page_size = get_page_size();
    let pages = (size + page_size - 1) / page_size;
    let mut results = vec![0u8; pages];
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let ret = unsafe { libc::mincore(mmap_ptr, size, results.as_mut_ptr()) };
    if ret != 0 {
        return Err(Box::new(error::MincoreError::new()));
    }

    let cached: usize = results.iter().map(|x| (x & 0b1) as usize).sum();
    let percent = (cached as f64 / pages as f64) * 100_f64;
    let uncached = pages - cached;

    // let per_page_cache_stat = results
    //     .iter()
    //     .map(|x| if (x & 0b1) == 1 { true } else { false })
    //     .collect();

    // unmap
    unsafe { libc::munmap(mmap_ptr, size) };

    Ok(model::PcStatus::new(
        &path, size, pages, cached, uncached, percent, timestamp,
        mtime,
        // per_page_cache_stat,
    ))
}
