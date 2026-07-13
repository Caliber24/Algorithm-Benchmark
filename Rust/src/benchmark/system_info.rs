use sysinfo::System;

pub struct SystemInfo;

impl SystemInfo {
    pub fn print_info() {
        let mut system = System::new_all();

        system.refresh_all();

        println!("{}", "=".repeat(70));
        println!("System Information");
        println!("{}", "=".repeat(70));

        println!(
            "Operating System : {}",
            System::name().unwrap_or_else(|| "Unknown".to_string())
        );

        println!(
            "OS Version       : {}",
            System::os_version().unwrap_or_else(|| "Unknown".to_string())
        );

        println!(
            "Kernel Version   : {}",
            System::kernel_version().unwrap_or_else(|| "Unknown".to_string())
        );

        println!(
            "Host Name        : {}",
            System::host_name().unwrap_or_else(|| "Unknown".to_string())
        );

        println!(
            "CPU Cores        : {}",
            system.cpus().len()
        );

        if let Some(cpu) = system.cpus().first() {
            println!(
                "CPU Brand        : {}",
                cpu.brand()
            );
        }

        println!(
            "Total Memory     : {:.2} GB",
            system.total_memory() as f64 / 1024.0 / 1024.0
        );

        println!(
            "Available Memory : {:.2} GB",
            system.available_memory() as f64 / 1024.0 / 1024.0
        );

        println!("{}", "=".repeat(70));
        println!();
    }
}