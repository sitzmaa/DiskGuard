use crate::{scanner, cleanup, score};
use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: diskguardian <command> [options]");
        return;
    }
    match args[1].as_str() {
        "scan" => scanner::scan_directory("./"),
        "cleanup" => cleanup::cleanup_files(vec!["/path/to/file".to_string()]),
        "score" => println!("File organization score: {}", score::calculate_score("./")),
        _ => println!("Unknown command"),
    }
}