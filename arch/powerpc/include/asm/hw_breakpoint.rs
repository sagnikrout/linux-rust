//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/hw_breakpoint.h
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
// PowerPC BookIII S hardware breakpoint definitions
//
// Copyright 2010, IBM Corporation.
// Author: K.Prasad <prasad@linux.vnet.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_hw_breakpoint {
    pub address: c_ulong,
    pub type: u16,
    pub /: *mut *mut u16 len; / length of the target data symbol,
    pub /: *mut *mut u16 hw_len; / length programmed in hw,
    pub flags: u8,
    pub /: *mut *mut bool perf_single_step; / temporarily uninstalled for a perf single step,
}

// Note: Don't change the first 6 bits below as they are in the same order
// as the dabr and dabrx.
//
pub const HW_BRK_TYPE_READ: c_uint = 0x01;
pub const HW_BRK_TYPE_WRITE: c_uint = 0x02;
pub const HW_BRK_TYPE_TRANSLATE: c_uint = 0x04;
pub const HW_BRK_TYPE_USER: c_uint = 0x08;
pub const HW_BRK_TYPE_KERNEL: c_uint = 0x10;
pub const HW_BRK_TYPE_HYP: c_uint = 0x20;
pub const HW_BRK_TYPE_EXTRANEOUS_IRQ: c_uint = 0x80;
// bits that overlap with the bottom 3 bits of the dabr

pub const HW_BRK_FLAG_DISABLED: c_uint = 0x1;
// Minimum granularity

pub const HW_BREAKPOINT_SIZE: c_uint = 0x4;

pub const HW_BREAKPOINT_SIZE: c_uint = 0x8;

pub const HW_BREAKPOINT_SIZE_QUADWORD: c_uint = 0x10;
pub const DABR_MAX_LEN: c_int = 8;
pub const DAWR_MAX_LEN: c_int = 512;

extern "C" {
    pub fn hw_breakpoint_slots(type: c_int) -> c_int;
}
extern "C" {
    pub fn arch_bp_generic_fields(type: c_int, gen_bp_type: *mut c_int) -> c_int;
}
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
    pub fn flush_ptrace_hw_breakpoint(tsk: *mut task_struct);
}
extern "C" {
    pub fn thread_change_pc(tsk: *mut task_struct, regs: *mut pt_regs);
}
extern "C" {
    pub fn hw_breakpoint_handler(args: *mut die_args) -> c_int;
}

extern "C" {
    pub fn set_dawr(nr: c_int, brk: *mut arch_hw_breakpoint) -> c_int;
}

