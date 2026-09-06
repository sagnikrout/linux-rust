//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/systemcfg.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2002 Peter Bergner <bergner@vnet.ibm.com>, IBM
// Copyright (C) 2005 Benjamin Herrenschmidy <benh@kernel.crashing.org>,
// IBM Corp.
//

//
// If the major version changes we are incompatible.
// Minor version changes are a hint.
//
pub const SYSTEMCFG_MAJOR: c_int = 1;
pub const SYSTEMCFG_MINOR: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct systemcfg {
    pub /: *mut *mut __u8 eye_catcher[16]; / Eyecatcher: SYSTEMCFG:PPC64 0x00,
    pub /: *mut *mut __u32 major; / Major number 0x10,
    pub /: *mut *mut __u32 minor; / Minor number 0x14,
    pub version: },
// Note about the platform flags: it now only contains the lpar
// bit. The actual platform number is dead and buried
//
    pub /: *mut *mut __u32 platform; / Platform flags 0x18,
    pub /: *mut *mut __u32 processor; / Processor type 0x1C,
    pub /: *mut *mut __u64 processorCount; / # of physical processors 0x20,
    pub /: *mut *mut __u64 physicalMemorySize; / Size of real memory(B) 0x28,
    pub /: *mut *mut __u64 tb_orig_stamp; / (NU) Timebase at boot 0x30,
    pub /: *mut *mut __u64 tb_ticks_per_sec; / Timebase tics / sec 0x38,
    pub /: *mut *mut __u64 tb_to_xs; / (NU) Inverse of TB to 2^20 0x40,
    pub /: *mut *mut __u64 stamp_xsec; / (NU) 0x48,
    pub /: *mut *mut __u64 tb_update_count; / (NU) Timebase atomicity ctr 0x50,
    pub /: *mut *mut __u32 tz_minuteswest; / (NU) Min. west of Greenwich 0x58,
    pub /: *mut *mut __u32 tz_dsttime; / (NU) Type of dst correction 0x5C,
    pub /: *mut *mut __u32 dcache_size; / L1 d-cache size 0x60,
    pub /: *mut *mut __u32 dcache_line_size; / L1 d-cache line size 0x64,
    pub /: *mut *mut __u32 icache_size; / L1 i-cache size 0x68,
    pub /: *mut *mut __u32 icache_line_size; / L1 i-cache line size 0x6C,
}

