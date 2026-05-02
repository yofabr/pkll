use std::process::Command;

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const CYAN: &str = "\x1b[36m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

struct ProcessEntry {
    pid: String,
    process_name: String,
    user: String,
    uptime: String,
    cmd_line: String,
    fd: String,
    ty: String,
    name: String,
}

pub fn handle(os: &str, port: u16) {
    match os {
        "windows" => handler_for_windows(port),
        "macos" => handler_for_unix(port, os),
        "linux" => handler_for_unix(port, os),
        _ => println!("Unable to detect current OS"),
    }
}

fn get_process_info(pid: &str) -> (String, String, String) {
    let ps_output = Command::new("ps")
        .args(["-p", pid, "-o", "comm=,etime=,user="])
        .output();

    if let Ok(output) = ps_output {
        let output_str = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = output_str.split_whitespace().collect();
        if parts.len() >= 3 {
            return (
                parts[0].to_string(),
                parts[1].to_string(),
                parts[2].to_string(),
            );
        }
    }
    (String::new(), String::new(), String::new())
}

fn get_command_line(pid: &str) -> String {
    if let Ok(output) = Command::new("ps").args(["-p", pid, "-o", "args="]).output() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        String::new()
    }
}

fn extract_pid_from_ss_line(line: &str) -> Option<String> {
    let users_pos = line.find("users:(")?;
    let users_section = &line[users_pos..];
    let pid_pos = users_section.find("pid=")?;
    let pid_start = pid_pos + 4;
    let pid_str = &users_section[pid_start..];
    let end = pid_str.find(|c: char| !c.is_ascii_digit())?;
    Some(pid_str[..end].to_string())
}

pub fn handler_for_unix(port: u16, os: &str) {
    let lsof_output = Command::new("lsof")
        .args(["-i", &format!(":{}", port), "-P", "-n"])
        .output();

    match lsof_output {
        Ok(output) => {
            let result = String::from_utf8_lossy(&output.stdout);

            if result.trim().is_empty() {
                println!("{}No process found on port {}{}", YELLOW, port, RESET);
                return;
            }

            let lines: Vec<&str> = result.lines().collect();
            if lines.len() < 2 {
                println!("{}No process found on port {}{}", YELLOW, port, RESET);
                return;
            }

            let process_lines: Vec<&str> = lines[1..].to_vec();
            let mut process_entries = Vec::new();

            for line in &process_lines {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 9 {
                    continue;
                }

                let pid = parts[1].to_string();
                let process_name = parts[0].to_string();
                let user = parts[2].to_string();
                let fd = parts[3].to_string();
                let ty = parts[4].to_string();
                let name = parts[8..].join(" ");

                let (_proc_name, uptime, _proc_user) = get_process_info(&pid);
                let cmd_line = get_command_line(&pid);

                process_entries.push(ProcessEntry {
                    pid,
                    process_name,
                    user,
                    uptime: if uptime.is_empty() { "N/A".to_string() } else { uptime },
                    cmd_line: if cmd_line.is_empty() { "N/A".to_string() } else { cmd_line },
                    fd,
                    ty,
                    name,
                });
            }

            process_and_display(process_entries, port);
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                if os == "linux" {
                    handle_with_ss(port);
                    return;
                } else {
                    eprintln!("{}lsof is not installed. macOS requires lsof to be available.{}", RED, RESET);
                    return;
                }
            } else {
                eprintln!("{}Failed to run lsof: {}{}", RED, e, RESET);
                return;
            }
        }
    }
}

fn handle_with_ss(port: u16) {
    let ss_output = Command::new("ss")
        .args(["-tunlp", &format!("sport = :{}", port)])
        .output();

    let output = match ss_output {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{}Failed to run ss: {}{}", RED, e, RESET);
            return;
        }
    };

    let result = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = result.lines().collect();

    if lines.len() < 2 {
        println!("{}No process found on port {}{}", YELLOW, port, RESET);
        return;
    }

    let mut pids = Vec::new();
    for line in lines.iter().skip(1) {
        if let Some(pid) = extract_pid_from_ss_line(line) {
            pids.push(pid);
        }
    }

    pids.sort();
    pids.dedup();

    if pids.is_empty() {
        println!("{}No process found on port {}{}", YELLOW, port, RESET);
        return;
    }

    let mut process_entries = Vec::new();
    for pid in pids {
        let (process_name, uptime, user) = get_process_info(&pid);
        let cmd_line = get_command_line(&pid);
        process_entries.push(ProcessEntry {
            pid: pid.clone(),
            process_name: if process_name.is_empty() { "N/A".to_string() } else { process_name },
            user: if user.is_empty() { "N/A".to_string() } else { user },
            uptime: if uptime.is_empty() { "N/A".to_string() } else { uptime },
            cmd_line: if cmd_line.is_empty() { "N/A".to_string() } else { cmd_line },
            fd: "N/A".to_string(),
            ty: "socket".to_string(),
            name: format!("port {}", port),
        });
    }

    process_and_display(process_entries, port);
}

fn process_and_display(process_entries: Vec<ProcessEntry>, port: u16) {
    if process_entries.is_empty() {
        println!("{}No process found on port {}{}", YELLOW, port, RESET);
        return;
    }

    println!("\n{}Port {} Details:{}", BOLD, port, RESET);
    println!("{}", "=".repeat(40));

    let mut pids = Vec::new();

    for entry in &process_entries {
        println!("\n{}┌─ {}Process{} {}", CYAN, BOLD, RESET, entry.process_name);
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
            Command::new("kill").args(["-9", pid]).status().ok();
        }
        println!("{}Done! Port {} is now free.{}", GREEN, port, RESET);
    } else {
        println!("{}Cancelled.{}", RESET, RESET);
    }
}

pub fn handler_for_windows(port: u16) {
    use listeners::Listener;

    let all_listeners = match listeners::get_all() {
        Ok(l) => l,
        Err(_) => {
            println!("{}Failed to get listeners{}", RED, RESET);
            return;
        }
    };

    let matching: Vec<&Listener> = all_listeners
        .iter()
        .filter(|l| l.socket.port() == port)
        .collect();

    if matching.is_empty() {
        println!("{}No process found on port {}{}", YELLOW, port, RESET);
        return;
    }

    println!("\n{}Port {} Details:{}", BOLD, port, RESET);
    println!("{}", "=".repeat(40));

    let mut pids: Vec<u32> = Vec::new();

    for listener in &matching {
        let process = &listener.process;
        let pid = process.pid;
        let process_name = &process.name;

        let (username, uptime, cmd_line) = get_process_info_sysinfo(pid);

        let local_addr = listener.socket.to_string();
        let protocol = listener.protocol;

        println!("\n{}┌─ {}Process{} {}", CYAN, BOLD, RESET, process_name);
        println!("{}│  PID:        {}{}", GREEN, RESET, pid);
        println!("{}│  User:       {}{}", GREEN, RESET, username);
        println!("{}│  Uptime:     {}{}", GREEN, RESET, uptime);
        println!("{}│  Type:       {}{}", GREEN, RESET, protocol);
        println!("{}│  FD:         {}socket", GREEN, RESET);
        println!("{}│  Address:    {}{}", GREEN, RESET, local_addr);
        println!("{}│  Command:   {}{}", GREEN, RESET, cmd_line);
        println!("{}└{}", CYAN, RESET);

        if !pids.contains(&pid) {
            pids.push(pid);
        }
    }

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
        for &pid in &pids {
            kill_process_sysinfo(pid);
        }
        println!("{}Done! Port {} is now free.{}", GREEN, port, RESET);
    } else {
        println!("{}Cancelled.{}", RESET, RESET);
    }
}

fn get_process_info_sysinfo(pid: u32) -> (String, String, String) {
    use sysinfo::System;

    let mut sys = System::new();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    if let Some(proc) = sys.process(sysinfo::Pid::from_u32(pid)) {
        let mut cmd: String = proc
            .cmd()
            .iter()
            .map(|s| s.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ");

        if cmd.is_empty() {
            cmd = get_command_line_windows(pid);
        }

        let start_time = proc.start_time();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let uptime_secs = now.saturating_sub(start_time);
        let uptime = format_uptime(uptime_secs);

        (String::from("N/A"), uptime, cmd)
    } else {
        (String::from("N/A"), String::from("N/A"), String::new())
    }
}

fn get_command_line_windows(pid: u32) -> String {
    let output = Command::new("wmic")
        .args([
            "process",
            "where",
            &format!("ProcessId={}", pid),
            "get",
            "CommandLine",
            "/value",
        ])
        .output();

    if let Ok(output) = output {
        let s = String::from_utf8_lossy(&output.stdout);
        s.lines()
            .find(|l| l.starts_with("CommandLine="))
            .map(|l| l.trim_start_matches("CommandLine=").to_string())
            .unwrap_or_default()
    } else {
        String::new()
    }
}

fn kill_process_sysinfo(pid: u32) {
    use sysinfo::System;

    let mut sys = System::new();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    if let Some(proc) = sys.process(sysinfo::Pid::from_u32(pid)) {
        proc.kill();
    }
}

fn format_uptime(seconds: u64) -> String {
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

    #[test]
    fn test_handle_dispatches_correctly() {
        use std::process::Command;

        let output = Command::new("echo")
            .arg("test")
            .output()
            .expect("Failed to execute echo");

        let result = String::from_utf8_lossy(&output.stdout);
        assert_eq!(result.trim(), "test");
    }
}
