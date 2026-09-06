//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/suspend.h
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
pub const NR_CTX_REGS: c_int = 14;
pub const NR_CALLEE_SAVED_REGS: c_int = 12;
//
// struct cpu_suspend_ctx must be 16-byte aligned since it is allocated on
// the stack, which must be 16-byte aligned on v8
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_suspend_ctx {
//
// This struct must be kept in sync with
// cpu_do_{suspend/resume} in mm/proc.S
//
    pub ctx_regs: [u64; NR_CTX_REGS],
    pub sp: u64,
    pub __aligned(16): },
//
// Memory to save the cpu state is allocated on the stack by
// __cpu_suspend_enter()'s caller, and populated by __cpu_suspend_enter().
// This data must survive until cpu_resume() is called.
//
// This struct describes the size and the layout of the saved cpu state.
// The layout of the callee_saved_regs is defined by the implementation
// of __cpu_suspend_enter(), and cpu_resume(). This struct must be passed
// in by the caller as __cpu_suspend_enter()'s stack-frame is gone once it
// returns, and the data would be subsequently corrupted by the call to the
// finisher.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sleep_stack_data {
    pub system_regs: cpu_suspend_ctx,
    pub callee_saved_regs: [c_ulong; NR_CALLEE_SAVED_REGS],
}

extern "C" {
    pub fn cpu_suspend(arg: c_ulong, long): *mut *mut int (fn)(unsigned) -> c_int;
}
extern "C" {
    pub fn cpu_resume();
}
extern "C" {
    pub fn __cpu_suspend_enter(state: *mut sleep_stack_data) -> c_int;
}
extern "C" {
    pub fn __cpu_suspend_exit();
}
extern "C" {
    pub fn _cpu_resume();
}
extern "C" {
    pub fn swsusp_arch_suspend() -> c_int;
}
extern "C" {
    pub fn swsusp_arch_resume() -> c_int;
}
extern "C" {
    pub fn arch_hibernation_header_save(addr: *mut c_void, max_size: c_uint) -> c_int;
}
extern "C" {
    pub fn arch_hibernation_header_restore(addr: *mut c_void) -> c_int;
}
// Used to resume on the CPU we hibernated on
extern "C" {
    pub fn hibernate_resume_nonboot_cpu_disable() -> c_int;
}
