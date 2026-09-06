//! Automatically rewritten from C Header to Rust Module
//! Source: tools/power/cpupower/lib/cpupower.h
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
pub const CPULIST_BUFFER: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpupower_topology {
// Amount of CPU cores, packages and threads per core in the system
    pub cores: c_uint,
    pub pkgs: c_uint,
    pub /: *mut *mut unsigned int threads; / per core,
// Array gets mallocated with cores entries, holding per core info
    pub core_info: *mut cpuid_core_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpuid_core_info {
    pub pkg: c_int,
    pub core: c_int,
    pub cpu: c_int,
    pub core_cpu_list: [c_char; CPULIST_BUFFER],
// flags
    pub is_online:1: c_uint,
}

extern "C" {
    pub fn get_cpu_topology(cpu_top: *mut cpupower_topology) -> c_int;
}
extern "C" {
    pub fn cpu_topology_release(cpu_top: cpupower_topology);
}
extern "C" {
    pub fn cpupower_is_cpu_online(cpu: c_uint) -> c_int;
}

