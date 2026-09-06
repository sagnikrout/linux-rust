//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/psci.h
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
// Copyright (C) 2015 ARM Limited
//

pub const PSCI_POWER_STATE_TYPE_STANDBY: c_int = 0;
pub const PSCI_POWER_STATE_TYPE_POWER_DOWN: c_int = 1;
extern "C" {
    pub fn psci_tos_resident_on(cpu: c_int) -> bool;
}
extern "C" {
    pub fn psci_cpu_suspend_enter(state: u32) -> c_int;
}
extern "C" {
    pub fn psci_power_state_is_valid(state: u32) -> bool;
}
extern "C" {
    pub fn psci_set_osi_mode(enable: bool) -> c_int;
}
extern "C" {
    pub fn psci_has_osi_support() -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psci_operations {
    pub (*get_version)(void): *mut u32,
    pub entry_point): *mut *mut int (cpu_suspend)(u32 state, unsigned long,
    pub state): *mut *mut int (cpu_off)(u32,
    pub entry_point): *mut *mut int (cpu_on)(unsigned long cpuid, unsigned long,
    pub cpuid): *mut *mut int (migrate)(unsigned long,
    pub lowest_affinity_level): c_ulong,
    pub (*migrate_info_type)(void): *mut c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psci_0_1_function_ids {
    pub cpu_suspend: u32,
    pub cpu_on: u32,
    pub cpu_off: u32,
    pub migrate: u32,
}

extern "C" {
    pub fn get_psci_0_1_function_ids() -> psci_0_1_function_ids;
}

extern "C" {
    pub fn psci_dt_init() -> int __init;
}

extern "C" {
    pub fn psci_acpi_init() -> int __init;
}
extern "C" {
    pub fn acpi_psci_present() -> bool __init;
}
extern "C" {
    pub fn acpi_psci_use_hvc() -> bool;
}

