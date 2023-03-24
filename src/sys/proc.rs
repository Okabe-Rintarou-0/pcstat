use shutil::pipe;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader};

pub fn get_proc_maps(pid: usize) -> Result<Vec<String>, Box<dyn Error>> {
    let mut maps = Vec::new();
    let maps_path = format!("/proc/{}/maps", pid);
    let maps_file = fs::File::open(maps_path)?;

    let mut file_set = HashSet::new();

    let reader = BufReader::new(maps_file);
    // Read the file line by line
    for line in reader.lines() {
        let line = line?.trim().to_string();
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() == 6 && parts[5].starts_with('/') {
            file_set.insert(parts[5].to_string());
        }
    }

    for file in file_set.into_iter() {
        maps.push(file);
    }

    Ok(maps)
}

pub fn get_all_children_pids(pid: usize) -> Result<Vec<usize>, Box<dyn Error>> {
    let mut pids = vec![];
    let output = pipe(vec![
        vec!["ps", "-ef"],
        vec!["awk", &format!("{{if($3==\"{pid}\") print $2}}")],
    ])?;
    let pid_strs: Vec<&str> = output.split('\n').collect();

    for pid_str in pid_strs {
        let r = pid_str.parse();
        if let Ok(pid) = r {
            pids.push(pid);
        }
    }

    Ok(pids)
}
