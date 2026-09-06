//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/imr.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// imr.h: Isolated Memory Region API
//
// Copyright(c) 2013 Intel Corporation.
// Copyright(c) 2015 Bryan O'Donoghue <pure.logic@nexus-software.ie>
//

//
// IMR agent access mask bits
// See section 12.7.4.7 from quark-x1000-datasheet.pdf for register
// definitions.
//

pub const IMR_ACCESS_NONE: c_int = 0;
//
// Read/Write access-all bits here include some reserved bits
// These are the values firmware uses and are accepted by hardware.
// The kernel defines read/write access-all in the same way as firmware
// in order to have a consistent and crisp definition across firmware,
// bootloader and kernel.
//
pub const IMR_READ_ACCESS_ALL: c_uint = 0xBFFFFFFF;
pub const IMR_WRITE_ACCESS_ALL: c_uint = 0xFFFFFFFF;
// Number of IMRs provided by Quark X1000 SoC
pub const QUARK_X1000_IMR_MAX: c_uint = 0x08;
pub const QUARK_X1000_IMR_REGBASE: c_uint = 0x40;
// IMR alignment bits - only bits 31:10 are checked for IMR validity
pub const IMR_ALIGN: c_uint = 0x400;

extern "C" {
    pub fn imr_remove_range(base: phys_addr_t, size: usize) -> c_int;
}
