use std::process::Command;

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const CYAN: &str = "\x1b[36m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

pub fn handle(os: &str, port: u16) {
     match os {
        "windows" => handler_for_windows(port),
        "macos" => handler_for_unix(port),
        "linux" => handler_for_unix(port),
        _ => println!("Unable to detect current OS")
    }
}

fn get_process_info(pid: &str) -> (String, String, String) {
    let ps_output = Command::new("ps")
        .args(["-p", pid, "-o", "comm=,etime=,user="])
        .output();

    if let Ok(output) = ps_output {
        let output_str = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = output_str.trim().split_whitespace().collect();
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

pub fn handler_for_unix(port: u16) {
    let output = Command::new("lsof")
        .args(["-i", &format!(":{}", port), "-P", "-n"])
        .output()
        .expect("Failed to run lsof");

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
    let mut pids: Vec<String> = Vec::new();

    println!("\n{}Port {} Details:{}", BOLD, port, RESET);
    println!("{}", "=".repeat(40));

    for line in &process_lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 9 {
            continue;
        }

        let pid = parts[1].to_string();
        let process = parts[0].to_string();
        let user = parts[2].to_string();
        let fd = parts[3].to_string();
        let ty = parts[4].to_string();
        let name = parts[8..].join(" ");

        let (_proc_name, uptime, _proc_user) = get_process_info(&pid);
        let cmd_line = get_command_line(&pid);

        println!("\n{}┌─ {}Process{} {}", CYAN, BOLD, RESET, process);
        println!("{}│  PID:        {}{}", GREEN, RESET, pid);
        println!("{}│  User:       {}{}", GREEN, RESET, user);
        println!(
            "{}│  Uptime:     {}{}",
            GREEN,
            RESET,
            if uptime.is_empty() {
                "N/A".to_string()
            } else {
                uptime
            }
        );
        println!("{}│  Type:       {}{}", GREEN, RESET, ty);
        println!("{}│  FD:         {}{}", GREEN, RESET, fd);
        println!("{}│  Name:       {}{}", GREEN, RESET, name);
        println!("{}│  Command:   {}{}", GREEN, RESET, cmd_line);
        println!("{}└{}", CYAN, RESET);

        pids.push(pid);
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
        for pid in &pids {
            // println!("{}🔪 Killing PID {}{} on port {}", RED, BOLD, pid, port);
            Command::new("kill").args(["-9", pid]).status().ok();
        }
        println!("{}Done! Port {} is now free.{}", GREEN, port, RESET);
    } else {
        println!("{}Cancelled.{}", RESET, RESET);
    }
}

pub fn handler_for_windows(port: u16) {
    println!("{}Port: {}{}", BLUE, port, RESET);
}
