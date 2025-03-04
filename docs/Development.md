## Overview

DiskGuard is a cross-platform disk utility tool designed to analyze disk health, detect and clean up bloat files, and provide a file organization score based on structural complexity and metadata.

## Project Structure

### 1. main.rs - Entry Point

Initializes the application and parses CLI arguments.

Calls appropriate modules based on user input.

Manages execution flow between scanning, cleanup, scoring, and disk health checks.

### 2. scanner.rs - Directory Scanner

Recursively scans a given directory and gathers metadata.

Uses walkdir to efficiently traverse large file trees.

Collects information such as:

File size

Modification time

File type (regular, directory, symlink, etc.)

Depth within the directory tree

### 3. cleanup.rs - Cleanup Module

Identifies large, outdated, or unnecessary files.

Provides safe deletion options with confirmation prompts.

Implements user-defined rules for automatic cleanup.

Stores logs of deleted files for review and rollback.

### 4. score.rs - File Organization Scoring

Analyzes the directory tree and assigns an organization score.

Factors considered in scoring:

Branch Depth: Average and maximum directory depth.

Modification Age: Older files contribute to disorganization.

File Clutter: Presence of redundant or orphaned files.

Outputs a numerical score and suggestions for improvement.

### 5. ui.rs - Command-Line Interface & GUI

Handles user interactions through CLI commands.

Uses clap for argument parsing and structured command execution.

Future scope: Optional GUI implementation using egui.

### 6. disk.rs - Disk Health Analysis

Checks available disk space and usage distribution.

Integrates with platform-specific APIs:

Windows: Uses winapi to check drive health.

Linux/macOS: Uses nix to get filesystem stats.

Retrieves SMART health status if available.

### 7. config.rs - Configuration Management

Loads and saves user settings from a JSON config file.

Allows customization of:

Scan paths

Cleanup rules

Scoring parameters

### 8. utils.rs - Utility Functions

Contains helper functions for:

Logging and error handling.

Date/time formatting.

File I/O operations.

Provides reusable code for other modules.

## Contribution Guide

Fork the repository and create a new branch.

Follow the module structure defined above.

Write unit tests before submitting code changes.

Ensure cross-platform compatibility when adding system calls.

Submit a pull request with a clear description of changes.

Future Enhancements

Add a GUI for visual interaction.

Implement machine learning for smarter cleanup recommendations.

Integrate deeper disk diagnostics for failure prediction.




