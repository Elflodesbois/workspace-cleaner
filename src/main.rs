use crate::parsing::get_filter;
use clap::Parser;
use parsing::load_file_as_sections;
use std::env;
use std::fs::remove_file;
use walkdir::WalkDir;

mod parsing;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(long = "what-if", action, required = false, default_value_t = false)]
    /// Show the result of the command if it was performed
    what_if: bool,

    #[clap(
        long = "verbose",
        short = 'v',
        required = false,
        default_value_t = false
    )]
    /// Show more information
    verbose: bool,
}

fn main() {
    env::set_current_dir("./test-wk").unwrap();

    let args = Args::parse();
    let mut seen = vec![];

    if args.what_if {
        println!("Following files would be deleted:");
    }

    for (_, val) in load_file_as_sections() {
        let filter = get_filter(&val);

        WalkDir::new(".")
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| filter(e))
            .for_each(|e| {
                let path = e.path().display().to_string();

                if seen.contains(&path) {
                    return;
                }

                if args.what_if || args.verbose {
                    println!("{}", e.path().display());
                }

                seen.push(path);

                if !args.what_if {
                    remove_file(e.path()).unwrap_or_else(|err| {
                        eprintln!("Couldn't delete {}: {}", e.path().display(), err);
                    });
                }
            })
    }
}
