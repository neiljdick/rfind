extern crate walkdir;
extern crate regex;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use walkdir::WalkDir;
use regex::Regex;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rfind", about = "A file finder.")]
struct Opt {
    #[structopt(short = "f", long="fuzzy", help = "Match partial file names")]
    fuzz:bool,

    #[structopt(help ="file name to search for")]
    needle: String,

    // Optional Directory
    #[structopt(help = "Directory to search, defaults to pwd")]
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

    let dir = opt.directory.unwrap_or(".".to_string());

    do_search(&re, &dir);
}
