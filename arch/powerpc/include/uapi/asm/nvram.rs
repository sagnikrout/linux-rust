//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/nvram.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// NVRAM definitions and access functions.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//
// Signatures for nvram partitions
pub const NVRAM_SIG_SP: c_uint = 0x02	/* support processor */;
pub const NVRAM_SIG_OF: c_uint = 0x50	/* open firmware config */;
pub const NVRAM_SIG_FW: c_uint = 0x51	/* general firmware */;
pub const NVRAM_SIG_HW: c_uint = 0x52	/* hardware (VPD) */;
pub const NVRAM_SIG_FLIP: c_uint = 0x5a	/* Apple flip/flop header */;
pub const NVRAM_SIG_APPL: c_uint = 0x5f	/* Apple "system" (???) */;
pub const NVRAM_SIG_SYS: c_uint = 0x70	/* system env vars */;
pub const NVRAM_SIG_CFG: c_uint = 0x71	/* config data */;
pub const NVRAM_SIG_ELOG: c_uint = 0x72	/* error log */;
pub const NVRAM_SIG_VEND: c_uint = 0x7e	/* vendor defined */;
pub const NVRAM_SIG_FREE: c_uint = 0x7f	/* Free space */;
pub const NVRAM_SIG_OS: c_uint = 0xa0	/* OS defined */;
pub const NVRAM_SIG_PANIC: c_uint = 0xa1	/* Apple OSX "panic" */;
// PowerMac specific nvram stuffs
// Some offsets in XPRAM
pub const PMAC_XPRAM_MACHINE_LOC: c_uint = 0xe4;
pub const PMAC_XPRAM_SOUND_VOLUME: c_uint = 0x08;
// Machine location structure in PowerMac XPRAM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmac_machine_location {
    pub /: *mut *mut unsigned int latitude; / 2+30 bit Fractional number,
    pub /: *mut *mut unsigned int longitude; / 2+30 bit Fractional number,
    pub /: *mut *mut unsigned int delta; / mix of GMT delta and DLS,
}

//
// /dev/nvram ioctls
//
// Note that PMAC_NVRAM_GET_OFFSET is still supported, but is
// definitely obsolete. Do not use it if you can avoid it
//

