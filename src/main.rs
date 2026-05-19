use crate::init::init;
use crate::parsing::get_filter;
use clap::Parser;
use parsing::load_file_as_sections;
use std::fs::remove_file;
use std::io::stdout;
use std::io::Write;
use std::process::exit;
use walkdir::WalkDir;

mod init;
mod parsing;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
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

    #[clap(long = "init", action, required = false, default_value_t = false)]
    /// Prompts to init cleanup in the working directory
    init: bool,
}

fn main() {
    let args = Args::parse();

    if args.init {
        init();
        exit(0);
    }

    let mut seen = vec![];

    if args.what_if {
        println!("Following files would be deleted:");
    }

    if args.verbose {
        writeln!(stdout(), "Deleted files:").expect("Failed to write to stdout");
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

    if (args.verbose || args.what_if) && seen.is_empty() {
        println!("None");
    }
}
