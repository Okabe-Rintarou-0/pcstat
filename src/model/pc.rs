use tabled::Tabled;

#[derive(Debug, Tabled)]
pub struct PcStatus {
    pub path: String,
    pub size: usize,
    pub pages: usize,
    pub cached: usize,
    pub uncached: usize,
    pub percent: f64,
    pub timestamp: u64,
    pub mtime: u64,
    // pub per_page_cache_stat: Vec<bool>,
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
        // per_page_cache_stat: Vec<bool>,
    ) -> Self {
        Self {
            path: path.to_string(),
            size: size,
            pages: pages,
            cached: cached,
            uncached: uncached,
            percent: percent,
            timestamp: timestamp,
            mtime: mtime,
            // per_page_cache_stat: per_page_cache_stat,
        }
    }
}
