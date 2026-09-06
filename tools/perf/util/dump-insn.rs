//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/dump-insn.h
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
pub const __PERF_DUMP_INSN_H: c_int = 1;
pub const MAXINSN: c_int = 15;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_insn {
// Initialized by callers:
    pub thread: *mut thread,
    pub machine: *mut machine,
    pub cpumode: u8,
    pub is64bit: bool,
    pub cpu: c_int,
// Temporary
    pub out: [c_char; 256],
}

extern "C" {
    pub fn arch_is_uncond_branch(buf: *const c_uchar, len: usize, x86_64: c_int) -> c_int;
}
