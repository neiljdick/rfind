extern crate walkdir;
extern crate regex;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use walkdir::WalkDir;
use regex::Regex;
use structopt::StructOpt;
use std::env; //get the pwd

#[derive(StructOpt, Debug)]
#[structopt(name = "example", about = "Example of structopt usage.")]
struct Opt {
    #[structopt(short = "f", long="fuzzy", help = "Match partial name")]
    fuzz:bool,

    #[structopt(help ="filename")]
    needle: String,

    // Optional Directory
    #[structopt(help = "Directory to search, default to pwd")]
    directory: Option<String>,
}

fn do_search(re: &Regex, dir: &str)
{
    for entry in WalkDir::new(dir)
        .into_iter().
        filter_map(|e| e.ok()) {
            if re.is_match(entry.file_name().to_str().unwrap()) { 
                println!("{}", entry.path().display());
            }
        }
}

fn make_regex(opt: &Opt) -> Regex {
    match opt.fuzz {
        true => Regex::new(&opt.needle).unwrap(),
        false => Regex::new(&format!("^{}$", opt.needle)).unwrap()
    }
}

fn main() {
    let opt = Opt::from_args();

    let re = make_regex(&opt);

    let dir = match opt.directory {
        Some(x) => x,
        None => env::current_dir().unwrap().to_str().unwrap().to_string()
    };

   do_search(&re, &dir);
}
