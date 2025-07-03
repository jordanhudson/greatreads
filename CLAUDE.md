# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust project called "greatreads" using the 2024 edition. The project is currently a minimal Hello World application.

## Common Commands

### Building and Running
- `cargo build` - Build the project
- `cargo run` - Build and run the project
- `cargo build --release` - Build with optimizations
- `cargo check` - Check for compilation errors without building

### Testing and Linting
- `cargo test` - Run tests
- `cargo fmt` - Format code
- `cargo clippy` - Run linter

### Development
- `cargo watch -x run` - Auto-rebuild and run on file changes (if cargo-watch is installed)
- `cargo clean` - Clean build artifacts

## Project Structure

- `src/main.rs` - Main entry point containing the application logic
- `Cargo.toml` - Project configuration and dependencies
- `target/` - Build artifacts (generated, not tracked in git)

## Architecture

Currently a simple single-file Rust application with no external dependencies. The main function serves as the entry point.