extern crate walkdir;
extern crate regex;

use walkdir::WalkDir;
use regex::Regex;

fn main() {
    let re = Regex::new(r"Caddies").unwrap();
    for entry in WalkDir::new("/home/neil/Downloads")
        .into_iter().
        filter_map(|e| e.ok()) {
            if re.is_match(entry.file_name().to_str().unwrap()) { 
                println!("{}", entry.path().display());
            }
        }
}
