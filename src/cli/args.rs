use clap::{ArgAction, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "File organiser")]
pub struct Cli {
    #[arg(short, long, value_name = "PATH", default_value = "lucid.toml")]
    pub config: PathBuf,

    #[arg(short, long, value_name = "PATH")]
    pub source: Option<PathBuf>,

    #[arg(short, long, value_name = "PATH")]
    pub destination: Option<PathBuf>,

    #[arg(long, action = ArgAction::SetTrue)]
    pub create_dest: bool,

    #[arg(long = "ext", value_name = "EXTENSION")]
    pub ext: Vec<String>,

    #[arg(long, action = ArgAction::SetTrue)]
    pub dry_run: bool,

    #[arg(long, value_name = "NUMBER")]
    pub max_depth: Option<usize>,

    #[arg(short, long, action = ArgAction::SetTrue)]
    pub yes: bool,

    #[arg(short, action = ArgAction::Count)]
    pub verbose: u8,
}