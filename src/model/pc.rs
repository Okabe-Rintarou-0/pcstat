use serde::Serialize;
use tabled::Tabled;

#[derive(Debug, Serialize)]
pub struct Block {
    pub begin: usize,
    pub end: usize,
}

impl Block {
    pub fn new(begin: usize, end: usize) -> Self {
        Self { begin, end }
    }
}

#[derive(Debug, Tabled, Serialize)]
pub struct PcStatus {
    pub path: String,
    pub size: usize,
    pub pages: usize,
    pub cached: usize,
    pub uncached: usize,
    pub percent: f64,
    pub timestamp: u64,
    pub mtime: u64,
    #[tabled(skip)]
    cached_index: Vec<Block>,
}

impl PcStatus {
    pub fn new(
        path: &str,
        size: usize,
        pages: usize,
        cached: usize,
        uncached: usize,
        percent: f64,
        timestamp: u64,
        mtime: u64,
        cached_index: Vec<Block>,
    ) -> Self {
        Self {
            path: path.to_string(),
            size,
            pages,
            cached,
            uncached,
            percent,
            timestamp,
            mtime,
            cached_index,
        }
    }
}
