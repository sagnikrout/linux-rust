//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/entry-common.h
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
// We are already called with irq disabled, so go without
// keeping track of riscv_v_flags.
//

extern "C" {
    pub fn handle_page_fault(regs: *mut pt_regs);
}
extern "C" {
    pub fn handle_break(regs: *mut pt_regs);
}

extern "C" {
    pub fn handle_misaligned_load(regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn handle_misaligned_store(regs: *mut pt_regs) -> c_int;
}

extern "C" {
    pub fn handle_user_cfi_violation(regs: *mut pt_regs) -> bool;
}
