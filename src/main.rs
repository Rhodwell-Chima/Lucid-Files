use Lucid_Files::action::{CopyActionRef, DeleteActionRef, FileAction, MoveActionRef};
use Lucid_Files::config::config::{ActionType, Config};
use Lucid_Files::config::{filter_from_config, load_config_from_path};
use Lucid_Files::filters::FileFilter;
use Lucid_Files::filters::extension::ExtensionFilter;
use Lucid_Files::filters::filter_chain::{AndMultiFilter, OrMultiFilter};
use Lucid_Files::filters::size::SizeFilter;
use Lucid_Files::scanner::RecursiveScanner;
use Lucid_Files::scanner::Scanner;
use clap::ArgAction;
use clap::Parser;
use log::{error, info};
use std::fs;
use std::io::{Write, stdin};
use std::path::{Path, PathBuf};

fn main() {
    env_logger::init();
    let parser = Cli::parse();
    let config_path = if parser.config.exists() {
        parser.config
    } else {
        println!("The path you entered does not exist. Defaulting to `lucid.toml`");
        "lucid.toml".parse().unwrap()
    };
    let config_result = load_config_from_path(&config_path);
    let config = match config_result {
        Ok(config) => {
            info!("Successfully loaded configuration.");
            config
        }
        Err(error) => {
            error!("Failed to load configuration: {}", error);
            info!("Using default configuration.");
            Config::default()
        }
    };
    println!("Configuration Loaded: {:?}", config);
    let source = match &parser.source {
        None => prompt_path("Enter a valid source path: ", true),
        Some(value) => {
            if value.is_dir() {
                value.clone()
            } else {
                prompt_path("Enter a valid source path: ", true)
            }
        }
    };
    let destination = match &parser.destination {
        None => prompt_path("Enter a valid destination path: ", true),
        Some(value) => {
            if value.is_dir() {
                value.clone()
            } else {
                prompt_path("Enter a valid destination path: ", true)
            }
        }
    };

    println!("Choose a filter to scan files:");
    println!("1. Extension Filter (txt, rs)");
    println!("2. Size Filter (0 - 1024 bytes)");
    println!("3. Or Multi Filter (Extension OR Size)");
    println!("4. And Multi Filter (Extension AND Size)");
    println!("5. Use configured filter from `lucid.toml`");
    let filter_choice = prompt_choice("Enter the number corresponding to your choice: ", 1, 5);
    let filter: Box<dyn FileFilter> = if filter_choice == 5 {
        filter_from_config(&config.filters)
    } else {
        choose_filter(filter_choice)
    };

    let scanner = RecursiveScanner::new(filter, 1, 200);
    let results = &scanner.scan(&source).unwrap();

    println!("Choose an action to perform on the scanned files:");
    println!("1. Copy Files");
    println!("2. Move Files");
    println!("3. Delete Files");
    println!("4. Use configured filter from `lucid.toml`");
    let action_choice = prompt_choice("Enter the number corresponding to your choice: ", 1, 4);
    let action = if action_choice == 4 {
        &config.core.action
    } else {
        &match action_choice {
            1 => ActionType::Copy,
            2 => ActionType::Move,
            3 => ActionType::Delete,
            _ => ActionType::Unknown,
        }
    };
    for i in results {
        println!("{}", &i.display());
        perform_configured_action(action, &i, &destination);
    }
}

fn prompt_line(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    let _ = std::io::stdout().flush();
    input.clear();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn prompt_path(prompt: &str, must_exist: bool) -> PathBuf {
    loop {
        let s = prompt_line(prompt);
        if s.is_empty() {
            println!("Input cannot be empty. Please try again.");
            continue;
        }
        let p = Path::new(&s).to_path_buf();
        if must_exist {
            if p.exists() {
                return p;
            } else {
                println!("Path does not exist. Please enter an existing path.");
                continue;
            }
        } else {
            if p.exists() {
                return p;
            } else {
                let mut yn = prompt_line("Destination does not exist. Create it? (y/n): ");
                yn.make_ascii_lowercase();
                if yn == "y" || yn == "yes" {
                    if let Err(e) = fs::create_dir_all(&p) {
                        println!("Failed to create directory: {}. Try again.", e);
                        continue;
                    }
                    return p;
                } else {
                    println!("Please enter a different destination.");
                    continue;
                }
            }
        }
    }
}

fn prompt_choice(prompt: &str, min: u8, max: u8) -> u8 {
    loop {
        let s = prompt_line(prompt);
        match s.parse::<u8>() {
            Ok(n) if n >= min && n <= max => return n,
            _ => {
                println!(
                    "Invalid choice. Enter a number between {} and {}.",
                    min, max
                );
            }
        }
    }
}

fn choose_filter(choice: u8) -> Box<dyn FileFilter> {
    match choice {
        1 => Box::new(ExtensionFilter::new(vec!["txt", "rs"])),
        2 => Box::new(SizeFilter::new(0, 1024)),
        3 => Box::new(OrMultiFilter::new(vec![
            Box::new(ExtensionFilter::new(vec!["txt", "rs"])),
            Box::new(SizeFilter::new(0, 1024)),
        ])),
        4 => Box::new(AndMultiFilter::new(vec![
            Box::new(ExtensionFilter::new(vec!["txt", "rs"])),
            Box::new(SizeFilter::new(0, 1024)),
        ])),
        _ => Box::new(ExtensionFilter::new(vec!["txt", "rs"])),
    }
}

fn perform_configured_action(choice: &ActionType, file: &PathBuf, destination: &Path) {
    match choice {
        ActionType::Copy => {
            if let Err(e) =
                CopyActionRef::new(&file, &destination.join(&file.file_name().unwrap())).execute()
            {
                println!("Copy failed: {}", e);
            } else {
                println!(
                    "Successfully Copied {} to {}",
                    &file.display(),
                    &destination.display()
                )
            }
        }
        ActionType::Move => {
            if let Err(e) =
                MoveActionRef::new(&file, &destination.join(&file.file_name().unwrap())).execute()
            {
                println!("Move failed: {}", e);
            } else {
                println!(
                    "Successfully moved {} to {}",
                    &file.display(),
                    &destination.display()
                )
            }
        }
        ActionType::Delete => {
            if let Err(e) = DeleteActionRef::new(&file).execute() {
                println!("Delete failed: {}", e);
            } else {
                println!("Successfully Deleted {}", &file.display())
            }
        }
        ActionType::Unknown => {
            println!("Invalid choice. No action will be performed.");
        }
    }
}

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
    #[arg(long, action = ArgAction::SetTrue)]
    pub dry_run: bool,

    /// Max recursion depth for scanner
    #[arg(long, value_name = "NUMBER")]
    pub max_depth: Option<usize>,

    /// Assume yes to prompts
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub yes: bool,

    /// Verbosity (-v, -vv, -vvv)
    #[arg(short, action = ArgAction::Count)]
    pub verbose: u8,
}
