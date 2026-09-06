//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/debug-monitors.h
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

// Low-level stepping controls.

// AArch64
pub const DBG_ESR_EVT_HWBP: c_uint = 0x0;
pub const DBG_ESR_EVT_HWSS: c_uint = 0x1;
pub const DBG_ESR_EVT_HWWP: c_uint = 0x2;
pub const DBG_ESR_EVT_BRK: c_uint = 0x6;
//
// Break point instruction encoding
//

pub const CACHE_FLUSH_IS_SAFE: c_int = 1;
// kprobes BRK opcodes with ESR encoding

// uprobes BRK opcodes with ESR encoding

// AArch32
pub const DBG_ESR_EVT_BKPT: c_uint = 0x4;
pub const DBG_ESR_EVT_VECC: c_uint = 0x5;
pub const AARCH32_BREAK_ARM: c_uint = 0x07f001f0;
pub const AARCH32_BREAK_THUMB: c_uint = 0xde01;
pub const AARCH32_BREAK_THUMB2_LO: c_uint = 0xf7f0;
pub const AARCH32_BREAK_THUMB2_HI: c_uint = 0xa000;

pub const DBG_HOOK_HANDLED: c_int = 0;
pub const DBG_HOOK_ERROR: c_int = 1;
extern "C" {
    pub fn debug_monitors_arch() -> u8;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dbg_active_el {
    DBG_ACTIVE_EL0 = 0,
    DBG_ACTIVE_EL1,
}

extern "C" {
    pub fn enable_debug_monitors(el: dbg_active_el);
}
extern "C" {
    pub fn disable_debug_monitors(el: dbg_active_el);
}
extern "C" {
    pub fn user_rewind_single_step(task: *mut task_struct);
}
extern "C" {
    pub fn user_fastforward_single_step(task: *mut task_struct);
}
extern "C" {
    pub fn kernel_enable_single_step(regs: *mut pt_regs);
}
extern "C" {
    pub fn kernel_disable_single_step();
}
extern "C" {
    pub fn kernel_active_single_step() -> c_int;
}
extern "C" {
    pub fn kernel_rewind_single_step(regs: *mut pt_regs);
}
extern "C" {
    pub fn kernel_fastforward_single_step(regs: *mut pt_regs);
}

extern "C" {
    pub fn try_step_suspended_breakpoints(regs: *mut pt_regs) -> bool;
}

extern "C" {
    pub fn try_handle_aarch32_break(regs: *mut pt_regs) -> bool;
}

