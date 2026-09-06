//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ras.h
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

extern "C" {
    pub fn ras_userspace_consumers() -> c_int;
}
extern "C" {
    pub fn ras_debugfs_init();
}
extern "C" {
    pub fn ras_add_daemon_trace() -> c_int;
}

extern "C" {
    pub fn parse_cec_param(str: *mut c_char) -> int __init;
}

extern "C" {
    pub fn log_arm_hw_error(err: *mut cper_sec_proc_arm, sev: u8);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atl_err {
    pub addr: u64,
    pub ipid: u64,
    pub cpu: u32,
}

extern "C" {
    pub fn amd_atl_register_decoder(): *mut *mut unsigned long (f)(struct atl_err);
}
extern "C" {
    pub fn amd_atl_unregister_decoder();
}
extern "C" {
    pub fn amd_retire_dram_row(err: *mut atl_err);
}
extern "C" {
    pub fn amd_convert_umc_mca_addr_to_sys_addr(err: *mut atl_err) -> c_ulong;
}

//
// Include ARM-specific SMP header which provides a function mapping mpidr to
// CPU logical index.
//

