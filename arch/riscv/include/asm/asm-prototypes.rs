//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/asm-prototypes.h
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
    pub fn __lshrdi3(a: c_longlong, b: c_int) -> c_longlong;
}
extern "C" {
    pub fn __ashrdi3(a: c_longlong, b: c_int) -> c_longlong;
}
extern "C" {
    pub fn __ashldi3(a: c_longlong, b: c_int) -> c_longlong;
}
extern "C" {
    pub fn __lshrti3(a: c_longlong, b: c_int) -> c_longlong;
}
extern "C" {
    pub fn __ashrti3(a: c_longlong, b: c_int) -> c_longlong;
}
extern "C" {
    pub fn __ashlti3(a: c_longlong, b: c_int) -> c_longlong;
}

extern "C" {
    pub fn enter_vector_usercopy(dst: *mut c_void, src: *mut c_void, n: usize, enable_sum: bool) -> asmlinkage int;
}

extern "C" {
    pub fn riscv_v_context_nesting_start(regs: *mut pt_regs) -> asmlinkage void;
}
extern "C" {
    pub fn riscv_v_context_nesting_end(regs: *mut pt_regs) -> asmlinkage void;
}

extern "C" {
    pub fn ret_from_fork_kernel(fn_arg: *mut c_void, ): *mut *mut int (fn)(void, regs: *mut pt_regs) -> asmlinkage void;
}
extern "C" {
    pub fn ret_from_fork_user(regs: *mut pt_regs) -> asmlinkage void;
}
extern "C" {
    pub fn handle_bad_stack(regs: *mut pt_regs) -> asmlinkage void;
}
extern "C" {
    pub fn do_page_fault(regs: *mut pt_regs) -> asmlinkage void;
}
extern "C" {
    pub fn do_irq(regs: *mut pt_regs) -> asmlinkage void;
}
