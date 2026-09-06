//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/hw_breakpoint.h
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
// Copyright (C) 2012 ARM Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_hw_breakpoint_ctrl {
    pub 1: enabled :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_hw_breakpoint {
    pub address: u64,
    pub trigger: u64,
    pub ctrl: arch_hw_breakpoint_ctrl,
}

// Privilege Levels
pub const AARCH64_BREAKPOINT_EL1: c_int = 1;
pub const AARCH64_BREAKPOINT_EL0: c_int = 2;

// Breakpoint
pub const ARM_BREAKPOINT_EXECUTE: c_int = 0;
// Watchpoints
pub const ARM_BREAKPOINT_LOAD: c_int = 1;
pub const ARM_BREAKPOINT_STORE: c_int = 2;
// Lengths
pub const ARM_BREAKPOINT_LEN_1: c_uint = 0x1;
pub const ARM_BREAKPOINT_LEN_2: c_uint = 0x3;
pub const ARM_BREAKPOINT_LEN_3: c_uint = 0x7;
pub const ARM_BREAKPOINT_LEN_4: c_uint = 0xf;
pub const ARM_BREAKPOINT_LEN_5: c_uint = 0x1f;
pub const ARM_BREAKPOINT_LEN_6: c_uint = 0x3f;
pub const ARM_BREAKPOINT_LEN_7: c_uint = 0x7f;
pub const ARM_BREAKPOINT_LEN_8: c_uint = 0xff;
// Kernel stepping
pub const ARM_KERNEL_STEP_NONE: c_int = 0;
pub const ARM_KERNEL_STEP_ACTIVE: c_int = 1;
pub const ARM_KERNEL_STEP_SUSPEND: c_int = 2;
//
// Limits.
// Changing these will require modifications to the register accessors.
//
pub const ARM_MAX_BRP: c_int = 16;
pub const ARM_MAX_WRP: c_int = 16;
// Virtual debug register bases.
pub const AARCH64_DBG_REG_BVR: c_int = 0;

// Debug register names.

// Accessor macros for the debug registers.

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
    pub fn hw_breakpoint_pmu_read(bp: *mut perf_event);
}
extern "C" {
    pub fn hw_breakpoint_slots(type: c_int) -> c_int;
}

extern "C" {
    pub fn hw_breakpoint_thread_switch(next: *mut task_struct);
}
extern "C" {
    pub fn ptrace_hw_copy_thread(task: *mut task_struct);
}

// Determine number of BRP registers available.
// Determine number of WRP registers available.

extern "C" {
    pub fn cpu_suspend_set_dbg_restorer(int): *mut *mut int (hw_bp_restore)(unsigned);
}

