//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/cputopo.h
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
pub struct cpu_topology {
// The number of unique package_cpus_lists below.
    pub package_cpus_lists: u32,
// The number of unique die_cpu_lists below.
    pub die_cpus_lists: u32,
// The number of unique core_cpu_lists below.
    pub core_cpus_lists: u32,
//
// An array of strings where each string is unique and read from
// /sys/devices/system/cpu/cpuX/topology/package_cpus_list. From the ABI
// each of these is a human-readable list of CPUs sharing the same
// physical_package_id. The format is like 0-3, 8-11, 14,17.
//
    pub package_cpus_list: *const c_char,
//
// An array of string where each string is unique and from
// /sys/devices/system/cpu/cpuX/topology/die_cpus_list. From the ABI
// each of these is a human-readable list of CPUs within the same die.
// The format is like 0-3, 8-11, 14,17.
//
    pub die_cpus_list: *const c_char,
//
// An array of string where each string is unique and from
// /sys/devices/system/cpu/cpuX/topology/core_cpus_list. From the ABI
// each of these is a human-readable list of CPUs within the same
// core. The format is like 0-3, 8-11, 14,17.
//
    pub core_cpus_list: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct numa_topology_node {
    pub cpus: *mut c_char,
    pub node: u32,
    pub mem_total: u64,
    pub mem_free: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct numa_topology {
    pub nr: u32,
    pub nodes: [numa_topology_node; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hybrid_topology_node {
    pub pmu_name: *mut c_char,
    pub cpus: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hybrid_topology {
    pub nr: u32,
    pub nodes: [hybrid_topology_node; ],
}

//
// The topology for online CPUs, lazily created.
//
extern "C" {
    pub fn cpu_topology__delete(tp: *mut cpu_topology);
}
// Determine from the core list whether SMT was enabled.
extern "C" {
    pub fn cpu_topology__smt_on(topology: *const cpu_topology) -> bool;
}
// Are the sets of SMT siblings all enabled or all disabled in user_requested_cpus.
extern "C" {
    pub fn numa_topology__delete(tp: *mut numa_topology);
}
extern "C" {
    pub fn hybrid_topology__delete(tp: *mut hybrid_topology);
}
