mod docker;
mod error;
mod model;
mod sys;

use std::collections::HashSet;

use argparse::{ArgumentParser, List, Store, StoreTrue};
use sys::*;
use tabled::{Style, Table};

struct Options {
    pub pid: isize,
    pub children: bool,
    pub sort: String,
    pub ge: f64,
    pub le: f64,
    pub docker: String,
    pub markdown: bool,
    pub files: Vec<String>,
}

fn main() {
    let mut opt = Options {
        pid: -1,
        children: false,
        sort: String::new(),
        ge: 0.0,
        le: 100.0,
        docker: String::new(),
        markdown: false,
        files: vec![],
    };
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
        ap.refer(&mut opt.docker).add_option(
            &["--docker"],
            Store,
            "Docker container name or id.",
        );
        ap.refer(&mut opt.markdown).add_option(
            &["--markdown"],
            StoreTrue,
            "Markdown style table.",
        );
        ap.parse_args_or_exit();
    }

    if opt.le < 0f64 || opt.le > 100.0 || opt.ge < 0f64 || opt.ge > 100.0 || opt.ge > opt.le {
        println!("Invalid range for arguments 'le' or 'ge'!");
        std::process::exit(1);
    }

    let do_filter = opt.le > 0f64 || opt.ge < 100.0;

    let mut pids = vec![];

    if opt.pid >= 0 {
        pids.push(opt.pid as usize);
    }

    if opt.docker.len() > 0 {
        if let Ok(pid) = docker::get_container_pid(&opt.docker) {
            pids.push(pid);
        }
    }

    if opt.children {
        match proc::get_all_children_pids(opt.pid as usize) {
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

    let mut file_set = HashSet::new();
    for pid in pids {
        let result = sys::proc::get_proc_maps(pid);
        if result.is_err() {
            println!("Process with given pid {} is not found", pid);
            std::process::exit(-1);
        }

        let result = result.unwrap();

        for file in result {
            if !file_set.contains(&file) {
                file_set.insert(file.to_string());
                opt.files.push(file);
            }
        }
    }

    let mut stats = vec![];

    for file_path in opt.files {
        let result = pc::get_file_page_stat(&file_path);
        if result.is_err() {
            // skip error files
            continue;
        }
        let stat = result.unwrap();
        stats.push(stat);
    }

    if do_filter {
        stats = stats
            .into_iter()
            .filter(|stat| stat.percent >= opt.ge && stat.percent <= opt.le)
            .collect();
    }

    if opt.sort.len() > 0 {
        if opt.sort == "asc" {
            stats.sort_by(|x, y| x.percent.partial_cmp(&y.percent).unwrap());
        } else {
            stats.sort_by(|x, y| y.percent.partial_cmp(&x.percent).unwrap());
        }
    }
    let mut table = Table::new(stats);
    let table = if !opt.markdown {
        table.with(Style::modern()).to_string()
    } else {
        table.with(Style::markdown()).to_string()
    };
    println!("{}", table);
}
