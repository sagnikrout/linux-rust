//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/olpc_ofw.h
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
// index into the page table containing the entry OFW occupies
pub const OLPC_OFW_PDE_NR: c_int = 1022;
pub const OLPC_OFW_SIG: c_uint = 0x2057464F	/* aka "OFW " */;

extern "C" {
    pub fn olpc_ofw_is_installed() -> bool;
}
// run an OFW command by calling into the firmware

// determine whether OFW is available and lives in the proper memory
extern "C" {
    pub fn olpc_ofw_detect();
}
// install OFW's pde permanently into the kernel's pgtable
extern "C" {
    pub fn setup_olpc_ofw_pgd();
}
// check if OFW was detected during boot
extern "C" {
    pub fn olpc_ofw_present() -> bool;
}
extern "C" {
    pub fn olpc_dt_build_devicetree();
}

