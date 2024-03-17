extern crate sysinfo;

use std::hash::Hash;

use maplit::hashmap;
use std::collections::HashMap;

use byte_unit::{Byte, Unit, UnitType};
use sysinfo::{Components, Disks, Networks, System};

#[derive(Debug)]
pub struct MetaInfo {
    name: String,
    value_i: u64,
    value_s: String,
}

pub fn get_system_meta_info() -> Vec<MetaInfo> {
    let mut meta_infos: Vec<MetaInfo> = Vec::new();

    let mut s = System::new_all();
    s.refresh_all();

    let meta_info_num: HashMap<&'static str, (String, u64)> = {
        let total_memory: u64 = s.total_memory();
        let used_memory: u64 = s.used_memory();
        let available_memory: u64 = s.available_memory();
        let percentage_used_memory: u64 = (used_memory * 100) / total_memory;

        let total_swap: u64 = s.total_swap();
        let used_swap: u64 = s.used_swap();
        let free_swap: u64 = s.free_swap();

        hashmap! {
            "total_memory" => (convert_memory(total_memory), total_memory),
            "used_memory" => (convert_memory(used_memory), used_memory),
            "available_memory" => (convert_memory(available_memory), available_memory),
            "percentage_used_memory" => (format!("{percentage_used_memory}%"), percentage_used_memory),
            "free_swap" => (convert_memory(free_swap), free_swap),
            "used_swap" => (convert_memory(used_swap), used_swap),
            "total_swap" => (convert_memory(total_swap), total_swap)
        }
    };

    for (name, val) in meta_info_num {
        println!("{}: {}", name, val.0)
    }

    println!("System name: {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());

    println!("NB CPUs: {}", s.cpus().len());

    return meta_infos;
}

fn convert_memory(value: u64) -> String {
    return Byte::from_u64(value)
        .get_appropriate_unit(UnitType::Binary)
        .to_string();
}
