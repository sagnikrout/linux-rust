//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/topology.h
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
// If zone_reclaim_mode is enabled, a RECLAIM_DISTANCE of 10 will mean that
// all zones on all nodes will be eligible for zone_reclaim().
//
pub const RECLAIM_DISTANCE: c_int = 10;

extern "C" {
    pub fn pcibus_to_node(bus: *mut pci_bus) -> c_int;
}

extern "C" {
    pub fn cpu_relative_distance(cpu1_assoc: *mut __be32, cpu2_assoc: *mut __be32) -> c_int;
}
extern "C" {
    pub fn __node_distance(_arg: c_int, _arg: c_int) -> c_int;
}

extern "C" {
    pub fn dump_numa_cpu_topology() -> void __init;
}
extern "C" {
    pub fn sysfs_add_device_to_node(dev: *mut device, nid: c_int) -> c_int;
}
extern "C" {
    pub fn sysfs_remove_device_from_node(dev: *mut device, nid: c_int);
}
//
// Fall back to node 0 if nid is unset (it should be, except bugs).
// This allows callers to safely do NODE_DATA(early_cpu_to_node(cpu)).
//
extern "C" {
    pub fn of_drconf_to_nid_single(lmb: *mut drmem_lmb) -> c_int;
}
extern "C" {
    pub fn update_numa_distance(node: *mut device_node);
}
extern "C" {
    pub fn map_cpu_to_node(cpu: c_int, node: c_int);
}

extern "C" {
    pub fn unmap_cpu_from_node(cpu: c_ulong);
}

extern "C" {
    pub fn find_and_update_cpu_nid(cpu: c_int);
}
extern "C" {
    pub fn cpu_to_coregroup_id(cpu: c_int) -> c_int;
}

extern "C" {
    pub fn cpu_to_core_id(_arg: cpu) -> return;
}

extern "C" {
    pub fn cpu_die_id(cpu: c_int) -> c_int;
}
//
// Points to where the LLC is. On power9 this will point at CACHE
// domain, On others it will point to SMT domain. In all cases
// cpu_l2_cache_mask points to where LLC is
//

