//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/sysinfo.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

pub const SI_LOAD_SHIFT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysinfo {
    pub /: *mut *mut __kernel_long_t uptime; / Seconds since boot,
    pub /: *mut *mut __kernel_ulong_t loads[3]; / 1, 5, and 15 minute load averages,
    pub /: *mut *mut __kernel_ulong_t totalram; / Total usable main memory size,
    pub /: *mut *mut __kernel_ulong_t freeram; / Available memory size,
    pub /: *mut *mut __kernel_ulong_t sharedram; / Amount of shared memory,
    pub /: *mut *mut __kernel_ulong_t bufferram; / Memory used by buffers,
    pub /: *mut *mut __kernel_ulong_t totalswap; / Total swap space size,
    pub /: *mut *mut __kernel_ulong_t freeswap; / swap space still available,
    pub /: *mut *mut __u16 procs; / Number of current processes,
    pub /: *mut *mut __u16 pad; / Explicit padding for m68k,
    pub /: *mut *mut __kernel_ulong_t totalhigh; / Total high memory size,
    pub /: *mut *mut __kernel_ulong_t freehigh; / Available high memory size,
    pub /: *mut *mut __u32 mem_unit; / Memory unit size in bytes,
    pub /: *mut *mut *mut char _f[20-2sizeof(__kernel_ulong_t)-sizeof(__u32)]; / Padding: libc5 uses this..,
}
