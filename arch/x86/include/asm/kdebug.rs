//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/kdebug.h
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

// Grossly misnamed.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum die_val {
    DIE_OOPS = 1,
    DIE_INT3,
    DIE_DEBUG,
    DIE_PANIC,
    DIE_NMI,
    DIE_DIE,
    DIE_KERNELDEBUG,
    DIE_TRAP,
    DIE_GPF,
    DIE_CALL,
    DIE_PAGE_FAULT,
    DIE_NMIUNKNOWN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum show_regs_mode {
    SHOW_REGS_SHORT,
//
// For when userspace crashed, but we don't think it's our fault, and
// therefore don't print kernel registers.
//
    SHOW_REGS_USER,
    SHOW_REGS_ALL
}

extern "C" {
    pub fn die(: *const c_char, : *mut pt_regs, _arg: c_long);
}
extern "C" {
    pub fn die_addr(str: *const c_char, regs: *mut pt_regs, err: c_long, gp_addr: c_long);
}
extern "C" {
    pub fn __die(: *const c_char, : *mut pt_regs, _arg: c_long) -> int __must_check;
}
extern "C" {
    pub fn show_stack_regs(regs: *mut pt_regs);
}
extern "C" {
    pub fn show_iret_regs(regs: *mut pt_regs, log_lvl: *const c_char);
}
extern "C" {
    pub fn oops_begin() -> c_ulong;
}
extern "C" {
    pub fn oops_end(long: unsigned, : *mut pt_regs, signr: c_int);
}
