mod args;
mod prompts;

pub use args::Cli;

use Lucid_Files::config::config::{ActionType, Config};
use Lucid_Files::config::{filter_from_config, load_config_from_path};
use Lucid_Files::filters::FileFilter;
use Lucid_Files::scanner::{RecursiveScanner, Scanner};
use Lucid_Files::util::scanner_utils::perform_scanning;
use Lucid_Files::util::{action_utils, filter_utils};
use clap::Parser;
use log::{error, info};

pub fn run() {
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
        None => prompts::prompt_path("Enter a valid source path: ", true),
        Some(value) => {
            if value.is_dir() {
                value.clone()
            } else {
                prompts::prompt_path("Enter a valid source path: ", true)
            }
        }
    };

    let destination = match &parser.destination {
        None => prompts::prompt_path("Enter a valid destination path: ", true),
        Some(value) => {
            if value.is_dir() {
                value.clone()
            } else {
                prompts::prompt_path("Enter a valid destination path: ", true)
            }
        }
    };

    println!("Choose a filter to scan files:");
    println!("1. Extension Filter (txt, rs)");
    println!("2. Size Filter (0 - 1024 bytes)");
    println!("3. Or Multi Filter (Extension OR Size)");
    println!("4. And Multi Filter (Extension AND Size)");
    println!("5. Use configured filter from `lucid.toml`");

    let filter_choice =
        prompts::prompt_choice("Enter the number corresponding to your choice: ", 1, 5);
    let filter: Box<dyn FileFilter> = if filter_choice == 5 {
        filter_from_config(&config.filters)
    } else {
        filter_utils::choose_filter(filter_choice)
    };

    // let scanner = RecursiveScanner::new(filter, 1, 200);
    let results = perform_scanning(&config.core.scanner, &source, filter).unwrap();

    println!("Choose an action to perform on the scanned files:");
    println!("1. Copy Files");
    println!("2. Move Files");
    println!("3. Delete Files");
    println!("4. Use configured filter from `lucid.toml`");

    let action_choice =
        prompts::prompt_choice("Enter the number corresponding to your choice: ", 1, 4);
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
    let dry_run: &bool = &config.general.dry_run;
    for i in results {
        // println!("{}", &i.display());
        action_utils::perform_action(action, &i, &destination, dry_run.clone());
    }
}
