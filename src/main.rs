use Lucid_Files::action::{CopyActionRef, DeleteActionRef, FileAction, MoveActionRef};
use Lucid_Files::config::config::{ActionType, Config};
use Lucid_Files::config::load_config_from_path;
use Lucid_Files::filters::FileFilter;
use Lucid_Files::filters::extension::ExtensionFilter;
use Lucid_Files::filters::filter_chain::{AndMultiFilter, OrMultiFilter};
use Lucid_Files::filters::size::SizeFilter;
use Lucid_Files::scanner::RecursiveScanner;
use Lucid_Files::scanner::Scanner;
use log::{error, info};
use std::fs;
use std::io::{Write, stdin};
use std::path::{Path, PathBuf};

fn main() {
    env_logger::init();
    let config_result = load_config_from_path("lucid.toml".as_ref());
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
    let source = prompt_path("Enter a valid source path: ", true);
    let destination = prompt_path("Enter a valid destination path: ", true);

    println!("Choose a filter to scan files:");
    println!("1. Extension Filter (txt, rs)");
    println!("2. Size Filter (0 - 1024 bytes)");
    println!("3. Or Multi Filter (Extension OR Size)");
    println!("4. And Multi Filter (Extension AND Size)");
    let filter_choice = prompt_choice("Enter the number corresponding to your choice: ", 1, 4);
    let filter: Box<dyn FileFilter> = choose_filter(filter_choice);

    let scanner = RecursiveScanner::new(filter, 1, 20);
    let results = &scanner.scan(&source).unwrap();

    println!("Choose an action to perform on the scanned files:");
    println!("1. Copy Files");
    println!("2. Move Files");
    println!("3. Delete Files");
    let action_choice = prompt_choice("Enter the number corresponding to your choice: ", 1, 3);

    for i in results {
        println!("{}", &i.display());
        perform_configured_action(&config.core.action, &i, &destination);
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

fn choose_action(choice: u8, file: &PathBuf, destination: &Path) {
    match choice {
        1 => {
            if let Err(e) =
                CopyActionRef::new(&file, &destination.join(&file.file_name().unwrap())).execute()
            {
                println!("Copy failed: {}", e);
            }
        }
        2 => {
            if let Err(e) =
                MoveActionRef::new(&file, &destination.join(&file.file_name().unwrap())).execute()
            {
                println!("Move failed: {}", e);
            }
        }
        3 => {
            if let Err(e) = DeleteActionRef::new(&file).execute() {
                println!("Delete failed: {}", e);
            }
        }
        _ => {
            println!("Invalid choice. No action will be performed.");
        }
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
