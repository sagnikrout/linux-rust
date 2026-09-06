//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/module.h
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
// This file contains the s390 architecture specific module code.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_arch_syminfo {
    pub got_offset: c_ulong,
    pub plt_offset: c_ulong,
    pub got_initialized: c_int,
    pub plt_initialized: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mod_arch_specific {
// Starting offset of got in the module core memory.
    pub got_offset: c_ulong,
// Starting offset of plt in the module core memory.
    pub plt_offset: c_ulong,
// Size of the got.
    pub got_size: c_ulong,
// Size of the plt.
    pub plt_size: c_ulong,
// Number of symbols in syminfo.
    pub nsyms: c_int,
// Additional symbol information (got and plt offsets).
    pub syminfo: *mut mod_arch_syminfo,

// Start of memory reserved for ftrace hotpatch trampolines.
    pub trampolines_start: *mut ftrace_hotpatch_trampoline,
// End of memory reserved for ftrace hotpatch trampolines.
    pub trampolines_end: *mut ftrace_hotpatch_trampoline,
// Next unused ftrace hotpatch trampoline slot.
    pub next_trampoline: *mut ftrace_hotpatch_trampoline,

}
