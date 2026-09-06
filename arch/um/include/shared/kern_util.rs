//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/include/shared/kern_util.h
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
// Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

extern "C" {
    pub fn alloc_stack(order: c_int, atomic: c_int) -> c_ulong;
}
extern "C" {
    pub fn free_stack(stack: c_ulong, order: c_int);
}
extern "C" {
    pub fn do_signal(regs: *mut pt_regs);
}
extern "C" {
    pub fn interrupt_end();
}
extern "C" {
    pub fn do_IRQ(irq: c_int, regs: *mut uml_pt_regs) -> c_uint;
}
extern "C" {
    pub fn initial_thread_cb(): *mut *mut void (proc)(void, arg: *mut c_void);
}
extern "C" {
    pub fn timer_handler(sig: c_int, unused_si: *mut siginfo, regs: *mut uml_pt_regs);
}
extern "C" {
    pub fn uml_pm_wake();
}
extern "C" {
    pub fn start_uml() -> c_int;
}
extern "C" {
    pub fn uml_cleanup();
}
extern "C" {
    pub fn do_uml_exitcalls();
}
//
// Are we disallowed to sleep? Used to choose between GFP_KERNEL and
// GFP_ATOMIC.
//
extern "C" {
    pub fn __uml_cant_sleep() -> c_int;
}
extern "C" {
    pub fn get_current_pid() -> c_int;
}
extern "C" {
    pub fn copy_from_user_proc(to: *mut c_void, from: *mut c_void, size: c_int) -> c_int;
}
extern "C" {
    pub fn uml_need_resched() -> c_int;
}
extern "C" {
    pub fn to_irq_stack(mask_out: *mut c_ulong) -> c_ulong;
}
extern "C" {
    pub fn from_irq_stack(nested: c_int) -> c_ulong;
}
extern "C" {
    pub fn singlestepping() -> c_int;
}
extern "C" {
    pub fn fatal_sigsegv(((noreturn): void) __attribute__);
}
extern "C" {
    pub fn um_idle_sleep();
}
extern "C" {
    pub fn kasan_map_memory(start: *mut c_void, len: usize);
}
