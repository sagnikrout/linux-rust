//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/edac/edac_module.h
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


// SPDX-License-Identifier: GPL-2.0
//
// edac_module.h
//
// For defining functions/data for within the EDAC_CORE module only
//
// written by doug thompson <norsk5@xmission.h>
//

//
// INTERNAL EDAC MODULE:
// EDAC memory controller sysfs create/remove functions
// and setup/teardown functions
//
// edac_mc objects
//
// on edac_mc_sysfs.c
extern "C" {
    pub fn edac_mc_sysfs_init() -> c_int;
}
extern "C" {
    pub fn edac_mc_sysfs_exit();
}
extern "C" {
    pub fn edac_remove_sysfs_mci_device(mci: *mut mem_ctl_info);
}
extern "C" {
    pub fn edac_mc_get_log_ue() -> c_int;
}
extern "C" {
    pub fn edac_mc_get_log_ce() -> c_int;
}
extern "C" {
    pub fn edac_mc_get_panic_on_ue() -> c_int;
}
extern "C" {
    pub fn edac_mc_get_poll_msec() -> c_uint;
}
// on edac_device.c
extern "C" {
    pub fn edac_device_create_sysfs(edac_dev: *mut edac_device_ctl_info) -> c_int;
}
extern "C" {
    pub fn edac_device_remove_sysfs(edac_dev: *mut edac_device_ctl_info);
}
// edac core workqueue: single CPU mode
extern "C" {
    pub fn edac_workqueue_setup() -> c_int;
}
extern "C" {
    pub fn edac_workqueue_teardown();
}
extern "C" {
    pub fn edac_queue_work(work: *mut delayed_work, delay: c_ulong) -> bool;
}
extern "C" {
    pub fn edac_stop_work(work: *mut delayed_work) -> bool;
}
extern "C" {
    pub fn edac_mod_work(work: *mut delayed_work, delay: c_ulong) -> bool;
}
extern "C" {
    pub fn edac_device_reset_delay_period(edac_dev: *mut edac_device_ctl_info, msec: c_uint);
}
extern "C" {
    pub fn edac_mc_reset_delay_period(value: c_ulong);
}
//
// EDAC debugfs functions
//

extern "C" {
    pub fn edac_debugfs_init();
}
extern "C" {
    pub fn edac_debugfs_exit();
}
extern "C" {
    pub fn edac_create_debugfs_nodes(mci: *mut mem_ctl_info);
}

//
// EDAC PCI functions
//

extern "C" {
    pub fn edac_pci_do_parity_check();
}
extern "C" {
    pub fn edac_pci_clear_parity_errors();
}
extern "C" {
    pub fn edac_sysfs_pci_setup() -> c_int;
}
extern "C" {
    pub fn edac_sysfs_pci_teardown();
}
extern "C" {
    pub fn edac_pci_get_check_errors() -> c_int;
}
extern "C" {
    pub fn edac_pci_get_poll_msec() -> c_int;
}
extern "C" {
    pub fn edac_pci_remove_sysfs(pci: *mut edac_pci_ctl_info);
}
extern "C" {
    pub fn edac_pci_handle_pe(pci: *mut edac_pci_ctl_info, msg: *const c_char);
}

// pre-process these away
// Macro flag: #define edac_pci_do_parity_check()
// Macro flag: #define edac_pci_clear_parity_errors()

// Macro flag: #define edac_sysfs_pci_teardown()
// Macro flag: #define edac_pci_get_check_errors()
// Macro flag: #define edac_pci_get_poll_msec()
// Macro flag: #define edac_pci_handle_pe()
// Macro flag: #define edac_pci_handle_npe()

