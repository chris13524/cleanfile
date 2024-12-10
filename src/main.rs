use clap::Parser;

mod args;
mod clean;
mod cleanfile;

fn main() {
    let args = args::Args::parse();

    if args.dry_run {
        println!("Running in dry run mode");
    }

    if !args.file.is_file() {
        panic!("{} is not a file", args.file.display());
    }

    let cleanfile_path = if args.file.is_absolute() {
        args.file.clone()
    } else {
        std::env::current_dir().unwrap().join(&args.file)
    };

    clean::read_and_clean(&cleanfile_path, args);
}
