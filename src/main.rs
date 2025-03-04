mod scanner;
mod cleanup;
mod score;
mod ui;
mod disk;
mod config;
mod utils;

fn main() {
    println!("DiskGuardian: A Cross-Platform Disk Utility");
    ui::run();
}