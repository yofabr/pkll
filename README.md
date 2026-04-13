<div align="center">

# ⚡ pkll

**Port Killer Lite** — Find and kill processes by port. Fast, safe, and informative.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/pkll.svg)](https://crates.io/crates/pkll)
[![GitHub Release](https://img.shields.io/github/v/release/yofabr/pkll)](https://github.com/yofabr/pkll/releases/latest)
![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey)
![Built with Rust](https://img.shields.io/badge/built%20with-Rust-orange)

</div>

---

## What is pkll?

Ever needed to kill a process blocking a port but had to juggle `lsof`, `netstat`, and `kill` just to do it? `pkll` handles it in one command — and shows you exactly what you're about to kill before you do it.

```
$ pkll 3000

  Process on port 3000
  ─────────────────────────────────────
  Name     node
  PID      48291
  User     yoftahe
  Uptime   1h 23m
  Type     TCP
  Command  node server.js

  Kill this process? [y/N]
```

---

## Features

- **Safe by default** — Shows process details and prompts for confirmation before killing
- **Informative** — Displays name, PID, user, uptime, connection type, and full command
- **Cross-platform** — Works on Linux, macOS, and Windows
- **Fast** — Single binary, no runtime, no dependencies
- **Colored output** — Easy to read at a glance

---

## Installation

### macOS & Linux

```sh
curl -fsSL https://github.com/yofabr/pkll/releases/latest/download/install.sh | sh
```

### Windows (PowerShell)

```powershell
irm https://github.com/yofabr/pkll/releases/latest/download/install.ps1 | iex
```

### Via Cargo

```sh
cargo install pkll
```

### Manual

Download the binary for your platform from the [latest release](https://github.com/yofabr/pkll/releases/latest):

| Platform | File |
|---|---|
| Linux x64 | `pkll-x86_64-unknown-linux-gnu.tar.xz` |
| Linux ARM64 | `pkll-aarch64-unknown-linux-gnu.tar.xz` |
| macOS Intel | `pkll-x86_64-apple-darwin.tar.xz` |
| macOS Apple Silicon | `pkll-aarch64-apple-darwin.tar.xz` |
| Windows x64 | `pkll-x86_64-pc-windows-msvc.zip` |
| Windows ARM64 | `pkll-aarch64-pc-windows-msvc.zip` |

---

## Usage

```sh
pkll <port>
```

**Examples:**

```sh
pkll 3000    # Kill whatever is running on port 3000
pkll 8080    # Kill whatever is running on port 8080
pkll 5432    # Free up your Postgres port
```

---

## What it shows

Before killing anything, `pkll` displays:

| Field | Description |
|---|---|
| **Name** | Process name |
| **PID** | Process ID |
| **User** | Owning user |
| **Uptime** | How long the process has been running |
| **Type** | TCP or UDP |
| **FD** | File descriptor |
| **Command** | Full command line |

You always get a confirmation prompt — no accidental kills.

---

## License

[MIT](LICENSE) © [Yoftahe Abraham](https://github.com/yofabr)
