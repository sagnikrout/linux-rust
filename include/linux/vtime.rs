//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/vtime.h
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
// Common vtime APIs
//

extern "C" {
    pub fn vtime_account_kernel(tsk: *mut task_struct);
}

extern "C" {
    pub fn vtime_user_enter(tsk: *mut task_struct);
}
extern "C" {
    pub fn vtime_user_exit(tsk: *mut task_struct);
}
extern "C" {
    pub fn vtime_guest_enter(tsk: *mut task_struct);
}
extern "C" {
    pub fn vtime_guest_exit(tsk: *mut task_struct);
}
extern "C" {
    pub fn vtime_init_idle(tsk: *mut task_struct, cpu: c_int);
}

extern "C" {
    pub fn context_tracking_enabled_cpu(_arg: cpu) -> return;
}
extern "C" {
    pub fn context_tracking_enabled_this_cpu() -> return;
}

extern "C" {
    pub fn vtime_account_idle(tsk: *mut task_struct);
}
extern "C" {
    pub fn vtime_account_irq(tsk: *mut task_struct, offset: c_uint);
}
extern "C" {
    pub fn vtime_account_softirq(tsk: *mut task_struct);
}
extern "C" {
    pub fn vtime_account_hardirq(tsk: *mut task_struct);
}
extern "C" {
    pub fn vtime_flush(tsk: *mut task_struct);
}

extern "C" {
    pub fn vtime_reset();
}
extern "C" {
    pub fn vtime_dyntick_start();
}
extern "C" {
    pub fn vtime_dyntick_stop();
}

//
// vtime_accounting_enabled_this_cpu() definitions/declarations
//

extern "C" {
    pub fn vtime_task_switch(prev: *mut task_struct);
}

//
// Checks if vtime is enabled on some CPU. Cputime readers want to be careful
// in that case and compute the tickless cputime.
// For now vtime state is tied to context tracking. We might want to decouple
// those later if necessary.
//
extern "C" {
    pub fn context_tracking_enabled() -> return;
}
extern "C" {
    pub fn vtime_generic_enabled_cpu(_arg: cpu) -> return;
}
extern "C" {
    pub fn vtime_generic_enabled_this_cpu() -> return;
}
extern "C" {
    pub fn vtime_task_switch_generic(prev: *mut task_struct);
}

extern "C" {
    pub fn irqtime_account_irq(tsk: *mut task_struct, offset: c_uint);
}

