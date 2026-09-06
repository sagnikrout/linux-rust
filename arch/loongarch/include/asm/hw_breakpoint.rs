//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/hw_breakpoint.h
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
// Copyright (C) 2022-2023 Loongson Technology Corporation Limited
//

// Breakpoint

// Watchpoints

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_hw_breakpoint_ctrl {
    pub 2: type :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_hw_breakpoint {
    pub address: u64,
    pub mask: u64,
    pub ctrl: arch_hw_breakpoint_ctrl,
}

// Lengths

//
// Limits.
// Changing these will require modifications to the register accessors.
//
pub const LOONGARCH_MAX_BRP: c_int = 14;
pub const LOONGARCH_MAX_WRP: c_int = 14;
// Virtual debug register bases.
pub const CSR_CFG_ADDR: c_int = 0;

// Debug register names.

// Accessor macros for the debug registers.

// Exact number
pub const CSR_FWPC_NUM: c_uint = 0x3f;
pub const CSR_MWPC_NUM: c_uint = 0x3f;
pub const CTRL_PLV_ENABLE: c_uint = 0x1e;
pub const CTRL_PLV0_ENABLE: c_uint = 0x02;
pub const CTRL_PLV3_ENABLE: c_uint = 0x10;
pub const MWPnCFG3_LoadEn: c_int = 8;
pub const MWPnCFG3_StoreEn: c_int = 9;
pub const MWPnCFG3_Type_mask: c_uint = 0x3;
pub const MWPnCFG3_Size_mask: c_uint = 0x3;
extern "C" {
    pub fn arch_check_bp_in_kernelspace(hw: *mut arch_hw_breakpoint) -> c_int;
}
extern "C" {
    pub fn arch_install_hw_breakpoint(bp: *mut perf_event) -> c_int;
}
extern "C" {
    pub fn arch_uninstall_hw_breakpoint(bp: *mut perf_event);
}
extern "C" {
    pub fn hw_breakpoint_slots(type: c_int) -> c_int;
}
extern "C" {
    pub fn hw_breakpoint_pmu_read(bp: *mut perf_event);
}
extern "C" {
    pub fn breakpoint_handler(regs: *mut pt_regs);
}
extern "C" {
    pub fn watchpoint_handler(regs: *mut pt_regs);
}

extern "C" {
    pub fn ptrace_hw_copy_thread(task: *mut task_struct);
}
extern "C" {
    pub fn hw_breakpoint_thread_switch(next: *mut task_struct);
}

// Determine number of BRP registers available.
// Determine number of WRP registers available.

