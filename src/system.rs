use sysinfo::{Cpu, System, MINIMUM_CPU_UPDATE_INTERVAL};
use tokio::time::sleep;

pub struct Sys {
    system: System,
}

#[allow(dead_code)]
impl Sys {
    pub fn new() -> Self {
        let mut res = Self {
            system: System::new_all(),
        };
        res.system.refresh_all();
        res
    }

    pub async fn cpu_usage(&mut self) -> Vec<&Cpu> {
        sleep(MINIMUM_CPU_UPDATE_INTERVAL).await;
        self.system.refresh_cpu();

        self.system.cpus().iter().collect::<Vec<&Cpu>>()
    }

    pub async fn average_cpu_usage(&mut self) -> &Cpu {
        sleep(MINIMUM_CPU_UPDATE_INTERVAL).await;
        self.system.refresh_cpu();

        self.system.global_cpu_info()
    }

    pub async fn memory_usage(&mut self) -> MemoryCount {
        let system = &mut self.system;
        system.refresh_memory();

        MemoryCount {
            available_memory: system.available_memory(),
            total_memory: system.total_memory(),
        }
    }
}

#[derive(Debug)]
pub struct MemoryCount {
    available_memory: u64,
    total_memory: u64,
}

impl MemoryCount {
    pub fn get_available_memory(&self) -> String {
        format!(
            "{:.2}GB",
            self.available_memory as f64 / (1024 * 1024 * 1024) as f64
        )
    }

    pub fn get_total_memory(&self) -> String {
        format!(
            "{:.2}GB",
            self.total_memory as f64 / (1024 * 1024 * 1024) as f64
        )
    }

    pub fn get_percent_usage(&self) -> String {
        format!(
            "{:.2}%",
            self.available_memory as f64 * 100.0 / self.total_memory as f64
        )
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test() {
        let proxy = sysproxy::Sysproxy::get_system_proxy().unwrap();
        println!("{:?}", proxy);
    }
}
