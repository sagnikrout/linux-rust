//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/debug.h
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
// Copyright (C) 1999 Cort Dougan <cort@cs.nmt.edu>
//

extern "C" {
    pub fn int(regs: *mut *mut __debugger)(struct pt_regs) -> extern;
}
extern "C" {
    pub fn int(regs: *mut *mut __debugger_ipi)(struct pt_regs) -> extern;
}
extern "C" {
    pub fn int(regs: *mut *mut __debugger_bpt)(struct pt_regs) -> extern;
}
extern "C" {
    pub fn int(regs: *mut *mut __debugger_sstep)(struct pt_regs) -> extern;
}
extern "C" {
    pub fn int(regs: *mut *mut __debugger_iabr_match)(struct pt_regs) -> extern;
}
extern "C" {
    pub fn int(regs: *mut *mut __debugger_break_match)(struct pt_regs) -> extern;
}
extern "C" {
    pub fn int(regs: *mut *mut __debugger_fault_handler)(struct pt_regs) -> extern;
}

extern "C" {
    pub fn __set_breakpoint(nr: c_int, brk: *mut arch_hw_breakpoint);
}
extern "C" {
    pub fn suspend_breakpoints();
}
extern "C" {
    pub fn restore_breakpoints();
}
extern "C" {
    pub fn ppc_breakpoint_available() -> bool;
}

