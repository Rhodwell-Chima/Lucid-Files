use crate::filters::name::NameMatch;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub general: General,
    pub categories: Categories,
    pub filters: Filters,
    pub core: CoreTypes,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            general: Default::default(),
            categories: Default::default(),
            filters: Default::default(),
            core: CoreTypes {
                action: ActionType::Move,
                scanner: ScannerType::Simple,
            },
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct General {
    pub dry_run: bool,
    pub confirm_before_action: bool,
    pub overwrite_existing: bool,
    pub remove_empty_dirs: bool,
    pub recursive: bool,
    pub timezone: String,
}

impl Default for General {
    fn default() -> Self {
        General {
            dry_run: false,
            confirm_before_action: false,
            overwrite_existing: false,
            remove_empty_dirs: false,
            recursive: false,
            timezone: "UTC".to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Categories {
    pub by_extension: HashMap<String, Vec<String>>,
    pub by_size: HashMap<String, SizeRange>,
}

impl Categories {
    pub fn new(mut self) -> Self {
        self.normalise_extensions();
        self
    }
    pub fn normalise_extensions(&mut self) {
        (&mut self.by_extension)
            .values_mut()
            .for_each(|mut extensions| {
                (&mut extensions)
                    .iter_mut()
                    .for_each(|extension: &mut String| {
                        if extension.chars().any(|c: char| c.is_uppercase()) {
                            *extension = extension.to_lowercase();
                        }
                    })
            });
    }
}

impl Default for Categories {
    fn default() -> Self {
        Categories {
            by_extension: HashMap::from([
                (
                    "Audio".to_string(),
                    vec!["mp3".to_string(), "ogg".to_string(), "wav".to_string()],
                ),
                (
                    "Video".to_string(),
                    vec!["mp4".to_string(), "mkv".to_string()],
                ),
            ]),
            by_size: HashMap::from([("unknown".to_string(), SizeRange { min: 0, max: 0 })]),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct SizeRange {
    pub min: u64,
    pub max: u64,
}

impl SizeRange {
    fn is_in_range(&self, size: u64) -> bool {
        (size >= self.min) & (size < self.max)
    }
}

#[derive(Deserialize, Debug)]
pub struct Filters {
    pub extensions: Vec<String>,
    pub sizes: SizeRange,
    pub names: Vec<NameMatch>,
}

impl Default for Filters {
    fn default() -> Self {
        Filters {
            extensions: vec!["txt".to_string(), "rs".to_string()],
            sizes: SizeRange { min: 0, max: 1024 },
            names: vec![NameMatch::Contains("log".to_string())],
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CoreTypes {
    pub action: ActionType,
    pub scanner: ScannerType,
}
#[derive(Deserialize, Debug)]
pub enum ActionType {
    Move,
    Copy,
    Delete,
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug)]
pub enum ScannerType {
    Simple,
    Recursive,
    #[serde(other)]
    Unknown,
}
