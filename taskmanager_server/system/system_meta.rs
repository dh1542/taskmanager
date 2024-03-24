extern crate sysinfo;

use byte_unit::{Byte, UnitType};
use serde::Serialize;
use sysinfo::System;

#[derive(Debug, Serialize)]
pub struct MetaInfo {
    name: String,
    value_i: u64,
    value_s: String,
}

pub fn get_system_meta_info() -> Vec<MetaInfo> {
    let mut s = System::new_all();
    s.refresh_all();
    return get_meta_info_vec(s);
}

///
/// Returns a vec of MetaInfo datastructures with relevant meta information
/// about the system running
///
fn get_meta_info_vec(s: System) -> Vec<MetaInfo> {
    let mut meta_infos: Vec<MetaInfo> = Vec::new();

    let total_memory: u64 = s.total_memory();
    let used_memory: u64 = s.used_memory();
    let available_memory: u64 = s.available_memory();
    let percentage_used_memory: u64 = (used_memory * 100) / total_memory;
    let total_swap: u64 = s.total_swap();
    let used_swap: u64 = s.used_swap();
    let free_swap: u64 = s.free_swap();
    let num_cpus: u64 = s.cpus().len() as u64;
    let system_name: Option<String> = System::name();
    let system_kernel: Option<String> = System::kernel_version();
    let os_version: Option<String> = System::os_version();
    let host_name: Option<String> = System::host_name();

    meta_infos.push(create_meta_info_struct(
        "Total Memory".to_string(),
        total_memory,
        convert_memory(total_memory),
    ));

    meta_infos.push(create_meta_info_struct(
        "Used Memory".to_string(),
        used_memory,
        convert_memory(used_memory),
    ));

    meta_infos.push(create_meta_info_struct(
        "Available Memory".to_string(),
        available_memory,
        convert_memory(available_memory),
    ));
    meta_infos.push(create_meta_info_struct(
        "Percentage Memory Used".to_string(),
        percentage_used_memory,
        format!("{}%", percentage_used_memory),
    ));
    meta_infos.push(create_meta_info_struct(
        "Total Swap".to_string(),
        total_swap,
        convert_memory(total_swap),
    ));
    meta_infos.push(create_meta_info_struct(
        "Used Swap".to_string(),
        used_swap,
        convert_memory(used_swap),
    ));
    meta_infos.push(create_meta_info_struct(
        "Free Swap".to_string(),
        free_swap,
        convert_memory(free_swap),
    ));
    meta_infos.push(create_meta_info_struct(
        "System Name".to_string(),
        0,
        get_string_from_option(system_name),
    ));
    meta_infos.push(create_meta_info_struct(
        "Host Name".to_string(),
        0,
        get_string_from_option(host_name),
    ));
    meta_infos.push(create_meta_info_struct(
        "Number of CPUs".to_string(),
        num_cpus,
        num_cpus.to_string(),
    ));
    meta_infos.push(create_meta_info_struct(
        "Kernel Name".to_string(),
        0,
        get_string_from_option(system_kernel),
    ));
    meta_infos.push(create_meta_info_struct(
        "OS Version".to_string(),
        0,
        get_string_from_option(os_version),
    ));

    return meta_infos;
}

///
/// Converts a byte value received as u64 into a string with an appropriate unit
///
fn convert_memory(value: u64) -> String {
    return Byte::from_u64(value)
        .get_appropriate_unit(UnitType::Binary)
        .to_string();
}

///
/// Returns the value of an Option<String> as a string.
/// If the option is empty an empty string gets returned.
///
fn get_string_from_option(option_string: Option<String>) -> String {
    match option_string {
        Some(name) => return name,
        None => return "".to_string(),
    }
}

fn create_meta_info_struct(name: String, value_i: u64, value_s: String) -> MetaInfo {
    return MetaInfo {
        name: name,
        value_i: value_i,
        value_s: value_s,
    };
}
