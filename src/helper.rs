use std::process::Command;

pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const CYAN: &str = "\x1b[36m";
pub const BOLD: &str = "\x1b[1m";
pub const RESET: &str = "\x1b[0m";

#[derive(Debug)]
pub struct ProcessEntry {
    pub pid: String,
    pub process_name: String,
    pub user: String,
    pub uptime: String,
    pub cmd_line: String,
    pub fd: String,
    pub ty: String,
    pub name: String,
}

pub fn process_and_display(process_entries: Vec<ProcessEntry>, port: u16) {
    if process_entries.is_empty() {
        println!("{}No process found on port {}{}", YELLOW, port, RESET);
        return;
    }

    println!("\n{}Port {} Details:{}", BOLD, port, RESET);
    println!("{}", "=".repeat(40));

    let mut pids = Vec::new();

    for entry in &process_entries {
        println!(
            "\n{}┌─ {}Process{} {}",
            CYAN, BOLD, RESET, entry.process_name
        );
        println!("{}│  PID:        {}{}", GREEN, RESET, entry.pid);
        println!("{}│  User:       {}{}", GREEN, RESET, entry.user);
        println!("{}│  Uptime:     {}{}", GREEN, RESET, entry.uptime);
        println!("{}│  Type:       {}{}", GREEN, RESET, entry.ty);
        println!("{}│  FD:         {}{}", GREEN, RESET, entry.fd);
        println!("{}│  Name:       {}{}", GREEN, RESET, entry.name);
        println!("{}│  Command:   {}{}", GREEN, RESET, entry.cmd_line);
        println!("{}└{}", CYAN, RESET);

        pids.push(entry.pid.clone());
    }

    pids.sort();
    pids.dedup();

    if pids.is_empty() {
        return;
    }

    println!(
        "\n{}⚠ Found {} process(es) on port {}{}",
        YELLOW,
        pids.len(),
        port,
        RESET
    );
    print!("{}Kill these processes? (y/N): {}", RED, RESET);

    std::io::Write::flush(&mut std::io::stdout()).ok();

    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer).ok();

    if answer.trim().to_lowercase() == "y" {
        for pid in &pids {
            #[cfg(any(target_os = "linux", target_os = "macos"))]
            {
                Command::new("kill").args(["-9", pid]).status().ok();
            }
        }
        println!("{}Done! Port {} is now free.{}", GREEN, port, RESET);
    } else {
        println!("{}Cancelled.{}", RESET, RESET);
    }
}

#[allow(dead_code)]
pub fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let mins = (seconds % 3600) / 60;

    if days > 0 {
        format!("{}d {}h {}m", days, hours, mins)
    } else if hours > 0 {
        format!("{}h {}m", hours, mins)
    } else if mins > 0 {
        format!("{}m", mins)
    } else {
        format!("{}s", seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_uptime_seconds() {
        assert_eq!(format_uptime(0), "0s");
        assert_eq!(format_uptime(30), "30s");
        assert_eq!(format_uptime(59), "59s");
    }

    #[test]
    fn test_format_uptime_minutes() {
        assert_eq!(format_uptime(60), "1m");
        assert_eq!(format_uptime(90), "1m");
        assert_eq!(format_uptime(120), "2m");
        assert_eq!(format_uptime(3599), "59m");
    }

    #[test]
    fn test_format_uptime_hours() {
        assert_eq!(format_uptime(3600), "1h 0m");
        assert_eq!(format_uptime(3660), "1h 1m");
        assert_eq!(format_uptime(7200), "2h 0m");
        assert_eq!(format_uptime(86399), "23h 59m");
    }

    #[test]
    fn test_format_uptime_days() {
        assert_eq!(format_uptime(86400), "1d 0h 0m");
        assert_eq!(format_uptime(90000), "1d 1h 0m");
        assert_eq!(format_uptime(172800), "2d 0h 0m");
    }
}
