# zyr

[![Build](https://github.com/SerbanUntu/zyr/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/SerbanUntu/zyr/actions/workflows/build.yml)

A command-line productivity timer and time tracking tool built in Rust. Track work sessions, manage time blocks, and view daily statistics directly from your terminal.

![Banner](./assets/banner.png)

## Overview

zyr provides a lightweight alternative to GUI-based time tracking applications. It focuses on simplicity and reliability, storing all data locally without requiring network connectivity or complex dependencies.

The tool operates on the concept of time blocks, which represent periods of work or activity with defined start and end times. Each time block is categorized (e.g., "code", "study", "break") and persisted on your local filesystem.

## Motivation

Traditional GUI-based time tracking applications often suffer from performance issues, including screen tearing, network instability, and interface unresponsiveness. These problems can disrupt workflow and create friction.

zyr addresses these issues by providing a terminal-based interface that is fast, reliable, and operates entirely offline. The minimal resource footprint ensures consistent performance across different systems and environments.

## Usage

### Timer Commands

The timer functionality allows you to track work sessions in real-time.

#### Start a Timer
```bash
# Start a basic timer for coding
zyr timer start code

# Start a timer with a specific duration. This timer will count backwards.
zyr timer start study --duration 2h30m

# Start a timer and immediately show it
zyr timer start break --show
```

![Running zyr timer start](./assets/zyr_timer_start.gif)

#### Manage Running Timers
```bash
# Add time to a running timer
zyr timer add 15m

# Subtract time from a running timer
zyr timer sub 5m

# Stop the currently running timer
zyr timer end

# Display the current timer status
zyr timer show
```

The `timer show` command provides a live display that updates every second. Press Ctrl+C to exit the display.

### Plan Commands

Plan commands allow you to manually create, modify, and delete time blocks without using the timer.

#### Add Time Blocks
```bash
# Create a time block with start time and duration
# Currently, only RFC 3339 timestamps are supported.
zyr plan add "meeting" --from "2024-01-15T10:00:00" --duration 1h

# Create a time block with start and end times
zyr plan add "research" --from "2024-01-15T14:00:00" --to "2024-01-15T16:30:00"
```

#### Edit Time Blocks
```bash
# Edit a specific time block by order number (e.g., the third most recent one)
zyr plan edit 2 --category "debugging" --from "2024-01-15T09:00:00"

# Edit the most recent time block. Same as using 0 for the order number.
zyr plan edit --last --category "documentation" --duration 45m

# Interactive mode for selecting time blocks
zyr plan edit --category "documentation"
```

#### Delete Time Blocks
```bash
# Delete the most recent time block
zyr plan del --last

# Delete a specific time block (e.g., the fourth most recent one)
zyr plan del 3

# Interactive deletion
zyr plan del
```

![Running zyr plan del](./assets/zyr_plan_del.gif)

### View Commands

#### Daily Statistics
```bash
# View today's work summary
zyr view
```

![Running zyr view](./assets/zyr_view.gif)

The view command displays:
- Total time worked (excluding breaks)
- Total break time
- Overall time spent
- Breakdown by category

### Data Management

#### Clear All Data
```bash
# Reset all stored data
zyr clear
```

This command removes all time blocks and resets the application to its initial state.

## Data Storage

zyr stores all data locally in JSON format. The data file is located in the appropriate application data directory for your operating system:

- **Linux**: `~/.local/share/zyr/data.json`
- **Windows**: `%APPDATA%\zyr\zyr\data\data.json`
- **macOS**: `~/Library/Application Support/org/zyr/zyr/data.json`

The data file is created automatically on first run and contains your time blocks and category information.

## Time Format

zyr supports the following time input formats:

- **Duration**: `1h30m45s`, `2h`, `30m`, `45s`
- **Timestamps**: `2024-01-15T10:00:00`, `2024-01-15T10:00:00+02:00`

Duration parsing supports any combination of hours (h), minutes (m), and seconds (s) in any order. Whitespace is ignored.

## Codebase

### Dependencies

- **chrono**: Date and time handling
- **clap**: Command-line argument parsing
- **serde**: Serialization and deserialization
- **serde_json**: JSON data format
- **humantime**: Human-readable time parsing
- **crossterm**: Cross-platform terminal manipulation
- **directories**: Platform-specific directory resolution

### Project Structure

```
zyr/
├── src/
│   ├── cli/
│   │   ├── clear.rs      # Data clearing functionality
│   │   ├── plan.rs       # Time block management
│   │   ├── timer.rs      # Timer operations
│   │   └── view.rs       # Statistics display
│   ├── cli.rs            # Main CLI interface
│   ├── domain.rs         # Core data structures
│   ├── main.rs           # Application entry point
│   ├── terminal.rs       # Terminal utilities
│   └── utils.rs          # Helper functions
├── assets/               # README assets
└── Cargo.toml            # Rust project configuration
```

#### Structure Descriptions

- **cli/**: Contains the implementation of all command-line subcommands
- **domain.rs**: Defines core data structures including `Timer`, `TimeBlock`, and `Data`
- **terminal.rs**: Provides terminal manipulation utilities for interactive displays
- **utils.rs**: Contains parsers, file operations, and time utility functions

### Architecture

The application follows a modular architecture with separation of concerns:

1. **Domain Layer**: Core data structures
2. **CLI Layer**: Command-line interface implementation
3. **Utility Layer**: Shared functionality and helper functions

The `Executable` trait provides a consistent interface for all commands, while the `Data` struct manages persistence and state management.
