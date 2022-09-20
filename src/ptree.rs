use std::collections::HashSet;
use libc::pid_t;
use sysinfo::{PidExt, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};

pub async fn find_all_descendants(needle: pid_t) -> anyhow::Result<Vec<pid_t>> {
    let system = System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::new()));
    let process_map = system.processes();

    let mut descendants = HashSet::new();

    for (pid, p) in process_map {
        let mut parent_pid = p.parent();
        loop {
            match parent_pid {
                Some(ppid) if ppid.as_u32() == needle as u32 => {
                    descendants.insert(pid.as_u32() as pid_t);
                    break;
                }
                Some(ppid) if descendants.contains(&(ppid.as_u32() as pid_t)) => {
                    break;
                }
                Some(ppid) => {
                    parent_pid = process_map.get(&ppid).and_then(|p| p.parent());
                }
                None => break,
            }
        }
    }

    Ok(descendants.into_iter().collect())
}