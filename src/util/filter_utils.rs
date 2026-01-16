use crate::filters::FileFilter;
use crate::filters::extension::ExtensionFilter;
use crate::filters::filter_chain::{AndMultiFilter, OrMultiFilter};
use crate::filters::size::SizeFilter;

pub fn choose_filter(choice: u8) -> Box<dyn FileFilter> {
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