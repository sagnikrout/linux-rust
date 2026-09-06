//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/exception.h
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

extern "C" {
    pub fn show_registers(regs: *mut pt_regs);
}
extern "C" {
    pub fn cache_parity_error() -> asmlinkage void;
}
extern "C" {
    pub fn do_ade(regs: *mut pt_regs) -> asmlinkage void noinstr;
}
extern "C" {
    pub fn do_ale(regs: *mut pt_regs) -> asmlinkage void noinstr;
}
extern "C" {
    pub fn do_bce(regs: *mut pt_regs) -> asmlinkage void noinstr;
}
extern "C" {
    pub fn do_bp(regs: *mut pt_regs) -> asmlinkage void noinstr;
}
extern "C" {
    pub fn do_ri(regs: *mut pt_regs) -> asmlinkage void noinstr;
}
extern "C" {
    pub fn do_fpu(regs: *mut pt_regs) -> asmlinkage void noinstr;
}
extern "C" {
    pub fn do_fpe(regs: *mut pt_regs, fcsr: c_ulong) -> asmlinkage void noinstr;
}
extern "C" {
    pub fn do_lsx(regs: *mut pt_regs) -> asmlinkage void noinstr;
}
extern "C" {
    pub fn do_lasx(regs: *mut pt_regs) -> asmlinkage void noinstr;
}
extern "C" {
    pub fn do_lbt(regs: *mut pt_regs) -> asmlinkage void noinstr;
}
extern "C" {
    pub fn do_watch(regs: *mut pt_regs) -> asmlinkage void noinstr;
}
extern "C" {
    pub fn do_syscall(regs: *mut pt_regs) -> asmlinkage void noinstr;
}
extern "C" {
    pub fn do_reserved(regs: *mut pt_regs) -> asmlinkage void noinstr;
}
extern "C" {
    pub fn do_vint(regs: *mut pt_regs, sp: c_ulong) -> asmlinkage void noinstr;
}
extern "C" {
    pub fn handle_ade() -> asmlinkage void;
}
extern "C" {
    pub fn handle_ale() -> asmlinkage void;
}
extern "C" {
    pub fn handle_bce() -> asmlinkage void;
}
extern "C" {
    pub fn handle_sys() -> asmlinkage void;
}
extern "C" {
    pub fn handle_bp() -> asmlinkage void;
}
extern "C" {
    pub fn handle_ri() -> asmlinkage void;
}
extern "C" {
    pub fn handle_fpu() -> asmlinkage void;
}
extern "C" {
    pub fn handle_fpe() -> asmlinkage void;
}
extern "C" {
    pub fn handle_lsx() -> asmlinkage void;
}
extern "C" {
    pub fn handle_lasx() -> asmlinkage void;
}
extern "C" {
    pub fn handle_lbt() -> asmlinkage void;
}
extern "C" {
    pub fn handle_watch() -> asmlinkage void;
}
extern "C" {
    pub fn handle_reserved() -> asmlinkage void;
}
extern "C" {
    pub fn handle_vint() -> asmlinkage void;
}
extern "C" {
    pub fn handle_loongarch_irq(regs: *mut pt_regs) -> asmlinkage void noinstr;
}
