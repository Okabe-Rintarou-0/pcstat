mod error;
mod model;
mod sys;

use std::{collections::HashSet, fs, process};

use argparse::{ArgumentParser, List, Store, StoreTrue};
use model::PcStatus;
use sys::*;
use tabled::{Style, Table};

#[macro_use]
extern crate lazy_static;

struct Options {
    pub pid: isize,
    pub children: bool,
    pub json: bool,
    pub pretty: bool,
    pub sort: String,
    pub ge: f64,
    pub le: f64,
    pub docker: String,
    pub markdown: bool,
    pub files: Vec<String>,
}

fn print_table(stats: &Vec<PcStatus>, markdown: bool) {
    let mut table = Table::new(stats);
    let table = if !markdown {
        table.with(Style::modern()).to_string()
    } else {
        table.with(Style::markdown()).to_string()
    };
    println!("{}", table);
}

fn filter_stats(stats: Vec<PcStatus>, opt: &Options) -> Vec<PcStatus> {
    if opt.le <= 0f64 && opt.ge >= 100.0 {
        return vec![];
    }
    let filtered = stats
        .into_iter()
        .filter(|stat| stat.percent >= opt.ge && stat.percent <= opt.le)
        .collect();
    return filtered;
}

fn parse_args(opt: &mut Options) {
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Get page cache status.");
        ap.refer(&mut opt.pid).add_option(
            &["-p", "--pid"],
            Store,
            "Pid of the process you want to checkout.",
        );
        ap.refer(&mut opt.files).add_option(
            &["-f", "--files"],
            List,
            "Files you want to checkout.",
        );
        ap.refer(&mut opt.children).add_option(
            &["-c", "--children"],
            StoreTrue,
            "With children processes.",
        );
        ap.refer(&mut opt.sort).add_option(
            &["-s", "--sort"],
            Store,
            "Sort results(descending order by default).",
        );
        ap.refer(&mut opt.ge).add_option(
            &["--ge"],
            Store,
            "Cache percentage of files should be greater than ge.",
        );
        ap.refer(&mut opt.le).add_option(
            &["--le"],
            Store,
            "Cache percentage of files should be less than le.",
        );
        ap.refer(&mut opt.docker)
            .add_option(&["--docker"], Store, "Docker container name or id.");
        ap.refer(&mut opt.markdown)
            .add_option(&["--markdown"], StoreTrue, "Markdown style table.");
        ap.refer(&mut opt.json)
            .add_option(&["--json"], StoreTrue, "Print in json format.");
        ap.refer(&mut opt.pretty)
            .add_option(&["--pretty"], StoreTrue, "Prettify json format.");
        ap.parse_args_or_exit();
    }
    if opt.le < 0f64 || opt.le > 100.0 || opt.ge < 0f64 || opt.ge > 100.0 || opt.ge > opt.le {
        println!("Invalid range for arguments 'le' or 'ge'!");
        std::process::exit(1);
    }
}

fn sort_stats(stats: &mut Vec<PcStatus>, sort: &str) {
    if sort.len() == 0 {
        return;
    }

    let ascending = sort == "asc";
    if ascending {
        stats.sort_by(|x, y| x.percent.partial_cmp(&y.percent).unwrap());
    } else {
        stats.sort_by(|x, y| y.percent.partial_cmp(&x.percent).unwrap());
    }
}

async fn get_docker_info(
    container_id: &str,
    container_pid: &mut usize,
    lower_dirs: &mut Vec<String>,
) {
    let info = docker::get_container_info(container_id)
        .await
        .unwrap_or_else(|e| {
            println!("{}", e);
            process::exit(-1);
        });

    let pid = docker::parse_container_pid(&info);
    if pid <= 0 {
        println!("Cannot find specified container's pid!");
        process::exit(-1);
    }
    *container_pid = pid as usize;
    docker::parse_container_lower_dirs(&info, lower_dirs);
}

fn get_children_pids(pids: &mut Vec<usize>) {
    let pid_cnt = pids.len();
    for i in 0..pid_cnt {
        let pid = pids[i];
        match proc::get_all_children_pids(pid) {
            Err(err) => {
                eprintln!("{:?}", err);
                std::process::exit(-1);
            }
            Ok(children_pids) => {
                for pid in children_pids {
                    pids.push(pid);
                }
            }
        }
    }
}

fn get_target_files(pids: &Vec<usize>, files: &Vec<String>) -> Vec<String> {
    let mut file_set = HashSet::new();
    let mut target_files = files.clone();
    for pid in pids {
        let result = sys::proc::get_proc_maps(*pid);
        if result.is_err() {
            println!("Process with given pid {} is not found", pid);
            std::process::exit(-1);
        }

        let result = result.unwrap();
        for file in result {
            if !file_set.contains(&file) {
                file_set.insert(file.to_string());
                target_files.push(file);
            }
        }
    }
    return target_files;
}

fn get_pc_stats(target_files: &mut Vec<String>, lower_dirs: &Vec<String>) -> Vec<PcStatus> {
    let mut stats = vec![];
    for file_path in target_files {
        for lower_dir in lower_dirs.iter() {
            let dir_path = format!("{}{}", lower_dir, file_path);
            if fs::metadata(&dir_path).is_ok() {
                *file_path = dir_path;
                break;
            }
        }
        // println!("{}", file_path);
        let result = pc::get_file_page_stat(file_path);
        if result.is_err() {
            eprintln!(
                "error occurred: {}, reason: {}",
                &file_path,
                result.unwrap_err()
            );
            // skip error files
            continue;
        }
        let stat = result.unwrap();
        stats.push(stat);
    }
    return stats;
}

fn stats_to_json(stats: &Vec<PcStatus>, pretty: bool) -> String {
    if !pretty {
        return serde_json::to_string(stats).unwrap();
    } else {
        return serde_json::to_string_pretty(stats).unwrap();
    }
}

#[tokio::main]
async fn main() {
    let mut opt = Options {
        pid: -1,
        children: false,
        json: false,
        pretty: false,
        sort: String::new(),
        ge: 0.0,
        le: 100.0,
        docker: String::new(),
        markdown: false,
        files: vec![],
    };
    parse_args(&mut opt);

    let mut pids = vec![];
    let mut container_lower_dirs = vec![];
    let use_docker = opt.docker.len() > 0;
    if use_docker {
        let mut pid: usize = 0;
        get_docker_info(&opt.docker, &mut pid, &mut container_lower_dirs).await;
        pids.push(pid as usize);
    } else if opt.pid >= 0 {
        pids.push(opt.pid as usize);
    }

    if opt.children {
        get_children_pids(&mut pids)
    }

    let mut target_files = get_target_files(&pids, &opt.files);

    let mut stats = get_pc_stats(&mut target_files, &container_lower_dirs);
    stats = filter_stats(stats, &opt);

    sort_stats(&mut stats, &opt.sort);

    if !opt.json {
        print_table(&stats, opt.markdown)
    } else {
        let json = stats_to_json(&stats, opt.pretty);
        println!("{}", &json);
    }
}
