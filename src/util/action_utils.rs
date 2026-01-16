use crate::action::{CopyActionRef, DeleteActionRef, FileAction, MoveActionRef};
use crate::config::config::ActionType;
use std::path::{Path, PathBuf};

pub fn perform_action(choice: &ActionType, file: &PathBuf, destination: &Path, dry_run: bool) {
    if dry_run {
        dry_run_configured_action(choice, file, destination);
    } else {
        perform_configured_action(choice, file, destination);
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

fn dry_run_configured_action(choice: &ActionType, file: &PathBuf, destination: &Path) {
    match choice {
        ActionType::Copy => {
            println!(
                "[DRY RUN] Would copy {} to {}",
                file.display(),
                destination.join(file.file_name().unwrap()).display()
            );
        }
        ActionType::Move => {
            println!(
                "[DRY RUN] Would move {} to {}",
                file.display(),
                destination.join(file.file_name().unwrap()).display()
            );
        }
        ActionType::Delete => {
            println!("[DRY RUN] Would delete {}", file.display());
        }
        ActionType::Unknown => {
            println!("[DRY RUN] Invalid choice. No action would be performed.");
        }
    }
}
