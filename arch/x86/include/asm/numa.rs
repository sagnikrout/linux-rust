//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/numa.h
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
// __apicid_to_node[] stores the raw mapping between physical apicid and
// node and is used to initialize cpu_to_node mapping.
//
// The mapping may be overridden by apic->numa_cpu_node() on 32bit and thus
// should be accessed by the accessors - set_apicid_to_node() and
// numa_cpu_node().
//
extern "C" {
    pub fn numa_cpu_node(cpu: c_int) -> c_int;
}

extern "C" {
    pub fn numa_set_node(cpu: c_int, node: c_int);
}
extern "C" {
    pub fn numa_clear_node(cpu: c_int);
}
extern "C" {
    pub fn init_cpu_to_node() -> void __init;
}
extern "C" {
    pub fn numa_add_cpu(cpu: c_uint);
}
extern "C" {
    pub fn numa_remove_cpu(cpu: c_uint);
}
extern "C" {
    pub fn init_gi_nodes();
}
extern "C" {
    pub fn num_phys_nodes() -> c_int;
}

extern "C" {
    pub fn debug_cpumask_set_cpu(cpu: c_uint, node: c_int, enable: bool);
}

