//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/kdump.h
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

pub const KDUMP_KERNELBASE: c_uint = 0x2000000;
// How many bytes to reserve at zero for kdump. The reserve limit should
// be greater or equal to the trampoline's end address.
// Reserve to the end of the FWNMI area, see head_64.S
pub const KDUMP_RESERVE_LIMIT: c_uint = 0x10000 /* 64K */;

//
// On PPC64 translation is disabled during trampoline setup, so we use
// physical addresses. Though on PPC32 translation is already enabled,
// so we can't do the same. Luckily create_trampoline() creates relative
// branches, so we can just add the PAGE_OFFSET and don't worry about it.
//

pub const KDUMP_TRAMPOLINE_START: c_uint = 0x0100;
pub const KDUMP_TRAMPOLINE_END: c_uint = 0x3000;

pub const KDUMP_MIN_TCE_ENTRIES: c_int = 2048;

extern "C" {
    pub fn reserve_kdump_trampoline();
}
extern "C" {
    pub fn setup_kdump_trampoline();
}

// !CRASH_DUMP || !NONSTATIC_KERNEL

