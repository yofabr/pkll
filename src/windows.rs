use listeners::Listener;
use sysinfo::{Pid, ProcessesToUpdate, System};

use crate::helper::{ProcessEntry, RED, RESET, YELLOW, format_uptime, process_and_display};

pub fn handle(port: u16) {
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

    let mut pids: Vec<u32> = Vec::new();
    let mut process_entries: Vec<ProcessEntry> = Vec::new();

    for listener in &matching {
        let process = &listener.process;
        let pid = process.pid;
        let process_name = process.name.clone();
        let local_addr = listener.socket.to_string();
        let protocol = format!("{:?}", listener.protocol);

        let (username, uptime, cmd_line) = get_process_info_sysinfo(pid);

        process_entries.push(ProcessEntry {
            pid: pid.to_string(),
            process_name: process_name.clone(),
            user: username,
            uptime,
            cmd_line,
            fd: "socket".to_string(),
            ty: protocol,
            name: local_addr,
        });

        if !pids.contains(&pid) {
            pids.push(pid);
        }
    }

    println!("\nPort {} Details:", port);
    println!("{}", "=".repeat(40));
    process_and_display(process_entries, port);

    if pids.is_empty() {
        return;
    }

    prompt_and_kill(&pids, port);
}

fn get_process_info_sysinfo(pid: u32) -> (String, String, String) {
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    if let Some(proc) = sys.process(Pid::from_u32(pid)) {
        let cmd: String = proc
            .cmd()
            .iter()
            .map(|s| s.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ");

        let start_time = proc.start_time();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let uptime_secs = now.saturating_sub(start_time);
        let uptime = format_uptime(uptime_secs);

        ("N/A".to_string(), uptime, cmd)
    } else {
        ("N/A".to_string(), "N/A".to_string(), String::new())
    }
}

fn kill_process_sysinfo(pid: u32) {
    let mut sys = System::new();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    if let Some(proc) = sys.process(Pid::from_u32(pid)) {
        proc.kill();
    }
}

fn prompt_and_kill(pids: &[u32], port: u16) {
    println!("\nFound {} process(es) on port {}", pids.len(), port);
    print!("Kill these processes? (y/N): ");

    std::io::Write::flush(&mut std::io::stdout()).ok();

    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer).ok();

    if answer.trim().to_lowercase() == "y" {
        for &pid in pids {
            kill_process_sysinfo(pid);
        }
        println!("Done! Port {} is now free.", port);
    } else {
        println!("Cancelled.");
    }
}
