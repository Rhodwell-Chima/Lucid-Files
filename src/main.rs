use crate::action::move_file::MoveActionRef;
use crate::action::{CopyActionRef, DeleteActionRef, FileAction};
use crate::filters::FileFilter;
use crate::filters::extension::ExtensionFilter;
use crate::filters::filter_chain::{AndMultiFilter, OrMultiFilter};
use crate::filters::size::SizeFilter;
use crate::scanner::{RecursiveScanner, Scanner};
use std::fs;
use std::io::{Write, stdin};
use std::path::{Path, PathBuf};

mod action;
mod classifier;
mod config;
mod filters;
mod scanner;
mod util;

fn main() {
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
        choose_action(action_choice, &i, &destination);
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
