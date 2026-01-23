mod args;
mod prompts;

pub use args::Cli;
use std::path::PathBuf;

use Lucid_Files::classifier::ExtensionClassifier;
use Lucid_Files::config::config::{ActionType, Config};
use Lucid_Files::config::{filter_from_config, load_config_from_path};
use Lucid_Files::filters::FileFilter;
use Lucid_Files::util::classifier_utils::{
    classified_destination_path, create_directory_with_validation,
};
use Lucid_Files::util::scanner_utils::perform_scanning;
use Lucid_Files::util::{action_utils, filter_utils};
use clap::Parser;
use log::{error, info};
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let parser = Cli::parse();

    let config_path = if parser.config.exists() {
        parser.config
    } else {
        println!("The path you entered does not exist. Defaulting to `lucid.toml`");
        PathBuf::from("lucid.toml")
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

    let source = get_valid_directory(&parser.source, "Enter a valid source path: ");

    let destination = get_valid_directory(&parser.destination, "Enter a valid destination path: ");

    display_filter_menu();

    let filter_choice =
        prompts::prompt_choice("Enter the number corresponding to your choice: ", 1, 5);
    let filter: Box<dyn FileFilter> = if filter_choice == 5 {
        filter_from_config(&config.filters)
    } else {
        filter_utils::choose_filter(filter_choice)
    };

    let results = perform_scanning(&config.core.scanner, &source, filter).map_err(|e| {
        error!("Failed to scan files in {:?}: {}", source, e);
        format!("Scanning failed: {}", e)
    })?;

    display_action_menu();

    let action_choice =
        prompts::prompt_choice("Enter the number corresponding to your choice: ", 1, 4);
    let action = if action_choice == 4 {
        &config.core.action
    } else {
        &match action_choice {
            1 => ActionType::Copy,
            2 => ActionType::Move,
            3 => ActionType::Delete,
            _ => return Err("Invalid action choice".into()),
        }
    };

    let dry_run: &bool = match &parser.dry_run {
        None => &config.general.dry_run,
        Some(value) => value,
    };
    let classifier = ExtensionClassifier::new(config.categories.by_extension.clone());

    for i in results {
        let joined = classified_destination_path(&destination, &classifier, &i);
        create_directory_with_validation(&joined)?;
        action_utils::perform_action(action, &i, joined.as_path(), *dry_run);
    }

    Ok(())
}

fn display_action_menu() {
    println!("Choose an action to perform on the scanned files:");
    println!("1. Copy Files");
    println!("2. Move Files");
    println!("3. Delete Files");
    println!("4. Use configured filter from `lucid.toml`");
}

fn display_filter_menu() {
    println!("Choose a filter to scan files:");
    println!("1. Extension Filter (txt, rs)");
    println!("2. Size Filter (0 - 1024 bytes)");
    println!("3. Or Multi Filter (Extension OR Size)");
    println!("4. And Multi Filter (Extension AND Size)");
    println!("5. Use configured filter from `lucid.toml`");
}

fn get_valid_directory(cli_value: &Option<PathBuf>, prompt_message: &str) -> PathBuf {
    match cli_value {
        Some(path) if path.is_dir() => path.clone(),
        _ => prompts::prompt_path(prompt_message, true),
    }
}
