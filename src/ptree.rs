use std::collections::HashSet;
use libc::pid_t;
use sysinfo::{ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};

pub async fn find_process_tree(root: pid_t) -> anyhow::Result<Vec<pid_t>> {
    let system = System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::new()));
    let process_map = system.processes();

    if !process_map.contains_key(&root.into()) {
        return Ok(Default::default());
    }

    let mut tree = HashSet::new();
    tree.insert(root);

    for (pid, p) in process_map {
        if root.eq(&(*pid).into()) {
            continue;
        }

        let mut parent_pid = p.parent();
        loop {
            match parent_pid {
                Some(ppid) if tree.contains(&ppid.into()) => {
                    tree.insert((*pid).into());
                    break;
                }
                Some(ppid) => {
                    parent_pid = process_map.get(&ppid).and_then(|p| p.parent());
                }
                None => break,
            }
        }
    }

    Ok(tree.into_iter().collect())
}