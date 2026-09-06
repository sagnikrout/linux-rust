//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/pc-conf-reg.h
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
// Support for the configuration register space at port I/O locations
// 0x22 and 0x23 variously used by PC architectures, e.g. the MP Spec,
// Cyrix CPUs, numerous chipsets.
//

pub const PC_CONF_INDEX: c_uint = 0x22;
pub const PC_CONF_DATA: c_uint = 0x23;
pub const PC_CONF_MPS_IMCR: c_uint = 0x70;
extern "C" {
    pub fn inb(_arg: PC_CONF_DATA) -> return;
}
