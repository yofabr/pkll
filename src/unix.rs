use std::io::ErrorKind;
use std::process::Command;

use crate::helper::{ProcessEntry, RED, RESET, YELLOW, process_and_display};

pub fn handle(port: u16) {
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
                    uptime: if uptime.is_empty() {
                        "N/A".to_string()
                    } else {
                        uptime
                    },
                    cmd_line: if cmd_line.is_empty() {
                        "N/A".to_string()
                    } else {
                        cmd_line
                    },
                    fd,
                    ty,
                    name,
                });
            }

            process_and_display(process_entries, port);
        }
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                #[cfg(target_os = "linux")]
                {
                    handle_with_ss(port);
                }
                #[cfg(target_os = "macos")]
                {
                    eprintln!(
                        "{}lsof is not installed. macOS requires lsof to be available.{}",
                        RED, RESET
                    );
                }
            } else {
                eprintln!("{}Failed to run lsof: {}{}", RED, e, RESET);
            }
        }
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

#[cfg(target_os = "linux")]
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
            process_name: if process_name.is_empty() {
                "N/A".to_string()
            } else {
                process_name
            },
            user: if user.is_empty() {
                "N/A".to_string()
            } else {
                user
            },
            uptime: if uptime.is_empty() {
                "N/A".to_string()
            } else {
                uptime
            },
            cmd_line: if cmd_line.is_empty() {
                "N/A".to_string()
            } else {
                cmd_line
            },
            fd: "N/A".to_string(),
            ty: "socket".to_string(),
            name: format!("port {}", port),
        });
    }

    process_and_display(process_entries, port);
}

#[cfg(target_os = "linux")]
fn extract_pid_from_ss_line(line: &str) -> Option<String> {
    let users_pos = line.find("users:(")?;
    let users_section = &line[users_pos..];
    let pid_pos = users_section.find("pid=")?;
    let pid_start = pid_pos + 4;
    let pid_str = &users_section[pid_start..];
    let end = pid_str.find(|c: char| !c.is_ascii_digit())?;
    Some(pid_str[..end].to_string())
}
