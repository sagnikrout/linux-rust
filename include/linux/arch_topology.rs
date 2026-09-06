//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/arch_topology.h
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
// include/linux/arch_topology.h - arch specific cpu topology information
//

extern "C" {
    pub fn topology_normalize_cpu_scale();
}
extern "C" {
    pub fn topology_update_cpu_topology() -> c_int;
}
extern "C" {
    pub fn topology_parse_cpu_capacity(cpu_node: *mut device_node, cpu: c_int) -> bool;
}
extern "C" {
    pub fn per_cpu(_arg: capacity_freq_ref, _arg: cpu) -> return;
}
extern "C" {
    pub fn per_cpu(_arg: arch_freq_scale, _arg: cpu) -> return;
}
extern "C" {
    pub fn topology_scale_freq_invariant() -> bool;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scale_freq_source {
    SCALE_FREQ_SOURCE_CPUFREQ = 0,
    SCALE_FREQ_SOURCE_ARCH,
    SCALE_FREQ_SOURCE_CPPC,
    SCALE_FREQ_SOURCE_VIRT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scale_freq_data {
    pub source: scale_freq_source,
    pub (*set_freq_scale)(void): *mut c_void,
}

extern "C" {
    pub fn topology_scale_freq_tick();
}
extern "C" {
    pub fn topology_set_scale_freq_source(data: *mut scale_freq_data, cpus: *const cpumask);
}
extern "C" {
    pub fn topology_clear_scale_freq_source(source: scale_freq_source, cpus: *const cpumask);
}
extern "C" {
    pub fn per_cpu(_arg: hw_pressure, _arg: cpu) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_topology {
    pub thread_id: c_int,
    pub core_id: c_int,
    pub cluster_id: c_int,
    pub package_id: c_int,
    pub thread_sibling: cpumask_t,
    pub core_sibling: cpumask_t,
    pub cluster_sibling: cpumask_t,
    pub llc_sibling: cpumask_t,
}

extern "C" {
    pub fn init_cpu_topology();
}
extern "C" {
    pub fn store_cpu_topology(cpuid: c_uint);
}
extern "C" {
    pub fn update_siblings_masks(cpu: c_uint);
}
extern "C" {
    pub fn remove_cpu_topology(cpuid: c_uint);
}
extern "C" {
    pub fn reset_cpu_topology();
}
extern "C" {
    pub fn parse_acpi_topology() -> c_int;
}
extern "C" {
    pub fn freq_inv_set_max_ratio(cpu: c_int, max_rate: u64);
}
//
// Architectures like ARM64 don't have reliable architectural way to get SMT
// information and depend on the firmware (ACPI/OF) report. Non-SMT core won't
// initialize thread_id so we can use this to detect the SMT implementation.
//

