//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/internal.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0-only
//
// acpi/internal.h
// For use by Linux/ACPI infrastructure, not drivers
//
// Copyright (c) 2009, Intel Corporation.
//

extern "C" {
    pub fn early_acpi_osi_init() -> c_int;
}
extern "C" {
    pub fn acpi_osi_init() -> c_int;
}
extern "C" {
    pub fn acpi_os_initialize1() -> acpi_status;
}
extern "C" {
    pub fn acpi_scan_init();
}

extern "C" {
    pub fn acpi_pci_root_init();
}
extern "C" {
    pub fn acpi_pci_link_init();
}

extern "C" {
    pub fn acpi_processor_init();
}
extern "C" {
    pub fn acpi_platform_init();
}
extern "C" {
    pub fn acpi_pnp_init();
}
extern "C" {
    pub fn acpi_sysfs_init() -> c_int;
}
extern "C" {
    pub fn acpi_gpe_apply_masked_gpes();
}
extern "C" {
    pub fn acpi_container_init();
}
extern "C" {
    pub fn acpi_memory_hotplug_init();
}

extern "C" {
    pub fn pci_ioapic_remove(root: *mut acpi_pci_root);
}
extern "C" {
    pub fn acpi_ioapic_remove(root: *mut acpi_pci_root) -> c_int;
}

extern "C" {
    pub fn dock_notify(adev: *mut acpi_device, event: u32) -> c_int;
}
extern "C" {
    pub fn acpi_dock_add(adev: *mut acpi_device);
}

extern "C" {
    pub fn acpi_cmos_rtc_init();
}

extern "C" {
    pub fn acpi_rev_override_setup(str: *mut c_char) -> c_int;
}
extern "C" {
    pub fn acpi_scan_hotplug_enabled(hotplug: *mut acpi_hotplug_profile, val: bool);
}

extern "C" {
    pub fn acpi_debugfs_init();
}

extern "C" {
    pub fn acpi_lpss_init();
}

extern "C" {
    pub fn acpi_apd_init();
}
extern "C" {
    pub fn acpi_hotplug_schedule(adev: *mut acpi_device, src: u32) -> acpi_status;
}
extern "C" {
    pub fn acpi_queue_hotplug_work(work: *mut work_struct) -> bool;
}
extern "C" {
    pub fn acpi_device_hotplug(adev: *mut acpi_device, src: u32);
}
extern "C" {
    pub fn acpi_scan_is_offline(adev: *mut acpi_device, uevent: bool) -> bool;
}
extern "C" {
    pub fn acpi_sysfs_table_handler(event: u32, table: *mut c_void, context: *mut c_void) -> acpi_status;
}
extern "C" {
    pub fn acpi_scan_table_notify();
}
extern "C" {
    pub fn acpi_active_trip_temp(adev: *mut acpi_device, id: c_int, ret_temp: *mut c_int) -> c_int;
}
extern "C" {
    pub fn acpi_passive_trip_temp(adev: *mut acpi_device, ret_temp: *mut c_int) -> c_int;
}
extern "C" {
    pub fn acpi_hot_trip_temp(adev: *mut acpi_device, ret_temp: *mut c_int) -> c_int;
}
extern "C" {
    pub fn acpi_critical_trip_temp(adev: *mut acpi_device, ret_temp: *mut c_int) -> c_int;
}

extern "C" {
    pub fn acpi_arch_thermal_cpufreq_pctg() -> c_int;
}

// --------------------------------------------------------------------------

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_device_bus_id {
    pub bus_id: *const c_char,
    pub instance_ida: ida,
    pub node: list_head,
}

extern "C" {
    pub fn acpi_tie_acpi_dev(adev: *mut acpi_device) -> c_int;
}
extern "C" {
    pub fn acpi_device_add(device: *mut acpi_device) -> c_int;
}
extern "C" {
    pub fn acpi_device_setup_files(dev: *mut acpi_device);
}
extern "C" {
    pub fn acpi_device_remove_files(dev: *mut acpi_device);
}
extern "C" {
    pub fn acpi_device_add_finalize(device: *mut acpi_device);
}
extern "C" {
    pub fn acpi_free_pnp_ids(pnp: *mut acpi_device_pnp);
}
extern "C" {
    pub fn acpi_device_is_enabled(adev: *const acpi_device) -> bool;
}
extern "C" {
    pub fn acpi_device_is_present(adev: *const acpi_device) -> bool;
}
extern "C" {
    pub fn acpi_device_is_battery(adev: *mut acpi_device) -> bool;
}
extern "C" {
    pub fn acpi_bus_register_early_device(type: c_int) -> c_int;
}
// --------------------------------------------------------------------------
extern "C" {
    pub fn acpi_power_resources_init();
}
extern "C" {
    pub fn acpi_power_resources_list_free(list: *mut list_head);
}
extern "C" {
    pub fn acpi_power_add_remove_device(adev: *mut acpi_device, add: bool);
}
extern "C" {
    pub fn acpi_power_wakeup_list_init(list: *mut list_head, system_level: *mut c_int) -> c_int;
}
extern "C" {
    pub fn acpi_power_get_inferred_state(device: *mut acpi_device, state: *mut c_int) -> c_int;
}
extern "C" {
    pub fn acpi_power_on_resources(device: *mut acpi_device, state: c_int) -> c_int;
}
extern "C" {
    pub fn acpi_power_transition(device: *mut acpi_device, state: c_int) -> c_int;
}
extern "C" {
    pub fn acpi_turn_off_unused_power_resources();
}
// --------------------------------------------------------------------------
extern "C" {
    pub fn acpi_device_get_power(device: *mut acpi_device, state: *mut c_int) -> c_int;
}
extern "C" {
    pub fn acpi_wakeup_device_init() -> c_int;
}
// --------------------------------------------------------------------------

extern "C" {
    pub fn acpi_early_processor_control_setup();
}
extern "C" {
    pub fn acpi_early_processor_set_pdc();
}

extern "C" {
    pub fn acpi_proc_quirk_mwait_check();
}

extern "C" {
    pub fn processor_physically_present(handle: acpi_handle) -> bool;
}

extern "C" {
    pub fn acpi_idle_rescan_dead_smt_siblings();
}

// --------------------------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_ec_event_state {
    EC_EVENT_READY = 0,	/* Event work can be submitted */
    EC_EVENT_IN_PROGRESS,	/* Event work is pending or being processed */
    EC_EVENT_COMPLETE,	/* Event work processing has completed */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ec {
    pub handle: acpi_handle,
    pub gpe: c_int,
    pub irq: c_int,
    pub command_addr: c_ulong,
    pub data_addr: c_ulong,
    pub global_lock: bool,
    pub flags: c_ulong,
    pub reference_count: c_ulong,
    pub mutex: mutex,
    pub wait: wait_queue_head_t,
    pub list: list_head,
    pub curr: *mut transaction,
    pub lock: spinlock_t,
    pub work: work_struct,
    pub timestamp: c_ulong,
    pub event_state: acpi_ec_event_state,
    pub events_to_process: c_uint,
    pub events_in_progress: c_uint,
    pub queries_in_progress: c_uint,
    pub busy_polling: bool,
    pub polling_guard: c_uint,
}

// If we find an EC via the ECDT, we need to keep a ptr to its context
// External interfaces use first EC only, so remember
extern "C" {
    pub fn int(data: *mut *mut acpi_ec_query_func) (void) -> typedef;
}

extern "C" {
    pub fn acpi_ec_init();
}
extern "C" {
    pub fn acpi_ec_ecdt_probe();
}
extern "C" {
    pub fn acpi_ec_dsdt_probe();
}
extern "C" {
    pub fn acpi_ec_block_transactions();
}
extern "C" {
    pub fn acpi_ec_unblock_transactions();
}
extern "C" {
    pub fn acpi_ec_remove_query_handler(ec: *mut acpi_ec, query_bit: u8);
}
extern "C" {
    pub fn acpi_ec_register_opregions(adev: *mut acpi_device);
}

extern "C" {
    pub fn acpi_ec_flush_work();
}
extern "C" {
    pub fn acpi_ec_dispatch_gpe() -> bool;
}

// --------------------------------------------------------------------------

extern "C" {
    pub fn acpi_s2idle_wakeup() -> bool;
}
extern "C" {
    pub fn acpi_sleep_init() -> c_int;
}

extern "C" {
    pub fn acpi_sleep_proc_init();
}
extern "C" {
    pub fn suspend_nvs_alloc() -> c_int;
}
extern "C" {
    pub fn suspend_nvs_free();
}
extern "C" {
    pub fn suspend_nvs_save() -> c_int;
}
extern "C" {
    pub fn suspend_nvs_restore();
}

extern "C" {
    pub fn force_storage_d3() -> bool;
}

// --------------------------------------------------------------------------

extern "C" {
    pub fn acpi_init_properties(adev: *mut acpi_device);
}
extern "C" {
    pub fn acpi_free_properties(adev: *mut acpi_device);
}

extern "C" {
    pub fn acpi_extract_apple_properties(adev: *mut acpi_device);
}

// --------------------------------------------------------------------------

extern "C" {
    pub fn acpi_watchdog_init();
}

extern "C" {
    pub fn acpi_init_lpit();
}

// --------------------------------------------------------------------------
extern "C" {
    pub fn acpi_mipi_check_crs_csi2(handle: acpi_handle);
}
extern "C" {
    pub fn acpi_mipi_scan_crs_csi2();
}
extern "C" {
    pub fn acpi_mipi_init_crs_csi2_swnodes();
}
extern "C" {
    pub fn acpi_mipi_crs_csi2_cleanup();
}

extern "C" {
    pub fn acpi_graph_ignore_port(handle: acpi_handle) -> bool;
}

