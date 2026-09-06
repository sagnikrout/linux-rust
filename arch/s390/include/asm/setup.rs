//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/setup.h
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
// S390 version
// Copyright IBM Corp. 1999, 2017
//

pub const PARMAREA: c_uint = 0x10400;

// Offsets to entry points in kernel/head.S
pub const STARTUP_NORMAL_OFFSET: c_uint = 0x10000;
pub const STARTUP_KDUMP_OFFSET: c_uint = 0x10010;
pub const LEGACY_COMMAND_LINE_SIZE: c_int = 896;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct parmarea {
    pub /: *mut *mut unsigned long ipl_device; / 0x10400,
    pub /: *mut *mut unsigned long initrd_start; / 0x10408,
    pub /: *mut *mut unsigned long initrd_size; / 0x10410,
    pub /: *mut *mut unsigned long oldmem_base; / 0x10418,
    pub /: *mut *mut unsigned long oldmem_size; / 0x10420,
    pub /: *mut *mut unsigned long kernel_version; / 0x10428,
    pub /: *mut *mut unsigned long max_command_line_size; / 0x10430,
    pub /: *mut *mut char pad1[0x10480-0x10438]; / 0x10438 - 0x10480,
    pub /: *mut *mut char command_line[COMMAND_LINE_SIZE]; / 0x10480,
}

pub const ZLIB_DFLTCC_DISABLED: c_int = 0;
pub const ZLIB_DFLTCC_FULL: c_int = 1;
pub const ZLIB_DFLTCC_DEFLATE_ONLY: c_int = 2;
pub const ZLIB_DFLTCC_INFLATE_ONLY: c_int = 3;
pub const ZLIB_DFLTCC_FULL_DEBUG: c_int = 4;
// The Write Back bit position in the physaddr is given by the SLPC PCI
//
// Console mode. Override with conmode=
//

extern "C" {
    pub fn register_early_console();
}

extern "C" {
    pub fn vmcp_cma_reserve();
}

extern "C" {
    pub fn report_user_fault(regs: *mut pt_regs, signr: c_long, is_mm_fault: c_int);
}
extern "C" {
    pub fn void(command: *mut *mut _machine_restart)(char) -> extern;
}
extern "C" {
    pub fn void(_arg: *mut _machine_halt)(void) -> extern;
}
extern "C" {
    pub fn void(_arg: *mut _machine_power_off)(void) -> extern;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct oldmem_data {
    pub start: c_ulong,
    pub size: c_ulong,
}

