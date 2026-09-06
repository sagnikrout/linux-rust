//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/bpf_counter.h
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
pub const __PERF_BPF_COUNTER_H: c_int = 1;

extern "C" {
    pub fn int(evsel: *mut *mut bpf_counter_evsel_op)(struct evsel) -> typedef;
}
// Shared ops between bpf_counter, bpf_counter_cgroup, etc.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_counter_ops {
    pub load: bpf_counter_evsel_target_op,
    pub enable: bpf_counter_evsel_op,
    pub disable: bpf_counter_evsel_op,
    pub read: bpf_counter_evsel_op,
    pub destroy: bpf_counter_evsel_op,
    pub install_pe: bpf_counter_evsel_install_pe_op,
}

extern "C" {
    pub fn bpf_counter__load(evsel: *mut evsel, target: *mut target) -> c_int;
}
extern "C" {
    pub fn bpf_counter__enable(evsel: *mut evsel) -> c_int;
}
extern "C" {
    pub fn bpf_counter__disable(evsel: *mut evsel) -> c_int;
}
extern "C" {
    pub fn bpf_counter__read(evsel: *mut evsel) -> c_int;
}
extern "C" {
    pub fn bpf_counter__destroy(evsel: *mut evsel);
}
extern "C" {
    pub fn bpf_counter__install_pe(evsel: *mut evsel, cpu_map_idx: c_int, fd: c_int) -> c_int;
}
extern "C" {
    pub fn bperf_trigger_reading(prog_fd: c_int, cpu: c_int) -> c_int;
}
extern "C" {
    pub fn set_max_rlimit();
}

