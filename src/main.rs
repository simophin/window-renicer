use std::collections::HashSet;
use std::time::Instant;
use async_std::prelude::StreamExt;

use errno::errno;
use libc::{c_int, id_t, PRIO_PROCESS, setpriority};
use signal_hook_async_std::Signals;
use zbus::{Connection, dbus_interface};

const MAX_ACTIVATED: usize = 5;

struct WindowInfo {
    pid: id_t,
    last_active_time: Instant,
}

struct WindowRenicer {
    max_windows: usize,
    windows: Vec<WindowInfo>,
}

#[dbus_interface(name = "dev.fanchao.WindowRenicer")]
impl WindowRenicer {
    async fn window_activated(&mut self, pid: &str) {
        let pid: id_t = match pid.parse() {
            Ok(v) => v,
            Err(e) => {
                log::error!("PID value {pid} is not a numerical value: {e:?}");
                return;
            }
        };


        log::info!("Window PID = {pid} activated!");

        // Find existing window
        match self.windows.iter_mut().find(|i| i.pid == pid) {
            Some(window) => window.last_active_time = Instant::now(),
            None => self.windows.push(WindowInfo {
                pid,
                last_active_time: Instant::now(),
            }),
        };

        // Reorder windows
        self.windows.sort_by(|lhs, rhs| rhs.last_active_time.cmp(&lhs.last_active_time));

        // Re-apply nice value
        let mut dead_processes = HashSet::new();
        for (i, window) in self.windows.iter().enumerate() {
            let nice: c_int = (i as c_int - self.max_windows as c_int).min(0);
            let pid = window.pid;
            log::info!("Setting process {pid}'s nice to {nice}");
            let rc = unsafe {
                setpriority(PRIO_PROCESS, pid, nice)
            };
            if rc < 0 {
                let err = errno();
                if err.0 == libc::ESRCH {
                    dead_processes.insert(pid);
                    log::info!("Process {pid} no longer exists. Removing");
                } else {
                    log::error!("Error setting nice on process: {pid}: {}", err);
                }
            }
        }

        // Remove dead processes
        self.windows.retain(|info| !dead_processes.contains(&info.pid));

        // Strip away excessive processes
        while self.windows.len() > self.max_windows {
            let info = self.windows.pop().unwrap();
            log::info!("Removing PID = {} from monitoring", info.pid);
        }
    }

    async fn window_removed(&mut self, pid: &str) {
        let pid: id_t = match pid.parse() {
            Ok(v) => v,
            Err(e) => {
                log::error!("PID value {pid} is not a numerical value: {e:?}");
                return;
            }
        };

        log::info!("Window PID = {pid} removed!");
        self.windows.retain_mut(|info| info.pid != pid);
    }
}


#[async_std::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let conn = Connection::session().await?;

    conn.object_server()
        .at("/dev/fanchao/WindowRenicer", WindowRenicer {
            max_windows: MAX_ACTIVATED,
            windows: Default::default(),
        })
        .await?;

    conn.request_name("dev.fanchao.WindowRenicer").await?;

    use signal_hook::consts::*;
    let mut signals = Signals::new(&[SIGINT])?;
    if let Some(s) = signals.next().await {
        log::info!("Received signal {s}. Quitting...");
    }

    Ok(())
}
