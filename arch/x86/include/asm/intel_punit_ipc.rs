//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/intel_punit_ipc.h
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
// Three types of 8bit P-Unit IPC commands are supported,
// bit[7:6]: [00]: BIOS; [01]: GTD; [10]: ISPD.
//
pub const IPC_TYPE_OFFSET: c_int = 6;

// BIOS => Pcode commands

// GT Driver => Pcode commands

// ISP Driver => Pcode commands

// Error codes
pub const IPC_PUNIT_ERR_SUCCESS: c_int = 0;
pub const IPC_PUNIT_ERR_INVALID_CMD: c_int = 1;
pub const IPC_PUNIT_ERR_INVALID_PARAMETER: c_int = 2;
pub const IPC_PUNIT_ERR_CMD_TIMEOUT: c_int = 3;
pub const IPC_PUNIT_ERR_CMD_LOCKED: c_int = 4;
pub const IPC_PUNIT_ERR_INVALID_VR_ID: c_int = 5;
pub const IPC_PUNIT_ERR_VR_ERR: c_int = 6;

extern "C" {
    pub fn intel_punit_ipc_command(cmd: u32, para1: u32, para2: u32, in: *mut u32, out: *mut u32) -> c_int;
}

