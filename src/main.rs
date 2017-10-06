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


fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let re = Regex::new(&opt.needle).unwrap();

    match opt.directory {
        Some(x) => println!("Result: {}", x),
        None => println!("directory is {:?}",
                         env::current_dir()
                         .unwrap()
                         .to_str()),
    }

    for entry in WalkDir::new("/home/neil/Downloads")
        .into_iter().
        filter_map(|e| e.ok()) {
            if re.is_match(entry.file_name().to_str().unwrap()) { 
                println!("{}", entry.path().display());
            }
        }
}
