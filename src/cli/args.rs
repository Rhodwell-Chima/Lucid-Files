use clap::{ArgAction, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "File organiser")]
pub struct Cli {
    /// Path to a configuration file
    #[arg(short, long, value_name = "PATH", default_value = "lucid.toml")]
    pub config: PathBuf,

    /// Source directory (overrides interactive prompt)
    #[arg(short, long, value_name = "PATH")]
    pub source: Option<PathBuf>,

    /// Destination directory (overrides interactive prompt)
    #[arg(short, long, value_name = "PATH")]
    pub destination: Option<PathBuf>,

    /// Create a destination if it does not exist
    #[arg(long, action = ArgAction::SetTrue)]
    pub create_dest: bool,

    /// Extensions for extension-based filters (e.g. --ext rs --ext txt)
    #[arg(long = "ext", value_name = "EXTENSION")]
    pub ext: Vec<String>,

    /// Dry run: do not modify files
    #[arg(long)]
    pub dry_run: Option<bool>,

    /// Min recursion depth for scanner
    #[arg(long, value_name = "NUMBER")]
    pub min_depth: Option<usize>,

    /// Max recursion depth for scanner
    #[arg(long, value_name = "NUMBER")]
    pub max_depth: Option<usize>,

    /// Assume yes to prompts
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub yes: bool,
}
