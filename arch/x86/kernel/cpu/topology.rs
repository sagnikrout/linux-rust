//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kernel/cpu/topology.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct topo_scan {
    pub c: *mut cpuinfo_x86,
    pub dom_shifts: [c_uint; TOPO_MAX_DOMAIN],
    pub dom_ncpus: [c_uint; TOPO_MAX_DOMAIN],
// Legacy CPUID[1]:EBX[23:16] number of logical processors
    pub ebx1_nproc_shift: c_uint,
// AMD specific node ID which cannot be mapped into APIC space.
    pub amd_nodes_per_pkg: u16,
    pub amd_node_id: u16,
}

extern "C" {
    pub fn cpu_init_topology(c: *mut cpuinfo_x86);
}
extern "C" {
    pub fn cpu_parse_topology(c: *mut cpuinfo_x86);
}
extern "C" {
    pub fn cpu_parse_topology_ext(tscan: *mut topo_scan) -> bool;
}
extern "C" {
    pub fn cpu_parse_topology_amd(tscan: *mut topo_scan);
}
extern "C" {
    pub fn cpu_topology_fixup_amd(tscan: *mut topo_scan);
}
extern "C" {
    pub fn get_topology_cpu_type(c: *mut cpuinfo_x86) -> x86_topology_cpu_type;
}
//
// Update a domain level after the fact without propagating. Used to fixup
// broken CPUID enumerations.
//

