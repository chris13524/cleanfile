use {clap::Parser, std::path::PathBuf};

/// A utility to clear out files for you based on a standard file format.
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to cleanfile
    #[arg(short, long)]
    pub file: PathBuf,

    /// Dry run
    #[arg(short, long)]
    pub dry_run: bool,
}
