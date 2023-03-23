mod error;
mod model;
mod sys;

use argparse::{ArgumentParser, List, Store};

use sys::*;
use tabled::{Style, Table};

struct Options {
    pub pid: isize,
    pub files: Vec<String>,
}

fn main() {
    let mut opt = Options {
        pid: -1,
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
        ap.parse_args_or_exit();
    }

    if opt.pid >= 0 {
        let result = sys::proc::get_proc_maps(opt.pid as usize);
        if result.is_err() {
            println!("Process with given pid {} is not found", opt.pid);
            std::process::exit(-1);
        }

        let result = result.unwrap();

        for file in result {
            opt.files.push(file);
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

    let table = Table::new(stats).with(Style::modern()).to_string();
    println!("{}", table);
}
