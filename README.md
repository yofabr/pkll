# pkll - Port Killer Lite

A lightweight CLI tool to find and kill processes running on a specific port.

## Features

- Cross-platform support (Linux, macOS, Windows)
- Shows detailed process information before killing
- Colored terminal output
- Prompts before killing (Linux)

## Installation

```bash
cargo install pkll
```

## Usage

```bash
pkll <port>
```

Example:
```bash
pkll 3000
```

## What it shows (Linux/macOS)

- Process name
- PID
- User
- Uptime
- Connection type (TCP/UDP)
- File descriptor
- Full command line

Then prompts for confirmation before killing.