//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/target.h
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
pub struct target {
    pub pid: *const c_char,
    pub tid: *const c_char,
    pub cpu_list: *const c_char,
    pub bpf_str: *const c_char,
    pub system_wide: bool,
    pub uses_mmap: bool,
    pub default_per_cpu: bool,
    pub per_thread: bool,
    pub use_bpf: bool,
    pub inherit: bool,
    pub initial_delay: c_int,
    pub attr_map: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum target_errno {
    TARGET_ERRNO__SUCCESS		= 0,

//
// Choose an arbitrary negative big number not to clash with standard
// errno since SUS requires the errno has distinct positive values.
// See 'Issue 6' in the link below.
//
// http://pubs.opengroup.org/onlinepubs/9699919799/basedefs/errno.h.html
//
    __TARGET_ERRNO__START		= -10000,

// for target__validate()
    TARGET_ERRNO__PID_OVERRIDE_CPU	= __TARGET_ERRNO__START,
    TARGET_ERRNO__PID_OVERRIDE_SYSTEM,
    TARGET_ERRNO__SYSTEM_OVERRIDE_THREAD,
    TARGET_ERRNO__BPF_OVERRIDE_CPU,
    TARGET_ERRNO__BPF_OVERRIDE_PID,
    TARGET_ERRNO__BPF_OVERRIDE_THREAD,

    __TARGET_ERRNO__END,
}

extern "C" {
    pub fn target__validate(target: *mut target) -> target_errno;
}
extern "C" {
    pub fn parse_uid(str: *const c_char) -> uid_t;
}
extern "C" {
    pub fn target__strerror(target: *mut target, errnum: c_int, buf: *mut c_char, buflen: usize) -> c_int;
}
//
// Normally enable_on_exec should be set if:
// 1) The tracee process is forked (not attaching to existed task or cpu).
// 2) And initial_delay is not configured.
// Otherwise, we enable tracee events manually.
//
