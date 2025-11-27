use crate::action::{CopyActionRef, FileAction};
use crate::classifier::FileClassifier;
use crate::filters::extension::ExtensionFilter;
use crate::filters::filter_chain::OrMultiFilter;
use crate::filters::size::SizeFilter;
use crate::scanner::{RecursiveScanner, Scanner};
use std::io::stdin;
use std::ops::Deref;
use std::path::Path;

mod action;
mod classifier;
mod config;
mod filters;
mod scanner;
mod util;

fn main() {
    let mut source = String::new();
    println!("Enter a valid source path: ");
    stdin().read_line(&mut source).unwrap();
    let source = Path::new(source.trim());

    let mut destination = String::new();
    println!("Enter a valid source path: ");
    stdin().read_line(&mut destination).unwrap();
    let destination = Path::new(destination.trim());

    //let extension_filter = ExtensionFilter::new(vec!["txt", "rs"]);
    let or_multi_filter = OrMultiFilter::new(vec![
        Box::new(ExtensionFilter::new(vec!["txt", "rs"])),
        Box::new(SizeFilter::new(0, 1024)),
    ]);
    let scanner = RecursiveScanner::new(Box::new(or_multi_filter), 1, 20);

    let results = &scanner.scan(&source).unwrap();

    for i in results {
        println!("{}", &i.display());
        //CopyActionRef::new(&i, &destination.join(&i.file_name().unwrap()))
        //    .execute()
        //    .unwrap()
    }
}
