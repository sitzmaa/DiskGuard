# DiskGuard

A cross-platform disk utility tool for analyzing file organization, detecting bloat, and cleaning up disk space.

## Features
- **Disk Health Analysis**: Check disk space usage, file distribution, and SMART status.
- **Bloat Scanner**: Identify large, old, or unnecessary files.
- **File Organization Score**: Get a score based on file structure depth, age, and usage.
- **Cleanup Utility**: Select files to delete and free up space safely.
- **Cross-Platform**: Works on macOS, Windows, and Linux.

## Installation
### Prerequisites
- Install Rust: [https://rust-lang.org](https://www.rust-lang.org/tools/install)

### Build & Run
```sh
git clone https://github.com/sitzmaa/DiskGuard.git
cd DiskGuardian
cargo build --release
./target/release/diskguard
```

## Roadmap

### Phase 1: Initial Setup

🔲 Setup Rust environment and dependencies

🔲 Define the project structure

🔲 Implement directory scanning and metadata analysis

### Phase 2: Core Features

🔲 Implement disk usage analysis

🔲 Implement bloat file detection

🔲 Implement file organization scoring system

🔲 Implement basic CLI commands

### Phase 3: Cleanup & Optimization

🔲 Implement safe file deletion with logging

🔲 Implement user-defined cleanup rules

🔲 Optimize disk scanning for performance

### Phase 4: UI & Cross-Platform Packaging

🔲 Implement CLI arguments & interactive prompts

🔲 (Optional) Build a GUI for an interactive experience

🔲 Create cross-platform installers

### Phase 5: Final Testing & Release

🔲 Test on macOS, Windows, Ubuntu, and openSUSE

🔲 Package the app for distribution
