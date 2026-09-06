//! Automatically rewritten from C Header to Rust Module
//! Source: tools/power/cpupower/lib/powercap.h
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
// (C) 2016 SUSE Software Solutions GmbH
// Thomas Renninger <trenn@suse.de>
//

pub const POWERCAP_MAX_CHILD_ZONES: c_int = 10;
pub const POWERCAP_MAX_TREE_DEPTH: c_int = 10;
pub const MAX_LINE_LEN: c_int = 4096;
pub const SYSFS_PATH_MAX: c_int = 255;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct powercap_zone {
    pub name: [c_char; MAX_LINE_LEN],
//
// sys_name relative to PATH_TO_POWERCAP,
// do not forget the / in between
//
    pub sys_name: [c_char; SYSFS_PATH_MAX],
    pub tree_depth: c_int,
    pub parent: *mut powercap_zone,
    pub children: [*mut powercap_zone; POWERCAP_MAX_CHILD_ZONES],
// More possible caps or attributes to be added?
}

extern "C" {
    pub fn powercap_get_enabled(mode: *mut c_int) -> c_int;
}
extern "C" {
    pub fn powercap_set_enabled(mode: c_int) -> c_int;
}
extern "C" {
    pub fn powercap_get_driver(driver: *mut c_char, buflen: c_int) -> c_int;
}
extern "C" {
    pub fn powercap_get_max_energy_range_uj(zone: *mut powercap_zone, val: *mut u64) -> c_int;
}
extern "C" {
    pub fn powercap_get_energy_uj(zone: *mut powercap_zone, val: *mut u64) -> c_int;
}
extern "C" {
    pub fn powercap_get_max_power_range_uw(zone: *mut powercap_zone, val: *mut u64) -> c_int;
}
extern "C" {
    pub fn powercap_get_power_uw(zone: *mut powercap_zone, val: *mut u64) -> c_int;
}
extern "C" {
    pub fn powercap_zone_get_enabled(zone: *mut powercap_zone, mode: *mut c_int) -> c_int;
}
extern "C" {
    pub fn powercap_zone_set_enabled(zone: *mut powercap_zone, mode: c_int) -> c_int;
}
