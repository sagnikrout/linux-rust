//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/coredump.h
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
pub struct core_vma_metadata {
    pub end: unsigned long start,,
    pub flags: vm_flags_t,
    pub dump_size: c_ulong,
    pub pgoff: c_ulong,
    pub file: *mut file,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coredump_params {
    pub siginfo: *const kernel_siginfo_t,
    pub file: *mut file,
    pub limit: c_ulong,
// MMF_DUMP_FILTER_* bits, snapshot of mm->flags at dump start.
    pub mm_flags: c_ulong,
// Snapshot of dumpable at dump start.
    pub dumpable: task_dumpable,
    pub cpu: c_int,
    pub written: loff_t,
    pub pos: loff_t,
    pub to_skip: loff_t,
    pub vma_count: c_int,
    pub vma_data_size: usize,
    pub vma_meta: *mut core_vma_metadata,
    pub pid: *mut pid,
}

//
// These are the only things you should do on a core-file: use only these
// functions to write out all the necessary info.
//
extern "C" {
    pub fn dump_skip_to(cprm: *mut coredump_params, to: c_ulong);
}
extern "C" {
    pub fn dump_skip(cprm: *mut coredump_params, nr: usize);
}
extern "C" {
    pub fn dump_emit(cprm: *mut coredump_params, addr: *const c_void, nr: c_int) -> c_int;
}
extern "C" {
    pub fn dump_align(cprm: *mut coredump_params, align: c_int) -> c_int;
}
extern "C" {
    pub fn vfs_coredump(siginfo: *const kernel_siginfo_t);
}
//
// Logging for the coredump code, ratelimited.
// The TGID and comm fields are added to the message.
//

// This will always be NUL terminated. */ \

// Macro flag: #define coredump_report(...)
// Macro flag: #define coredump_report_failure(...)

extern "C" {
    pub fn validate_coredump_safety();
}

