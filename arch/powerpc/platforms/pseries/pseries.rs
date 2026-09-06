//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/platforms/pseries/pseries.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2006 IBM Corporation.
//

extern "C" {
    pub fn pSeries_system_reset_exception(regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn pSeries_machine_check_exception(regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn pseries_machine_check_realmode(regs: *mut pt_regs) -> c_long;
}
extern "C" {
    pub fn pSeries_machine_check_log_err();
}

extern "C" {
    pub fn smp_init_pseries();
}
// Get state of physical CPU from query_cpu_stopped
extern "C" {
    pub fn smp_query_cpu_stopped(pcpu: c_uint) -> c_int;
}
pub const QCSS_STOPPED: c_int = 0;
pub const QCSS_STOPPING: c_int = 1;
pub const QCSS_NOT_STOPPED: c_int = 2;

extern "C" {
    pub fn pseries_kexec_cpu_down(crash_shutdown: c_int, secondary: c_int);
}
extern "C" {
    pub fn pSeries_final_fixup();
}
// Poweron flag used for enabling auto ups restart
// Dynamic logical Partitioning/Mobility
extern "C" {
    pub fn dlpar_free_cc_nodes(: *mut device_node);
}
extern "C" {
    pub fn dlpar_free_cc_property(: *mut property);
}
extern "C" {
    pub fn dlpar_attach_node(: *mut device_node, : *mut device_node) -> c_int;
}
extern "C" {
    pub fn dlpar_detach_node(: *mut device_node) -> c_int;
}
extern "C" {
    pub fn dlpar_acquire_drc(drc_index: u32) -> c_int;
}
extern "C" {
    pub fn dlpar_release_drc(drc_index: u32) -> c_int;
}
extern "C" {
    pub fn dlpar_unisolate_drc(drc_index: u32) -> c_int;
}
extern "C" {
    pub fn post_mobility_fixup();
}
extern "C" {
    pub fn queue_hotplug_event(hp_errlog: *mut pseries_hp_errorlog);
}
extern "C" {
    pub fn handle_dlpar_errorlog(hp_errlog: *mut pseries_hp_errorlog) -> c_int;
}

extern "C" {
    pub fn dlpar_memory(hp_elog: *mut pseries_hp_errorlog) -> c_int;
}
extern "C" {
    pub fn dlpar_hp_pmem(hp_elog: *mut pseries_hp_errorlog) -> c_int;
}

extern "C" {
    pub fn dlpar_cpu(hp_elog: *mut pseries_hp_errorlog) -> c_int;
}
extern "C" {
    pub fn pseries_cpu_hotplug_init();
}

// PCI root bridge prepare function override for pseries
extern "C" {
    pub fn pseries_root_bridge_prepare(bridge: *mut pci_host_bridge) -> c_int;
}
extern "C" {
    pub fn pseries_msi_allocate_domains(phb: *mut pci_controller) -> c_int;
}
extern "C" {
    pub fn pseries_msi_free_domains(phb: *mut pci_controller);
}
extern "C" {
    pub fn dlpar_workqueue_init() -> c_int;
}
extern "C" {
    pub fn pseries_setup_security_mitigations();
}

extern "C" {
    pub fn pseries_lpar_read_hblkrm_characteristics();
}

extern "C" {
    pub fn pseries_rng_init();
}

