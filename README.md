# RAM Observer RS

A terminal-based RAM monitoring tool written in Rust that provides real-time memory management capabilities using Microsoft's RAMMap utility.

## Why

I created this tool to automate Microsoft's RAMMap utility and solve a Windows memory management issue which I was facing. While RAMMap is powerful, it requires manual intervention each time you want to clear different types of memory caches. I wanted a solution that could:

- Monitor RAM usage automatically and execute RAMMap commands when memory usage exceeds certain thresholds
- Provide quick keyboard shortcuts for common RAMMap operations without opening the GUI
- Run in the background with minimal resource usage
- Give a clear terminal-based visualization of current memory status

Beyond solving this practical automation need, this project also served as an excellent opportunity to learn Rust.

## Demo

![Demo](assets/demo.gif)

## Features

- Real-time RAM and page file usage monitoring with visual gauges
- Automatic memory management based on configurable thresholds
- Direct integration with Microsoft's RAMMap utility
- ⌨Keyboard shortcuts for quick actions
- Action logging with timestamps
- Config support

## Memory Management Actions

- Empty Working Sets
- Empty System Working Sets
- Empty Modified Page Lists
- Empty Standby List
- Empty Priority 0 Standby List

## Controls

- `1-5`: Quick action keys for memory management
- `↑/↓`: Navigate through actions
- `Enter`: Execute selected action
- `Shift + A`: Cycle through auto-execution actions
- `Shift + T`: Cycle auto-execution threshold (50-95%, 5% increments)
- `q`: Quit application

## Auto-Execution

The tool can automatically execute memory management actions when RAM usage exceeds a configured threshold (default: 90%).

## Installation

1. Download the latest release from the releases page
2. Extract and run the executable
3. RAMMap will be automatically downloaded on first use (if not already present)

## Building from Source

```bash
git clone https://github.com/sh1ftd/ram-observer-rs.git
```

```bash
cd ram-observer-rs
```

```bash
cargo build --release

```

## Requirements

- Windows OS 64-bit (RAMMap dependency)
- Internet connection for first-time RAMMap download or you can download it manually and place in the same directory as the executable without internet connection

## License

MIT License

## Acknowledgments

- Uses Microsoft's [RAMMap utility from Sysinternals](https://docs.microsoft.com/en-us/sysinternals/downloads/rammap)
- Built with [Rust](https://www.rust-lang.org/) and [Ratatui](https://ratatui.rs/)
