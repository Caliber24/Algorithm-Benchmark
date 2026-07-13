use sysinfo::{ProcessesToUpdate, System};

pub struct MemoryMonitor {
    system: System,
    start_memory: u64,
}

impl MemoryMonitor {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
            start_memory: 0,
        }
    }

    pub fn start(&mut self) {
        self.system.refresh_processes(ProcessesToUpdate::All, true);

        if let Ok(pid) = sysinfo::get_current_pid() {
            if let Some(process) = self.system.process(pid) {
                self.start_memory = process.memory();
            }
        }
    }

    pub fn stop(&mut self) -> f64 {
        self.system.refresh_processes(ProcessesToUpdate::All, true);

        if let Ok(pid) = sysinfo::get_current_pid() {
            if let Some(process) = self.system.process(pid) {
                let end_memory = process.memory();

                if end_memory >= self.start_memory {
                    return (end_memory - self.start_memory) as f64 / 1024.0;
                }
            }
        }

        0.0
    }
}