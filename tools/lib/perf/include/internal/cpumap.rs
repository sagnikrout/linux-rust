//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/perf/include/internal/cpumap.h
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
// A sized, reference counted, sorted array of integers representing CPU
// numbers. This is commonly used to capture which CPUs a PMU is associated
// with. The indices into the cpumap are frequently used as they avoid having
// gaps if CPU numbers were used. For events associated with a pid, rather than
// a CPU, a single dummy map with an entry of -1 is used.
//
// Length of the map array.
// The CPU values.
extern "C" {
    pub fn perf_cpu_map__idx(cpus: *const perf_cpu_map, cpu: perf_cpu) -> c_int;
}
extern "C" {
    pub fn perf_cpu_map__is_subset(a: *const perf_cpu_map, b: *const perf_cpu_map) -> bool;
}
extern "C" {
    pub fn perf_cpu_map__set_nr(map: *mut perf_cpu_map, nr_cpus: c_uint);
}
