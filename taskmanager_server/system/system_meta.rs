extern crate sysinfo;

use byte_unit::{Byte, Unit, UnitType};
use sysinfo::{Components, Disks, Networks, System};

struct meta_info {
    name: String,
    value_i: u64,
    value_s: String,
}

pub fn get_system_meta_info() {
    let mut s = System::new_all();
    s.refresh_all();

    // all system information

    let memory_raw: u64 = s.total_memory();
    let used_mem_raw: u64 = s.used_memory();

    let memory = Byte::from_u64(memory_raw).get_appropriate_unit(UnitType::Binary);
    let used_memory = Byte::from_u64(used_mem_raw).get_appropriate_unit(UnitType::Binary);

    println!("mem percentage: {}%", (used_mem_raw * 100) / memory_raw);
    println!("memory: {}", memory.to_string());
    println!("used memory: {}", used_memory.to_string());

    println!("used swap   : {} bytes", s.used_swap());

    println!("System name: {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());

    println!("NB CPUs: {}", s.cpus().len());
}
